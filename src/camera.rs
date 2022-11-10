use cgmath::*;
use crate::ray::*;
use std::f64::consts::*;

pub struct Camera {
    pub origin: Vector3<f64>,
    pub lower_left_corner: Vector3<f64>,
    pub horizontal: Vector3<f64>,
    pub vertical: Vector3<f64>
}
impl Camera {
    pub fn new(lookfrom: Vector3<f64>, lookat: Vector3<f64>, vup: Vector3<f64>, vfov: f64, aspect: f64) -> Camera{
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);
        Camera {
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: lookfrom
        }
    }
    pub fn get_ray(&self, uv: Vector2<f64>) -> Ray {
        return Ray::new(self.origin, self.lower_left_corner + uv.x * self.horizontal + uv.y * self.vertical - self.origin)
    }

}