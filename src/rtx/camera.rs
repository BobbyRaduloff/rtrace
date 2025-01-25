use super::{Hittable, HittableObject, Interval, Ray, Vector, RGB};
use std::sync::Arc;
use threadpool::ThreadPool;

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
    pub aspect_ratio: f64,        // Ratio of image width to height
    pub image_width: usize,       // Rendered image width in pixels
    pub image_height: usize,      // Rendered image height in pixels
    pub samples_per_pixel: usize, // antialiasing
    pub max_depth: usize,         // max bounces of a ray
    pub vfov: f64,                // vertical field of view
    pub lookfrom: Vector<3>,      // camera position
    pub lookat: Vector<3>,        // camera target
    pub vup: Vector<3>,           // camera relative up direction
    pub defocus_angle: f64,       // variation angle of rays through each pixel
    pub focus_dist: f64,          // distance from camera lookfrom point to plane of perfect focus
    center: Vector<3>,            // Camera center
    pixel00_loc: Vector<3>,       // Location of pixel (0, 0)
    pixel_delta_u: Vector<3>,     // Horizontal delta to the next pixel
    pixel_delta_v: Vector<3>,     // Vertical delta to the next pixel,
    pixel_sample_scale: f64,      // Color scale factor for sum of pixels
    defocus_disk_u: Vector<3>,    // defocus disk horizontal radius
    defocus_disk_v: Vector<3>,    // defocus disk vertical radius
}

impl Camera {
    pub fn new(
        image_width: usize,
        image_height: usize,
        samples_per_pixel: usize,
        max_depth: usize,
        vfov: f64,
        lookfrom: Vector<3>,
        lookat: Vector<3>,
        vup: Vector<3>,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Self {
        let aspect_ratio = (image_width as f64) / (image_height as f64);
        let center = lookfrom;

        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * (-1.0 * v);

        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

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

    fn get_ray(&self, i: usize, j: usize) -> Ray {
        let offset = Vector::new([fastrand::f64() - 0.5, fastrand::f64() - 0.5]);
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.components[0]) * self.pixel_delta_u)
            + ((j as f64 + offset.components[1]) * self.pixel_delta_v);
        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;

        Ray::new(origin, direction.normalize())
    }

    fn defocus_disk_sample(&self) -> Vector<3> {
        let p = Vector::<3>::random_in_unit_disk();
        self.center
            + (p.components[0] * self.defocus_disk_u)
            + (p.components[1] * self.defocus_disk_v)
    }

    fn ray_color(&self, ray: Ray, world: Arc<Hittable>, depth: usize) -> RGB {
        if depth <= 0 {
            return RGB::new([0.0, 0.0, 0.0]);
        }

        match world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            Some(hit) => {
                let scatter = hit.material.scatter(ray, hit);
                scatter.attenuation * self.ray_color(scatter.scattered, world, depth - 1)
            }
            None => {
                let direction = ray.direction.normalize();
                let t = 0.5 * (direction.components[1] + 1.0);
                (1.0 - t) * RGB::new([1.0, 1.0, 1.0]) + t * RGB::new([0.5, 0.7, 1.0])
            }
        }
    }

    pub fn render(&self, world: Arc<Hittable>) -> Vec<RGB> {
        let mut image = Vec::<RGB>::new();

        for y in 0..self.image_height {
            let world = world.clone();
            eprint!("\rScanlines remaining: {}", self.image_height - y);
            let mut row = self.render_row(world, y);
            image.append(&mut row);
        }

        eprintln!("\nDone.");
        image
    }

    pub fn render_mt(
        &self,
        world: Arc<Hittable>,
        num_threads: usize,
        rows_per_task: usize,
    ) -> Vec<RGB> {
        use std::sync::mpsc::channel;

        let pool = ThreadPool::new(num_threads);
        let (tx, rx) = channel();

        let mut tasks = Vec::new();
        let mut current_row = 0;

        while current_row < self.image_height {
            let start_row = current_row;
            let end_row = (current_row + rows_per_task).min(self.image_height);
            tasks.push(RenderTask::new(start_row, end_row));
            current_row = end_row;
        }

        for task in tasks.clone() {
            let tx = tx.clone();
            let camera = self.clone();
            let world = world.clone();

            pool.execute(move || {
                let mut rows = Vec::new();
                for y in task.start_row..task.end_row {
                    rows.push((y, camera.render_row(world.clone(), y)));
                }
                tx.send(rows).expect("Failed to send task results");
            });
        }

        // Collect rows from threads
        let mut row_buffers = vec![Vec::new(); self.image_height];
        for _ in 0..tasks.len() {
            let rows = rx.recv().expect("Failed to receive task results");
            for (y, row) in rows {
                row_buffers[y] = row;
            }
        }

        // Flatten rows into a single vector
        row_buffers.into_iter().flatten().collect()
    }

    fn render_row(&self, world: Arc<Hittable>, y: usize) -> Vec<RGB> {
        let mut row = Vec::<RGB>::new();
        for x in 0..self.image_width {
            let world = world.clone();
            row.push(self.render_pixel(world, y, x));
        }

        row
    }

    fn render_pixel(&self, world: Arc<Hittable>, y: usize, x: usize) -> RGB {
        let mut color = RGB::new([0.0, 0.0, 0.0]);
        for _ in 0..self.samples_per_pixel {
            let ray = self.get_ray(x, y);
            let world = world.clone();
            color = color + self.ray_color(ray, world, self.max_depth);
        }

        color * self.pixel_sample_scale
    }

    pub fn rgb_array_to_ppm(&self, image: Vec<RGB>) -> String {
        let mut ppm_data = format!("P3\n{} {}\n255\n", self.image_width, self.image_height);

        for pixel in image.iter() {
            let r = (pixel.components[0].clamp(0.0, 1.0) * 255.0).round() as u8;
            let g = (pixel.components[1].clamp(0.0, 1.0) * 255.0).round() as u8;
            let b = (pixel.components[2].clamp(0.0, 1.0) * 255.0).round() as u8;
            ppm_data.push_str(&format!("{} {} {} ", r, g, b));
            ppm_data.push('\n');
        }

        ppm_data
    }
}
