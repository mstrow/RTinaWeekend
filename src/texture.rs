use std::fs::copy;
use std::rc::Rc;
use cgmath::{Vector2, Vector3};
use crate::num_traits::{clamp, Pow};
use bmp::{Image, Pixel, px};

#[derive(Clone)]
pub enum Texture {
    SolidColor(Vector3<f64>),
    CheckerBoard(Vector3<f64>, Vector3<f64>),
    ImageTexture(Rc<Image>)
}
impl Texture {
    pub fn color(&self, uv: &Vector2<f64>, pos: &Vector3<f64>) -> Vector3<f64> {
        match self {
            Texture::SolidColor(color) => *color,
            Texture::CheckerBoard(color1, color2) => {
                let sines = Vector3::new(
                    (pos.x * 10.0).sin(),
                    (pos.y * 10.0).sin(),
                    (pos.z * 10.0).sin()
                );
                if (sines.x * sines.y * sines.z) < 0.0 {
                    return *color1
                } else {
                    return *color2
                }
            },
            Texture::ImageTexture(img) => {
                let u = (clamp(uv.x,0.0,1.0) * img.get_width() as f64);
                let v = ((1.0 - clamp(uv.y,0.0,1.0)) * img.get_height() as f64);
                let pixel = img.get_pixel(u as u32, v as u32);
                let mut col = Vector3::new(pixel.r as f64 / 255.99, pixel.g as f64 / 255.99, pixel.b as f64 / 255.99);
                col = Vector3::new(col.x.pow(2), col.y.pow(2), col.z.pow(2));
                return col;
            }
        }
    }
}


