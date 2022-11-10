use std::rc::Rc;
use bmp::{Image, Pixel, px};
use cgmath::*;
use crate::ray::*;
use crate::sphere::Sphere;
use crate::surface::Surface;
use crate::surface_list::SurfaceList;
use crate::camera::Camera;
use crate::material_type::*;
use rand::Rng;
use crate::num_traits::pow;
use crate::texture::Texture;

mod ray;
mod sphere;
mod surface;
mod surface_list;
mod camera;
mod material_type;
mod texture;

pub fn schlick(cosine: f64, ref_idx: f64) -> f64{
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0-r0)*pow(1.0-cosine, 5);
}

pub fn scatter(ray: &Ray, hit_record: &HitRecord, attenuation: &mut Vector3<f64>, scattered: &mut Ray) -> bool{
    match &hit_record.material {
        MaterialType::Lambert(tex, roughness) => {
            let target = hit_record.pos + hit_record.normal + *roughness * random_in_unit_sphere();
            *scattered = Ray::new(hit_record.pos, target - hit_record.pos);
            *attenuation = tex.color(&hit_record.uv, &hit_record.pos);
            return true;
        },
        MaterialType::Metal(tex, roughness) => {
            let reflected = reflect(ray.direction.normalize(), hit_record.normal);
            *scattered = Ray::new(hit_record.pos, reflected + *roughness * random_in_unit_sphere());
            *attenuation = tex.color(&hit_record.uv, &hit_record.pos);
            return scattered.direction.dot(hit_record.normal) > 0.0;
        }
        MaterialType::Dielectric(tex, ref_idx, roughness) => {
            let reflected = reflect(ray.direction.normalize(), hit_record.normal);
            let outward_normal;
            let mut ni_over_nt:f64;
            let cosine;

            if ray.direction.dot(hit_record.normal) > 0.0 {
                outward_normal = -hit_record.normal;
                ni_over_nt = *ref_idx;
                cosine = ref_idx * ray.direction.dot(hit_record.normal) / ray.direction.magnitude();
            } else {
                outward_normal = hit_record.normal;
                ni_over_nt = (1.0 / ref_idx);
                cosine = -ray.direction.dot(hit_record.normal) / ray.direction.magnitude();
            }
            let refracted = refract(&ray.direction, &outward_normal, &ni_over_nt);
            let reflect_prob;

            if !refracted.is_zero() {
                reflect_prob = schlick(cosine, *ref_idx);
            }
            else{
                reflect_prob = 1.0;
            }
            if rand::thread_rng().gen::<f64>() < reflect_prob {
                *scattered = Ray::new(hit_record.pos, reflected + *roughness * random_in_unit_sphere());
            } else {
                *scattered = Ray::new(hit_record.pos, refracted + *roughness * random_in_unit_sphere());
            }
            *attenuation = tex.color(&hit_record.uv, &hit_record.pos);
            return true;
        }
    }
}

fn random_in_unit_sphere() -> Vector3<f64> {
    let mut rng = rand::thread_rng();
    let mut p = Vector3::new(rng.gen(), rng.gen(), rng.gen());
    while p.magnitude2() >= 1.0 {
        p.x = rng.gen();
        p.y = rng.gen();
        p.z = rng.gen();
    }
    return p
}

fn reflect(v: Vector3<f64>, n: Vector3<f64>) -> Vector3<f64> {
    return v - 2.0 * v.dot(n) * n
}

fn refract(v: &Vector3<f64>, n: &Vector3<f64>, ni_over_nt: &f64) -> Vector3<f64> {
    let uv = v.normalize();
    let dt = uv.dot(*n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
    if discriminant > 0.0 {
        return *ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
    } else {
        return Vector3::new(0.0, 0.0, 0.0);
    }
}

fn ray_color(r: &Ray, world: &SurfaceList, recursion_depth: u32, darken: f64) -> Vector3<f64> {
    if recursion_depth <= 0 {
        return Vector3::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    if world.intersect(r, 0.001, f64::MAX, &mut rec) {
       let mut scattered = Ray::new(Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));
       let mut attenuation = Vector3::new(0.0, 0.0, 0.0);
       if scatter(r, &rec, &mut attenuation, &mut scattered) {
           let col = ray_color(&scattered, world, recursion_depth - 1, darken - 0.08);
           return Vector3::new(col.x * attenuation.x, col.y * attenuation.y, col.z * attenuation.z) * darken;
       }
        else{
            return Vector3::new(0.0, 0.0, 0.0);
        }
    }
    else{
        let unitdirection: Vector3<f64> = r.direction.normalize();
        let t: f64 = 0.5 * (unitdirection.y + 1.0);
        return (1.0-t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0);
    }
}


fn main() {
    let width:u32 = 1920;
    let height:u32 = 1080;
    let samples:u32 = 100;
    let recursion_depth:u32 = 20;
    let mut rng = rand::thread_rng();

    let mut img = Image::new(width, height);
    let earth = bmp::open("soccer.bmp").unwrap();
    let mut world = SurfaceList::new();
    world.surfaces.push(Sphere::new(Vector3::new(0.0, 0.0, -1.0),
                                             0.5,
                                             MaterialType::Dielectric(Texture::SolidColor(Vector3::new(1.0, 1.0, 0.6)),1.45, 0.01)
    ).to_box());
    world.surfaces.push(Sphere::new(Vector3::new(-2.2, 0.0, -2.5),
                                                 0.5,
                                                 MaterialType::Lambert(Texture::SolidColor(Vector3::new(0.1, 0.3, 0.9)), 1.0)
    ).to_box());
    world.surfaces.push(Sphere::new(Vector3::new(0.5, 0.0, -2.3),
                                             0.4,
                                             MaterialType::Lambert(Texture::SolidColor(Vector3::new(1.0, 0.8, 0.5)), 1.0)
    ).to_box());
    world.surfaces.push(Sphere::new(Vector3::new(1.1, 0.0, -1.0),
                                             0.5,
                                             MaterialType::Metal(Texture::ImageTexture(Rc::from(earth)), 0.3)
    ).to_box());
    world.surfaces.push(Sphere::new(Vector3::new(-1.2, 0.0, -1.0),
                                             0.4,
                                             MaterialType::Dielectric(Texture::SolidColor(Vector3::new(0.6, 1.0, 1.0)),1.325, 0.1)
    ).to_box());
    world.surfaces.push(Sphere::new(Vector3::new(0.0, -100.5, -1.0),
                                    100.0,
                                    MaterialType::Lambert(Texture::CheckerBoard(Vector3::new(0.2, 0.2, 0.2), Vector3::new(0.8, 0.8, 0.8)), 1.0)
    ).to_box());
    let cam = Camera::new(Vector3::new(4.0,2.0,2.0),
                          Vector3::new(0.0,0.0,-1.0),
                          Vector3::new(0.0,1.0,0.0),
                          25.0,
                          width as f64 / height as f64
    );

    for x in 0..width {
        for y in 0..height {
            let mut col = Vector3::new(0.0, 0.0, 0.0);
            for _ in 0..samples {
                let mut offset_x = x as f64;
                let mut offset_y = y as f64;
                if samples > 1{
                    offset_x -= rng.gen::<f64>();
                    offset_y -= rng.gen::<f64>();
                }
                let uv: Vector2<f64> = Vector2::new(1.0 -(offset_x / (width-1) as f64),
                                                    1.0 - (offset_y / (height-1) as f64));

                let r = cam.get_ray(uv);
                col += ray_color(&r, &world, recursion_depth, 1.0);
            }

            col /= samples as f64;
            col = Vector3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());
            let i_col: Vector3<u8> = col.map(|c| (c * 255.99) as u8);
            img.set_pixel(x, y, px!(i_col.x, i_col.y, i_col.z));
        }
    }
    let _ = img.save("img.bmp");
}
