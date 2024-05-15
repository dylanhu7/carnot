use super::camera::CameraTransform;

pub struct Transform(pub glam::Mat4);

impl Transform {
    pub fn borrow_matrix(&self) -> &glam::Mat4 {
        &self.0
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self(glam::Mat4::IDENTITY)
    }
}

impl From<glam::Mat4> for Transform {
    fn from(mat: glam::Mat4) -> Self {
        Self(mat)
    }
}

impl From<Transform> for glam::Mat4 {
    fn from(transform: Transform) -> glam::Mat4 {
        transform.0
    }
}

impl From<&Transform> for glam::Mat4 {
    fn from(transform: &Transform) -> glam::Mat4 {
        transform.0
    }
}

impl From<&glam::Mat4> for Transform {
    fn from(mat: &glam::Mat4) -> Self {
        Self(*mat)
    }
}

impl From<&CameraTransform> for Transform {
    fn from(camera: &CameraTransform) -> Self {
        Self(camera.0)
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Mat4Uniform {
    model: [[f32; 4]; 4],
}

impl From<Transform> for Mat4Uniform {
    fn from(transform: Transform) -> Self {
        Self {
            model: transform.0.to_cols_array_2d(),
        }
    }
}

impl From<&Transform> for Mat4Uniform {
    fn from(transform: &Transform) -> Self {
        Self {
            model: transform.0.to_cols_array_2d(),
        }
    }
}
