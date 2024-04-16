use crate::graphics::mesh::{Mesh, MeshVertex};

pub enum Primitive {
    CUBE,
    SPHERE,
    PLANE,
}

impl Primitive {
    /// A cube centered at the origin with a side length of 1 and 8 vertices.
    const CUBE_VERTICES: [MeshVertex; 8] = [
        MeshVertex {
            position: [-0.5, -0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [0.5, -0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [0.5, 0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [-0.5, 0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 1.0],
        },
        MeshVertex {
            position: [-0.5, -0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [0.5, -0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [0.5, 0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [-0.5, 0.5, 0.5],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 1.0],
        },
    ];

    #[rustfmt::skip]
    pub const CUBE_INDICES: [u32; 36] = [
        0, 1, 2,
        0, 2, 3, // front
        1, 5, 6,
        1, 6, 2, // right
        5, 4, 7,
        5, 7, 6, // back
        4, 0, 3,
        4, 3, 7, // left
        3, 2, 6,
        3, 6, 7, // top
        4, 5, 1,
        4, 1, 0, // bottom
    ];

    pub fn spawn(primitive: Primitive) -> Mesh {
        match primitive {
            Primitive::CUBE => Mesh {
                vertices: Self::CUBE_VERTICES.to_vec(),
                indices: Self::CUBE_INDICES.to_vec(),
            },
            Primitive::SPHERE => todo!(),
            Primitive::PLANE => todo!(),
        }
    }
}
