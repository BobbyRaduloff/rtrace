pub mod rtx;

use rtx::{Camera, HittableList, Sphere, Vector};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vector::new([0.0, 0.0, -1.0]), 0.5)));
    world.add(Box::new(Sphere::new(
        Vector::new([0.0, -100.5, -1.0]),
        100.0,
    )));

    camera.render(&world);
}
