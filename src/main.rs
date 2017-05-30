#[macro_use(s)]
extern crate ndarray;
extern crate rand;

use ndarray::prelude::*;
use rand::distributions::{Normal, IndependentSample};

#[allow(dead_code)]
fn pretty_print_grid(grid: &ArrayViewMut2<f64>) {
    for lane in grid.lanes(Axis(1)) {
        for val in lane {
            print!("| {:>7.4} ", val);
        }
        println!("|");
    }
}

fn corner(index: usize, grid: &ArrayViewMut2<f64>) -> f64 {
    let bound = grid.dim().0 - 1;
    match index {
        0 => grid[[0, 0]],
        1 => grid[[0, bound]],
        2 => grid[[bound, 0]],
        3 => grid[[bound, bound]],
        _ => panic!("Invalid corner index")
    }
}

fn corner_mut<'a>(index: usize, grid: &'a mut ArrayViewMut2<f64>) -> &'a mut f64 {
    let bound = grid.dim().0 - 1;
    match index {
        0 => &mut grid[[0, 0]],
        1 => &mut grid[[0, bound]],
        2 => &mut grid[[bound, 0]],
        3 => &mut grid[[bound, bound]],
        _ => panic!("Invalid corner index")
    }
}

fn build_grid(n: u32, corner_values: &[f64; 4]) -> Array2<f64> {
    let dim = (2_u32.pow(n) + 1) as usize;
    let mut grid = Array2::<f64>::zeros((dim, dim));
    // map the initial condition to the corners of the matrix in row major order.
    for i in 0..4 {
        let ref mut grid_view = grid.view_mut();
        *corner_mut(i, grid_view) = corner_values[i];
    }

    grid
}

fn diamond_square(grid: &mut ArrayViewMut2<f64>, rng: &mut FnMut() -> f64) {
    let bound = grid.dim().0 - 1;
    let center = bound / 2;
    let corner_avg = (0..4).map(|i| corner(i, grid)).fold(0.0, |sum, i| sum + i) / 4.0;

    // Diamond processing step.
    grid[[center, center]] = corner_avg + rng();

    // Square processing step.
    grid[[0, center]] = corner_avg + rng();
    grid[[center, 0]] = corner_avg + rng();
    grid[[center, bound]] = corner_avg + rng();
    grid[[bound, center]] = corner_avg + rng();

    // Recursively process the rest of the grid. With bound = 2, we're out of work to do.
    let center = center as isize;
    if bound > 2 {
        // Top left corner.
        diamond_square(&mut grid.slice_mut(s![..center+1, ..center+1]), rng);
        // Top right corner.
        diamond_square(&mut grid.slice_mut(s![center.., ..center+1]), rng);
        // Bottom left corner.
        diamond_square(&mut grid.slice_mut(s![..center+1, center..]), rng);
        // Bottom right corner.
        diamond_square(&mut grid.slice_mut(s![center.., center..]), rng);
    }
}

fn main() {
    // Seed values for the 4 corners.
    let corner_values: [f64; 4] = [0.5, 0.1, 0.2, 0.7];
    // Construct an initialized matrix for the generated terrain.
    let mut grid = build_grid(8, &corner_values);

    let mut rng = rand::thread_rng();
    let normal = Normal::new(1.0, 3.0);
    let ref mut get_sample = || normal.ind_sample(&mut rng);

    let ref mut grid_view = grid.view_mut();
    diamond_square(grid_view, get_sample);

    let shape = grid_view.dim().0;
       
}
