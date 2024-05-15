use super::{ray::Ray, Transform};

pub trait Implicit {
    fn intersect_world(&self, world_ray: &Ray, self_transform: &Transform) -> Option<f32>;
}

pub struct ImplicitSphere;

impl Implicit for ImplicitSphere {
    fn intersect_world(&self, world_ray: &Ray, self_transform: &Transform) -> Option<f32> {
        let local_ray = Ray {
            origin: self_transform
                .0
                .inverse()
                .transform_point3(world_ray.origin),
            direction: self_transform
                .0
                .inverse()
                .transform_vector3(world_ray.direction),
        };

        let a = local_ray.direction.length_squared();
        let b = 2.0 * local_ray.direction.dot(local_ray.origin);
        let c = local_ray.origin.length_squared() - 0.25; // radius squared is 0.25

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return None;
        }

        let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        let t = if t0 < 0.0 {
            t1
        } else if t1 < 0.0 {
            t0
        } else {
            t0.min(t1)
        };

        Some(t)
    }
}
