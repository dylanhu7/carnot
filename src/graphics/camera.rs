use glam::Mat4;

/// A perspective camera with intrinsics and extrinsics.
pub struct PerspectiveCamera {
    /// The position of the camera.
    eye: glam::Vec3,
    /// The point the camera is looking at.
    target: glam::Vec3,
    /// The direction that is up for the camera.
    up: glam::Vec3,
    /// The aspect ratio of the camera (width / height).
    aspect_ratio: f32,
    /// The field of view of the camera in the y direction in radians.
    fov: f32,
    /// The near clipping plane of the camera.
    near: f32,
    /// The far clipping plane of the camera.
    far: f32,
    /// The most recently calculated projection matrix of the camera.
    pub projection_matrix: Mat4,
    /// The most recently calculated view matrix of the camera.
    pub view_matrix: Mat4,
}

impl PerspectiveCamera {
    /// Creates a new perspective camera.
    /// # Arguments
    /// * `eye` - The position of the camera.
    /// * `target` - The point the camera is looking at.
    /// * `up` - The direction that is up for the camera.
    /// * `aspect_ratio` - The aspect ratio of the camera (width / height).
    /// * `fov` - The field of view of the camera in the y direction in degrees.
    /// * `near` - The near clipping plane of the camera.
    /// * `far` - The far clipping plane of the camera.
    pub fn new(
        eye: glam::Vec3,
        target: glam::Vec3,
        up: glam::Vec3,
        aspect_ratio: f32,
        fov: f32,
        near: f32,
        far: f32,
    ) -> Self {
        Self {
            eye,
            target,
            up,
            aspect_ratio,
            fov: fov.to_radians(),
            near,
            far,
            projection_matrix: Self::compute_projection_matrix(fov, aspect_ratio, near, far),
            view_matrix: Self::compute_view_matrix(eye, target, up),
        }
    }

    fn compute_projection_matrix(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Mat4 {
        Mat4::perspective_rh(fov, aspect_ratio, near, far)
    }

    fn compute_view_matrix(eye: glam::Vec3, target: glam::Vec3, up: glam::Vec3) -> Mat4 {
        Mat4::look_at_rh(eye, target, up)
    }

    pub fn update_intrinsics(&mut self, aspect_ratio: f32, fov: f32, near: f32, far: f32) {
        self.aspect_ratio = aspect_ratio;
        self.fov = fov;
        self.near = near;
        self.far = far;
        self.projection_matrix = Self::compute_projection_matrix(fov, aspect_ratio, near, far);
    }

    pub fn update_extrinsics(&mut self, eye: glam::Vec3, target: glam::Vec3, up: glam::Vec3) {
        self.eye = eye;
        self.target = target;
        self.up = up;
        self.view_matrix = Self::compute_view_matrix(eye, target, up);
    }
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_view_proj(&mut self, view: &Mat4, proj: &Mat4) {
        self.view_proj = (*proj * *view).to_cols_array_2d();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        Self {
            view_proj: glam::Mat4::IDENTITY.to_cols_array_2d(),
        }
    }
}
