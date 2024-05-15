use glam::Mat4;

use super::Transform;

pub struct CameraTransform(pub Mat4);

impl From<&Transform> for CameraTransform {
    fn from(transform: &Transform) -> Self {
        Self(transform.0)
    }
}

/// A perspective camera with only intrinsic parameters (extrinsics should be handled by something like [Transform](crate::graphics::Transform))
pub struct PerspectiveCamera {
    /// The field of view of the camera in the x direction in degrees.
    fov: f32,
    /// The aspect ratio of the camera (width / height).
    aspect_ratio: f32,
    /// The near clipping plane of the camera.
    near: f32,
    /// The far clipping plane of the camera.
    far: f32,
    /// The most recently calculated projection matrix of the camera.
    projection_matrix: Mat4,
}

impl PerspectiveCamera {
    /// Creates a new perspective camera.
    /// # Arguments
    /// * `fov_x` - The field of view of the camera in the x direction in degrees.
    /// * `aspect_ratio` - The aspect ratio of the camera (width / height).
    /// * `near` - The near clipping plane of the camera.
    /// * `far` - The far clipping plane of the camera.
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        let fov_y = Self::fov_x_deg_to_fov_y_rad(fov, aspect_ratio);
        Self {
            aspect_ratio,
            fov,
            near,
            far,
            projection_matrix: Mat4::perspective_rh(fov_y, aspect_ratio, near, far),
        }
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// Returns the horizontal field of view in degrees.
    pub fn get_fov(&self) -> f32 {
        self.fov
    }

    pub fn get_near(&self) -> f32 {
        self.near
    }

    pub fn get_far(&self) -> f32 {
        self.far
    }

    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    pub fn update_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.recompute_projection_matrix();
    }

    pub fn update_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.recompute_projection_matrix();
    }

    pub fn update_near(&mut self, near: f32) {
        self.near = near;
        self.recompute_projection_matrix();
    }

    pub fn update_far(&mut self, far: f32) {
        self.far = far;
        self.recompute_projection_matrix();
    }

    pub fn update(&mut self, fov: f32, aspect_ratio: f32, near: f32, far: f32) {
        self.fov = fov;
        self.aspect_ratio = aspect_ratio;
        self.near = near;
        self.far = far;
        self.recompute_projection_matrix();
    }

    fn recompute_projection_matrix(&mut self) {
        let fov_y = Self::fov_x_deg_to_fov_y_rad(self.fov, self.aspect_ratio);
        self.projection_matrix =
            Mat4::perspective_rh(fov_y, self.aspect_ratio, self.near, self.far);
    }

    fn fov_x_deg_to_fov_y_rad(fov_x_deg: f32, aspect_ratio: f32) -> f32 {
        let fov_x_rad = fov_x_deg.to_radians();
        2.0 * ((fov_x_rad / 2.0).tan() / aspect_ratio).atan()
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
