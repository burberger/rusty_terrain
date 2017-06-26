#[macro_use(s)]
extern crate ndarray;
extern crate rand;
extern crate image;
extern crate kiss3d;
extern crate nalgebra as na;

use std::fs::File;
use std::path::Path;

use rand::distributions::{Normal, IndependentSample};
use image::ImageBuffer;
use na::{Point3, Vector3};
use kiss3d::window::Window;
use kiss3d::light::Light;

mod grid;

fn draw_to_image(grid: &grid::Grid) {
    let dim = grid.dim().0 as u32;
    let img = ImageBuffer::from_fn(dim, dim, |x, y| {
        let value = grid[[x as usize, y as usize]];
        if value >= 0.0 {
            image::Rgb([(value * 255.0) as u8, 0u8, 0u8])
        } else {
            image::Rgb([0u8, 0u8, (-value * 255.0) as u8])
        }
    });
    let ref mut fout = File::create(&Path::new("test.png")).unwrap();
    let _ = image::ImageRgb8(img).save(fout, image::PNG);
}

fn draw_3d(grid: &grid::Grid, size: f32) {
    let dim = grid.dim().0;
    let mut window = Window::new("3d terrain");
    window.set_light(Light::StickToCamera);

    let mut surface = window.add_quad(size, size, dim, dim);
    surface.set_color(1.0, 0.0, 0.0);
    let zero = Point3::new(0.0, 0.0, 0.0);
    let point = Point3::new(0.0, 0.0, -1.0);
    let up = Vector3::new(0.0, 0.0, 0.0);
    surface.reorient(&zero, &point, &up);

    while window.render() {}
}

fn main() {
    // Seed values for the 4 corners.
    let corner_values: [f64; 4] = [0.05, 0.1, 0.2, 0.07];
    // Construct an initialized matrix for the generated terrain.
    let mut grid = grid::build_grid(5, &corner_values);

    let mut rng = rand::thread_rng();
    let normal = Normal::new(1.0, 6.0);
    let ref mut get_sample = || normal.ind_sample(&mut rng);

    let ref mut grid_view = grid.view_mut();
    grid::diamond_square(grid_view, get_sample);

    let mut img_grid = grid::copy_grid(grid_view);
    grid::scale_dynamic_range(&mut img_grid.view_mut(), -1, 1);
    draw_to_image(&img_grid);

    draw_3d(&img_grid, 50.0);
}
