use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum VolumeError {
    DimensionOverflow,
    LengthMismatch { expected: usize, actual: usize },
    NonFiniteVoxel(usize),
}

impl fmt::Display for VolumeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VolumeError::DimensionOverflow => {
                write!(f, "Volume dimensions cause an arithmetic overflow")
            }
            VolumeError::LengthMismatch { expected, actual } => write!(
                f,
                "Voxel buffer length {} does not match expected {}",
                actual, expected
            ),
            VolumeError::NonFiniteVoxel(index) => {
                write!(f, "Infinite or NaN at index {} of provided voxel", index)
            }
        }
    }
}

impl std::error::Error for VolumeError {}

#[derive(Debug, PartialEq)]
pub enum MeshExtractorError {
    NonFiniteIsovalue(f32),
}

impl fmt::Display for MeshExtractorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MeshExtractorError::NonFiniteIsovalue(iso_value) => {
                write!(f, "Isovalue {} is infinite or NaN", iso_value)
            }
        }
    }
}
