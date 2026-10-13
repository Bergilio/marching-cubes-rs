#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<(f32, f32, f32)>,
    pub indices: Vec<usize>,
}

impl Mesh {
    pub(crate) fn new() -> Mesh {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }
}