use crate::error::MeshExtractorError;
use crate::volume::Volume;
use crate::mesh::Mesh;
use crate::math::Vec3;
use crate::tables::{EDGE_TABLE, EDGE_VERTICES, VERTEX_OFFSET, TRI_TABLE};

pub struct MeshExtractor {
    iso_value: f32,
}

impl MeshExtractor {
    pub fn new(iso_value: f32) -> Result<Self, MeshExtractorError> {
        if !iso_value.is_finite() {
            return Err(MeshExtractorError::NonFiniteIsovalue(iso_value));
        }
        Ok(MeshExtractor {iso_value})
    }

    pub fn extract(&self, volume: &Volume) -> Mesh {
        let mut mesh = Mesh::new();

        if volume.width < 2 || volume.height < 2 || volume.depth < 2 {
            return mesh;
        }

        let y_stride = volume.width;
        let z_stride = volume.width * volume.height;

        for z in 0..volume.depth - 1 {
            let z_offset = z * z_stride;
            for y in 0..volume.height - 1 {
                let y_offset = z_offset + y * y_stride;
                for x in 0..volume.width - 1 {
                    let coordinates = Vec3::new(x as f32, y as f32, z as f32);

                    let i0 = x + y_offset;
                    let i1 = i0 + 1;
                    let i2 = i0 + 1 + y_stride;
                    let i3 = i0 + y_stride;
                    let i4 = i0 + z_stride;
                    let i5 = i1 + z_stride;
                    let i6 = i2 + z_stride;
                    let i7 = i3 + z_stride;
                    let corner_index = [i0, i1, i2, i3, i4, i5, i6, i7];

                    let mut cube_index = 0;
                    for (v, idx) in corner_index.iter().enumerate() {
                        if volume.voxels[*idx] <= self.iso_value {
                            cube_index |= 1 << v;
                        }
                    }

                    let intersected_edges = EDGE_TABLE[cube_index];
                    if intersected_edges == 0 {
                        continue;
                    }

                    let mut edge_points = [None; 12];
                    for e in 0..12 {
                        if (intersected_edges & (1 << e)) != 0 {
                            let va = EDGE_VERTICES[e].0;
                            let vb = EDGE_VERTICES[e].1;
                            let voa = VERTEX_OFFSET[va];
                            let vob = VERTEX_OFFSET[vb];
                            let factor = (self.iso_value - volume.voxels[corner_index[va]])
                                / (volume.voxels[corner_index[vb]]
                                    - volume.voxels[corner_index[va]]);
                            let pos_a = coordinates + voa;
                            let pos_b = coordinates + vob;
                            edge_points[e] = Some(pos_a.lerp(pos_b, factor));
                        }
                    }

                    let triangles = TRI_TABLE[cube_index];
                    for i in triangles {
                        if i == -1 {
                            break;
                        }
                        mesh.vertices.push(edge_points[i as usize].expect("Corrupt TRI_TABLE being used"));
                        mesh.indices.push(mesh.vertices.len() - 1);
                    }
                }
            }
        }

        mesh
    }
}