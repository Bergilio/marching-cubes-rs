# marching-cubes-rs

A Rust implementation of the [marching cubes](https://en.wikipedia.org/wiki/Marching_cubes) algorithm, using lookup tables. It turns a 3D grid of `f32` samples into a triangle mesh of the surface at a chosen iso-value.
 
This is a learning project, written mostly by hand, and it exists to be used in another learning project ([radix-rs](https://github.com/)). It is not meant to compete with the mature isosurface crates, and it will stay that way.


## Features
 
- Simple to use and dependency free
- Input validation with proper error types 

## Planned
 
- Test suite
- Performance work, including optional parallel extraction with [rayon](https://github.com/rayon-rs/rayon)


## Usage
 
```rust
use marching_cubes::{MeshExtractor, Volume};
 
// voxels are laid out x first, then y, then z
let volume = Volume::new(width, height, depth, voxels)?;
let mesh = MeshExtractor::new(0.5)?.extract(&volume);
 
// mesh.vertices: positions in grid units
// mesh.indices:  triangle list, three indices per triangle
```
 
To use it from another local project without publishing it:
 
```toml
[dependencies]
marching-cubes-rs = { path = "../marching-cubes-rs" }
```