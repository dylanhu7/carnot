pub struct Transform(pub glam::Mat4);

impl Default for Transform {
    fn default() -> Self {
        Self(glam::Mat4::IDENTITY)
    }
}
