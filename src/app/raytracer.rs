use nalgebra::UnitVector4;
use rayon::prelude::*;
use std::future::Future;

pub struct Raytracer {
    width: u32,
    height: u32,
    bounces: u32,
}

impl Raytracer {
    pub fn new(width: u32, height: u32, bounces: u32) -> Self {
        Self {
            width,
            height,
            bounces,
        }
    }

    pub fn raytrace(&self, data: &mut [u32]) {
        data.par_chunks_mut(self.width as usize)
            .enumerate()
            .for_each(|(y, row)| {
                row.iter_mut().enumerate().for_each(|(x, pixel)| {
                    let result = self.fire_ray(x, y).push(1.);
                    let result = UnitVector4::new_normalize(result);
                    *pixel = pixel_from_vector(result);
                })
            })
    }

    fn fire_ray(&self, x: usize, y: usize) -> nalgebra::Vector3<f32> {
        todo!()
    }
}

fn pixel_from_vector(rgba: UnitVector4<f32>) -> u32 {
    let rgba = [
        (255. * rgba.x) as u8,
        (255. * rgba.y) as u8,
        (255. * rgba.z) as u8,
        (255. * rgba.z) as u8,
    ];
    u32::from_le_bytes(rgba)
}
