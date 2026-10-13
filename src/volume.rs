use crate::error::VolumeError;

pub struct Volume {
    pub(crate) width: usize,  // x axis
    pub(crate) height: usize, // y axis
    pub(crate) depth: usize,  // z axis

    pub(crate) voxels: Vec<f32>,
}

impl Volume {
    pub fn new(
        width: usize,
        height: usize,
        depth: usize,
        voxels: Vec<f32>,
    ) -> Result<Volume, VolumeError> {
        let expected_len = width
            .checked_mul(height)
            .ok_or(VolumeError::DimensionOverflow)?;
        let expected_len = expected_len
            .checked_mul(depth)
            .ok_or(VolumeError::DimensionOverflow)?;

        if expected_len != voxels.len() {
            return Err(VolumeError::LengthMissmatch {
                expected: expected_len,
                actual: voxels.len(),
            });
        }

        for (i, v) in voxels.iter().enumerate() {
            if !v.is_finite() {
                return Err(VolumeError::NonFiniteVoxel(i));
            }
        }

        Ok(Volume {
            width,
            height,
            depth,
            voxels,
        })
    }
}
