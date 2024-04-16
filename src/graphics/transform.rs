pub struct Transform(pub glam::Mat4);

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
