use crate::ray::*;
use crate::surface::Surface;
pub struct SurfaceList {
    pub surfaces: Vec<Box<dyn Surface>>
}
impl SurfaceList {
    pub fn new() -> SurfaceList {
        SurfaceList {
            surfaces: Vec::new(),
        }
    }
}
impl Surface for SurfaceList {
    fn intersect(&self, ray: &Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for surface in &self.surfaces {
            if surface.intersect(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t.clone();
                *rec = temp_rec.clone();
            }
        }
        return hit_anything
    }
}