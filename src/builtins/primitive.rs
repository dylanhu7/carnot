use crate::graphics::mesh::{Mesh, MeshVertex};

pub enum Primitive {
    TRIANGLE,
    PLANE,
    CUBE,
    SPHERE,
    CYLINDER,
    CONE,
}

impl Primitive {
    pub fn spawn(primitive: Primitive) -> Mesh {
        match primitive {
            Primitive::TRIANGLE => Mesh {
                vertices: Self::TRIANGLE_VERTICES.to_vec(),
                indices: Self::TRIANGLE_INDICES.to_vec(),
            },
            Primitive::PLANE => Mesh {
                vertices: Self::PLANE_VERTICES.to_vec(),
                indices: Self::PLANE_INDICES.to_vec(),
            },
            Primitive::CUBE => Mesh {
                vertices: Self::CUBE_VERTICES.to_vec(),
                indices: Self::CUBE_INDICES.to_vec(),
            },
            Primitive::SPHERE => todo!(),
            Primitive::CYLINDER => todo!(),
            Primitive::CONE => todo!(),
        }
    }

    const TRIANGLE_VERTICES: [MeshVertex; 3] = [
        MeshVertex {
            position: [0.0, 0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.5, 1.0],
        },
        MeshVertex {
            position: [-0.5, -0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [0.5, -0.5, 0.0],
            normal: [0.0, 0.0, 1.0],
            tex_coords: [1.0, 0.0],
        },
    ];

    const TRIANGLE_INDICES: [u32; 3] = [0, 1, 2];

    const PLANE_VERTICES: [MeshVertex; 4] = [
        MeshVertex {
            position: [-0.5, 0.0, 0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [0.5, 0.0, 0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [0.5, 0.0, -0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [-0.5, 0.0, -0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];

    const PLANE_INDICES: [u32; 6] = [0, 1, 2, 0, 2, 3];

    /// A cube centered at the origin with a side length of 1 and 8 vertices.
    const CUBE_VERTICES: [MeshVertex; 24] = [
        // front
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
        // right
        MeshVertex {
            position: [0.5, -0.5, 0.5],
            normal: [1.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [0.5, -0.5, -0.5],
            normal: [1.0, 0.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [0.5, 0.5, -0.5],
            normal: [1.0, 0.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [0.5, 0.5, 0.5],
            normal: [1.0, 0.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
        // back
        MeshVertex {
            position: [0.5, -0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [-0.5, -0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [-0.5, 0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [0.5, 0.5, -0.5],
            normal: [0.0, 0.0, -1.0],
            tex_coords: [0.0, 1.0],
        },
        // left
        MeshVertex {
            position: [-0.5, -0.5, -0.5],
            normal: [-1.0, 0.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [-0.5, -0.5, 0.5],
            normal: [-1.0, 0.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [-0.5, 0.5, 0.5],
            normal: [-1.0, 0.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [-0.5, 0.5, -0.5],
            normal: [-1.0, 0.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
        // top
        MeshVertex {
            position: [-0.5, 0.5, 0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [0.5, 0.5, 0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [0.5, 0.5, -0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [-0.5, 0.5, -0.5],
            normal: [0.0, 1.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
        // bottom
        MeshVertex {
            position: [-0.5, -0.5, -0.5],
            normal: [0.0, -1.0, 0.0],
            tex_coords: [0.0, 0.0],
        },
        MeshVertex {
            position: [0.5, -0.5, -0.5],
            normal: [0.0, -1.0, 0.0],
            tex_coords: [1.0, 0.0],
        },
        MeshVertex {
            position: [0.5, -0.5, 0.5],
            normal: [0.0, -1.0, 0.0],
            tex_coords: [1.0, 1.0],
        },
        MeshVertex {
            position: [-0.5, -0.5, 0.5],
            normal: [0.0, -1.0, 0.0],
            tex_coords: [0.0, 1.0],
        },
    ];

    const CUBE_INDICES: [u32; 36] = [
        0, 1, 2, 0, 2, 3, // front
        4, 5, 6, 4, 6, 7, // right
        8, 9, 10, 8, 10, 11, // back
        12, 13, 14, 12, 14, 15, // left
        16, 17, 18, 16, 18, 19, // top
        20, 21, 22, 20, 22, 23, // bottom
    ];
}
