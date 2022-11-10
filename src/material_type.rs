use cgmath::*;
use crate::texture::Texture;

#[derive(Clone)]
pub enum MaterialType{
    Lambert(Texture, f64),
    Metal(Texture, f64),
    Dielectric(Texture, f64, f64),
}
