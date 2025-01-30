use super::{Ray, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderTask {
    start_row: usize,
    end_row: usize,
}

impl RenderTask {
    pub fn new(start_row: usize, end_row: usize) -> Self {
        Self { start_row, end_row }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub aspect_ratio: f32,        // Ratio of image width to height
    pub image_width: usize,       // Rendered image width in pixels
    pub image_height: usize,      // Rendered image height in pixels
    pub samples_per_pixel: usize, // antialiasing
    pub max_depth: usize,         // max bounces of a ray
    pub vfov: f32,                // vertical field of view
    pub lookfrom: Vector<3>,      // camera position
    pub lookat: Vector<3>,        // camera target
    pub vup: Vector<3>,           // camera relative up direction
    pub defocus_angle: f32,       // variation angle of rays through each pixel
    pub focus_dist: f32,          // distance from camera lookfrom point to plane of perfect focus
    center: Vector<3>,            // Camera center
    pixel00_loc: Vector<3>,       // Location of pixel (0, 0)
    pixel_delta_u: Vector<3>,     // Horizontal delta to the next pixel
    pixel_delta_v: Vector<3>,     // Vertical delta to the next pixel,
    pixel_sample_scale: f32,      // Color scale factor for sum of pixels
    defocus_disk_u: Vector<3>,    // defocus disk horizontal radius
    defocus_disk_v: Vector<3>,    // defocus disk vertical radius
}

impl Camera {
    pub fn new(
        image_width: usize,
        image_height: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f32,
        lookfrom: Vector<3>,
        lookat: Vector<3>,
        vup: Vector<3>,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> Self {
        let aspect_ratio = (image_width as f32) / (image_height as f32);
        let center = lookfrom;

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).normalize();
        let u = (-1.0 * vup.cross(w)).normalize();
        let v = (-1.0 * w).cross(u).normalize();

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * (-1.0 * v);

        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f32;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

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
            lookat,
            lookfrom,
            vup,
            vfov,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            focus_dist,
        }
    }

    pub fn generate_rays(&self) -> Vec<Ray> {
        let mut rays = Vec::new();
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                for _ in 0..self.samples_per_pixel {
                    let offset = Vector::new([fastrand::f32() - 0.5, fastrand::f32() - 0.5]);
                    let pixel_sample = self.pixel00_loc
                        + ((x as f32 + offset.components[0]) * self.pixel_delta_u)
                        + ((y as f32 + offset.components[1]) * self.pixel_delta_v);
                    let origin = if self.defocus_angle <= 0.0 {
                        self.center
                    } else {
                        self.defocus_disk_sample()
                    };
                    let direction = (pixel_sample - origin).normalize();

                    rays.push(Ray {
                        origin: [
                            origin.components[0],
                            origin.components[1],
                            origin.components[2],
                        ],
                        direction: [
                            direction.components[0],
                            direction.components[1],
                            direction.components[2],
                        ],
                        color: [1.0, 1.0, 1.0],
                        bounces_left: self.max_depth as u32,
                        _pad2: 0.0,
                        _pad3: 0.0,
                    });
                }
            }
        }
        rays
    }

    fn defocus_disk_sample(&self) -> Vector<3> {
        let p = Vector::<3>::random_in_unit_disk();
        self.center
            + (p.components[0] * self.defocus_disk_u)
            + (p.components[1] * self.defocus_disk_v)
    }
}
