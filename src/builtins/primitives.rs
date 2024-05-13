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
            Primitive::SPHERE => tessellate_sphere(1.0, 64, 64),
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

/// Tessellates a sphere with the given radius, number of rings, and number of sectors.
/// The sphere is centered at the origin.
/// When `rings` is 1 and `sectors` is 1, the sphere will be a single tetrahedron.
fn tessellate_sphere(radius: f32, rings: usize, sectors: usize) -> Mesh {
    // panic if the number of rings or sectors is less than 1
    assert!(rings > 0);
    assert!(sectors > 0);

    let rings = rings + 2;
    let sectors = sectors + 2;

    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    let pi = std::f32::consts::PI;
    let two_pi = 2.0 * pi;

    for ring in 0..=rings {
        let phi = pi * ring as f32 / rings as f32; // 0 to PI
        let y = phi.cos();
        let r = phi.sin();

        for sector in 0..=sectors {
            let theta = two_pi * sector as f32 / sectors as f32; // 0 to 2PI
            let x = r * theta.sin();
            let z = r * theta.cos();

            let s = sector as f32 / sectors as f32;
            let t = ring as f32 / rings as f32;

            vertices.push(MeshVertex {
                position: [radius * x, radius * y, radius * z],
                normal: [x, y, z],
                tex_coords: [s, t],
            });
        }
    }

    for ring in 0..rings {
        for sector in 0..sectors {
            let lower = ring * (sectors + 1) + sector;
            let upper = lower + sectors + 1;

            // Each quad on the sphere is made up of two triangles
            indices.push(lower as u32);
            indices.push(upper as u32);
            indices.push((lower + 1) as u32);

            indices.push(upper as u32);
            indices.push((upper + 1) as u32);
            indices.push((lower + 1) as u32);
        }
    }

    Mesh { vertices, indices }
}
