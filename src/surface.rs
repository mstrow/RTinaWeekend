use crate::ray::*;
pub trait Surface{
    fn intersect(&self, ray: &Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool;
}