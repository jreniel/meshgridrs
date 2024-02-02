# meshgridrs

Currently, ndarray [doesn't have a built-in meshgrid method](https://github.com/rust-ndarray/ndarray/discussions/1231).

There exists a [meshgrid](https://crates.io/crates/meshgrid) crate, but it only works with i32 types.

This is a meshgrid function that should work over generic types and in n-dimensions.

My hope is that eventually ndarray incorporates this, or it's own particular version.

```Rust
use ndarray::Array;
use meshgridrs::{meshgrid, Indexing};

fn main() {
    // Example with 3D.
    let x = Array::linspace(0.0, 1.0, 3);
    let y = Array::linspace(0.0, 1.0, 2);
    let z = Array::linspace(0.0, 1.0, 2);
    let xi = vec![x, y, z];
    let grids = meshgrid(&xi, Indexing::Xy).unwrap();
    for (i, grid) in grids.iter().enumerate() {
        println!("Grid {}:\n{:?}", i, grid);
    };
}
```

`SPDX-License-Identifier: LicenseRef-MIT`
