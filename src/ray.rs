use cgmath::{Vector3, Vector2};
use crate::material_type::MaterialType;
use crate::texture::Texture;

pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>
}
impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin: origin,
            direction: direction
        }
    }
    pub fn at(&self, t: f64) -> Vector3<f64> {
        return self.origin + t * self.direction
    }
}
#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub pos: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub material: MaterialType,
    pub uv: Vector2<f64>
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            t: 0.0,
            pos: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            material: MaterialType::Lambert(Texture::SolidColor(Vector3::new(0.0,0.0,0.0)), 0.0),
            uv: Vector2::new(0.0, 0.0)
        }
    }
}