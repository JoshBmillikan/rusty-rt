use crate::app::raytracer::camera::Camera;
use rayon::prelude::*;

mod camera;

pub struct Raytracer {
    width: u32,
    height: u32,
    bounces: u32,
    camera: Camera,
    sphere: Sphere,
}

impl Raytracer {
    pub fn new(width: u32, height: u32, bounces: u32) -> Self {
        let camera = Camera::new(width, height);
        Self {
            width,
            height,
            bounces,
            camera,
            sphere: Sphere {
                center: nalgebra::Point3::new(0., 0., -1.),
                radius: 0.5,
            },
        }
    }

    pub fn raytrace(&self, data: &mut [u32]) {
        data.par_chunks_mut(self.width as usize)
            .enumerate()
            .for_each(|(y, row)| {
                row.iter_mut().enumerate().for_each(|(x, pixel)| {
                    let ray = self.camera.fire_ray(x, y);
                    let result = ray.color(&self.sphere).normalize().zyx().push(1.);
                    *pixel = pixel_from_vector(result);
                })
            })
    }
}

fn pixel_from_vector(rgba: nalgebra::Vector4<f32>) -> u32 {
    let rgba = [
        (255. * rgba.x) as u8,
        (255. * rgba.y) as u8,
        (255. * rgba.z) as u8,
        (255. * rgba.z) as u8,
    ];
    u32::from_le_bytes(rgba)
}

struct Ray {
    origin: nalgebra::Point3<f32>,
    direction: nalgebra::UnitVector3<f32>,
}

impl Ray {
    fn color(&self, sphere: &Sphere) -> nalgebra::Vector3<f32> {
        if sphere.hit(self) {
            nalgebra::Vector3::new(1., 0., 0.)
        } else {
            let direction = self.direction.normalize();
            let a = (direction.y + 1.) * 0.5;
            nalgebra::Vector3::new(1., 1., 1.).lerp(&nalgebra::Vector3::new(0.5, 0.7, 1.), a)
        }
    }
}

struct Sphere {
    center: nalgebra::Point3<f32>,
    radius: f32,
}

impl Sphere {
    fn hit(&self, ray: &Ray) -> bool {
        let oc = self.center - ray.origin;
        let a = ray.direction.dot(&ray.direction);
        let b = -2. * ray.direction.dot(&oc);
        let c = oc.dot(&oc) - self.radius.powi(2);
        let discriminant = b.powi(2) - 4. * a * c;
        discriminant >= 0.
    }
}
