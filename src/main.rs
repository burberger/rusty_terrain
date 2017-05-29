extern crate ndarray;
use ndarray::Array2;

fn build_grid(n: u32, corner_values: &[f64; 4]) -> Array2<f64> {
    let dim = (2_u32.pow(n) + 1) as usize;
    let mut grid = Array2::<f64>::zeros((dim, dim));
    // map the initial condition to the corners of the matrix in row major order.
    let bound = dim - 1;
    grid[[0, 0]] = corner_values[0];
    grid[[bound, 0]] = corner_values[1];
    grid[[0, bound]] = corner_values[2];
    grid[[bound, bound]] = corner_values[3];

    grid
}

//fn diamond_step() {
    
//}

fn main() {
    let corner_values: [f64; 4] = [0.0, 1.1, 2.2, 3.3];
    let grid = build_grid(2, &corner_values);
    println!("grid is");
    println!("{:?}", grid);
    println!("corners are {} {} {} {}", grid[[0, 0]], grid[[4, 0]], grid[[0, 4]], grid[[4, 4]]);
}
