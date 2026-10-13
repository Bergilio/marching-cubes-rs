use marching_cubes::{Volume, mesh_extractor::MeshExtractor};

fn main() {
    let voxels = Vec::from([1.0, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0, 0.0]);
    let volume = Volume::new(2, 2, 2, voxels).unwrap();
    let mesh_extractor = MeshExtractor::new(0.5).unwrap();

    let mesh = mesh_extractor.extract(&volume);

    println!("{:?}", mesh);
}
