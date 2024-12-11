use meshgridrs::{meshgrid, Indexing};
use ndarray::Array;

fn main() {
    // Example with 3D.
    let x = Array::linspace(0.0, 1.0, 3);
    let y = Array::linspace(0.0, 1.0, 2);
    let z = Array::linspace(0.0, 1.0, 2);
    let xi = vec![x, y, z];
    let grids = meshgrid(&xi, Indexing::Xy).unwrap();
    for (i, grid) in grids.iter().enumerate() {
        println!("Grid {}:\n{:?}", i, grid);
    }
}
