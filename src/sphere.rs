use cgmath::*;
use crate::ray::*;
use crate::surface::Surface;
use crate::material_type::MaterialType;
use std::f64::consts::*;

#[derive(Clone)]
pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
    material: MaterialType
}
impl Sphere {
    pub fn new(center: Vector3<f64>, radius: f64, material: MaterialType) -> Sphere {
        Sphere { center, radius , material}
    }
    pub fn to_box(&self) -> Box<Sphere> {
        Box::new(self.clone())
    }
    fn get_sphere_uv(&self, p: Vector3<f64>) -> Vector2<f64> {
        let phi = p.x.atan2(p.z);
        let theta = p.y.asin();
        return Vector2::new((1.0 - (phi + PI) / (2.0 * PI)) * 0.5, (theta + PI / 2.0) / PI)
    }

}
impl Surface for Sphere {
    fn intersect(&self, ray: &Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.pos = ray.at(rec.t);
                rec.normal = (rec.pos - self.center) / self.radius;
                rec.material = self.material.clone();
                rec.uv = self.get_sphere_uv(rec.normal);
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.pos = ray.at(rec.t);
                rec.normal = (rec.pos - self.center) / self.radius;
                rec.material = self.material.clone();
                rec.uv = self.get_sphere_uv(rec.normal);
                return true;
            }
        }
        return false
    }

}