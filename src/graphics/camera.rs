use glam::Mat4;

/// A perspective camera with only intrinsic parameters (extrinsics should be handled by something like [Transform](crate::graphics::Transform))
pub struct PerspectiveCamera {
    aspect_ratio: f32,
    /// The field of view of the camera in the y direction in radians.
    fov: f32,
    /// The near clipping plane of the camera.
    near: f32,
    /// The far clipping plane of the camera.
    far: f32,
    /// The most recently calculated projection matrix of the camera.
    pub projection_matrix: Mat4,
}

impl PerspectiveCamera {
    /// Creates a new perspective camera.
    /// # Arguments
    /// * `aspect_ratio` - The aspect ratio of the camera (width / height).
    /// * `fov` - The field of view of the camera in the y direction in degrees.
    /// * `near` - The near clipping plane of the camera.
    /// * `far` - The far clipping plane of the camera.
    pub fn new(aspect_ratio: f32, fov: f32, near: f32, far: f32) -> Self {
        Self {
            aspect_ratio,
            fov: fov.to_radians(),
            near,
            far,
            projection_matrix: Self::compute_projection_matrix(
                fov.to_radians(),
                aspect_ratio,
                near,
                far,
            ),
        }
    }

    fn compute_projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
        Mat4::perspective_rh(fov, aspect_ratio, near, far)
    }

    pub fn update(&mut self, aspect_ratio: f32, fov: f32, near: f32, far: f32) {
        self.aspect_ratio = aspect_ratio;
        self.fov = fov;
        self.near = near;
        self.far = far;
        self.projection_matrix = Self::compute_projection_matrix(fov, aspect_ratio, near, far);
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
    camera_pos: [f32; 4],
}

impl CameraUniform {
    pub fn from_view_proj(view: &Mat4, proj: &Mat4) -> Self {
        Self {
            view_proj: (*proj * *view).to_cols_array_2d(),
            camera_pos: view.w_axis.to_array(),
        }
    }

    pub fn from_inv_view_proj(inv_view: &Mat4, proj: &Mat4) -> Self {
        Self {
            view_proj: (*proj * inv_view.inverse()).to_cols_array_2d(),
            camera_pos: inv_view.w_axis.to_array(),
        }
    }
}
