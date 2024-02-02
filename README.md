# meshgridrs

Currently, ndarray [doesn't have a built-in meshgrid method](https://github.com/rust-ndarray/ndarray/discussions/1231).

There exists a [meshgrid](https://crates.io/crates/meshgrid) crate, but it only works with i32 types.

This is a meshgrid function that should work over generic types.

My hope is that eventually ndarray incorporates this, or it's own particular version.

`SPDX-License-Identifier: LicenseRef-MIT`
