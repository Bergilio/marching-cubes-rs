use crate::math::Vec3;

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<Vec3>,
    pub indices: Vec<usize>,
}

impl Mesh {
    pub(crate) fn new() -> Self {
        Mesh {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn to_obj(&self) -> String {
        let mut st = String::new();

        for v in &self.vertices {
            st.push_str(&format!("v {} {} {}", v.x, v.y, v.z));
        }

        for c in self.indices.chunks(3) {
            st.push_str(&format!("f {} {} {}", c[0] + 1, c[1] + 1, c[2] + 1));
        }

        st
    }
}