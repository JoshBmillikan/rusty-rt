use crate::app::raytracer::Ray;

struct Viewport {
    aspect_ratio: f32,
    width: f32,
    height: f32,
}

impl Viewport {
    fn new(width: u32, height: u32) -> Self {
        let image_width = width as f32;
        let image_height = height as f32;
        let aspect_ratio = width as f32 / height as f32;
        let height = 2.;
        let width = height * aspect_ratio;

        Self {
            aspect_ratio,
            width,
            height,
        }
    }
}

pub struct Camera {
    focal_length: f32,
    center: nalgebra::Point3<f32>,
    viewport: Viewport,
    pixel_00: nalgebra::Point3<f32>,
    delta_u: nalgebra::Vector3<f32>,
    delta_v: nalgebra::Vector3<f32>,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        let viewport = Viewport::new(width, height);
        let viewport_u = nalgebra::Vector3::new(viewport.width, 0., 0.);
        let viewport_v = nalgebra::Vector3::new(0., -viewport.height, 0.);
        let delta_u = viewport_u / width as f32;
        let delta_v = viewport_v / height as f32;

        let center = nalgebra::Point3::new(0., 0., 0.);
        let upper_left = center - nalgebra::Vector3::z() - viewport_u / 2. - viewport_v / 2.;
        let pixel_00 = upper_left + 0.5 * (delta_u + delta_v);

        Self {
            focal_length: 1.,
            center: Default::default(),
            viewport,
            pixel_00,
            delta_u,
            delta_v,
        }
    }

    pub fn fire_ray(&self, x: usize, y: usize) -> Ray {
        let origin = self.center;
        let center = self.pixel_00 + (x as f32 * self.delta_u) + (y as f32 * self.delta_v);
        let direction = nalgebra::UnitVector3::new_normalize(center - self.center);
        Ray { origin, direction }
    }
}
