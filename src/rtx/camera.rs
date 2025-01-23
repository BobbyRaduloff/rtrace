use super::{Hittable, Interval, Ray, Vector, RGB};
use std::f64::INFINITY;

pub struct Camera {
    pub aspect_ratio: f64,        // Ratio of image width to height
    pub image_width: usize,       // Rendered image width in pixels
    pub image_height: usize,      // Rendered image height in pixels
    pub samples_per_pixel: usize, // antialiasing
    pub max_depth: usize,
    center: Vector<3>,        // Camera center
    pixel00_loc: Vector<3>,   // Location of pixel (0, 0)
    pixel_delta_u: Vector<3>, // Horizontal delta to the next pixel
    pixel_delta_v: Vector<3>, // Vertical delta to the next pixel,
    pixel_sample_scale: f64,  // Color scale factor for sum of pixels
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let image_height = (image_width as f64 / aspect_ratio).ceil() as usize;
        let center = Vector::new([0.0, 0.0, 0.0]);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;

        let viewport_u = Vector::new([viewport_width, 0.0, 0.0]);
        let viewport_v = Vector::new([0.0, -viewport_height, 0.0]);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left =
            center - Vector::new([0.0, 0.0, focal_length]) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            pixel_sample_scale,
            max_depth,
        }
    }

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Vector::new([fastrand::f64() - 0.5, fastrand::f64() - 0.5]);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.components[0]) * self.pixel_delta_u)
            + ((j as f64 + offset.components[1]) * self.pixel_delta_v);
        let origin = self.center;
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for y in 0..self.image_height {
            eprint!("\rScanlines remaining: {}", self.image_height - y);
            for x in 0..self.image_width {
                let mut color = RGB::new([0.0, 0.0, 0.0]);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    color = color + self.ray_color(ray, world, self.max_depth);
                }

                println!("{}", color * self.pixel_sample_scale);
            }
        }

        eprintln!("\nDone.");
    }

    fn ray_color(&self, ray: Ray, target: &dyn Hittable, depth: usize) -> RGB {
        if depth <= 0 {
            return RGB::new([0.0, 0.0, 0.0]);
        }

        match target.hit(ray, Interval::new(0.001, INFINITY)) {
            Some(hit) => {
                let direction = hit.normal + Vector::<3>::random_unit_vector();
                0.5 * self.ray_color(Ray::new(hit.p, direction), target, depth - 1)
            }
            None => {
                let direction = ray.direction.normalize();
                let a = 0.5 * (direction.components[1] + 1.0);
                (1.0 - a) * RGB::new([1.0, 1.0, 1.0]) + a * RGB::new([0.5, 0.7, 1.0])
            }
        }
    }
}
