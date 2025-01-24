pub mod rtx;

use rtx::{
    Camera, Hittable, LambertianData, Material, SphereData, Target, TargetList, Vector, RGB,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    let mut world = TargetList::new();
    let lambertian = Material::Lambertian(LambertianData::new(RGB::new([0.0, 0.5, 0.0])));

    world.add(Target::Sphere(SphereData::new(
        Vector::new([0.0, 0.0, -1.0]),
        0.5,
        lambertian,
    )));

    world.add(Target::Sphere(SphereData::new(
        Vector::new([0.0, -100.5, -1.0]),
        100.0,
        lambertian,
    )));

    camera.render(&Hittable::Multiple(world));
}
