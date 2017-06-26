use ndarray::prelude::*;

pub type Grid = Array2<f64>;
pub type GridViewMut<'a> = ArrayViewMut2<'a, f64>;

#[allow(dead_code)]
pub fn pretty_print_grid(grid: &GridViewMut) {
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

fn corner_mut<'a>(index: usize, grid: &'a mut GridViewMut) -> &'a mut f64 {
    let bound = grid.dim().0 - 1;
    match index {
        0 => &mut grid[[0, 0]],
        1 => &mut grid[[0, bound]],
        2 => &mut grid[[bound, 0]],
        3 => &mut grid[[bound, bound]],
        _ => panic!("Invalid corner index")
    }
}

pub fn build_grid(n: u32, corner_values: &[f64; 4]) -> Array2<f64> {
    let dim = (2_u32.pow(n) + 1) as usize;
    let mut grid = Grid::zeros((dim, dim));
    // map the initial condition to the corners of the matrix in row major order.
    for i in 0..4 {
        let ref mut grid_view = grid.view_mut();
        *corner_mut(i, grid_view) = corner_values[i];
    }

    grid
}

pub fn copy_grid(grid: &GridViewMut) -> Grid {
    Grid::from_shape_fn(grid.dim(), |index| {
        grid[index]
    })
}

pub fn scale_dynamic_range(grid: &mut GridViewMut, min_val: i32, max_val: i32) {
    let mut max: f64 = 0.0;
    let mut min: f64 = 0.0;

    for elem in grid.iter() {
        let val = *elem;
        max = if val > max { val } else { max };
        min = if val < min { val } else { min };
    }

    max = max / (max_val as f64);
    min = min / (min_val as f64);
    for elem in grid.iter_mut() {
        if *elem >= 0.0 as f64 {
            *elem = *elem / max;
        } else {
            *elem = *elem / min;
        }
    }
}

pub fn diamond_square(grid: &mut GridViewMut, rng: &mut FnMut() -> f64) {
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
        /*
         * Subdivide the grid into 4 subgrids:
         * ---------------
         * |  TL  |  TR  | Edge bound
         * ---------------
         * |  BL  |  BR  |
         * ---------------
         *        ^
         *     Center bound
         *
         * As the grid is square, bounds are the same for each dimension. Each subgrid is
         * a slice of the main grid over some combination of bounds.
         */

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
