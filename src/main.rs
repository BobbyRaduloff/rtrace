pub mod rtx;

use rtx::{
    Camera, DielectricData, Hittable, LambertianData, Material, MetalData, SphereData, Target,
    TargetList, Vector, RGB,
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 90.0;

    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
    );

    let mut world = TargetList::new();

    let material_ground = Material::Lambertian(LambertianData::new(RGB::new([0.8, 0.8, 0.0])));
    let material_center = Material::Lambertian(LambertianData::new(RGB::new([0.1, 0.2, 0.5])));
    let material_left = Material::Dielectric(DielectricData::new(1.5));
    let material_bubble = Material::Dielectric(DielectricData::new(1.0 / 1.50));
    let material_right = Material::Metal(MetalData::new(RGB::new([0.8, 0.6, 0.2]), 1.0));

    world.add(Target::Sphere(SphereData::new(
        Vector::new([0.0, -100.5, -1.0]),
        100.0,
        material_ground,
    )));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([0.0, 0.0, -1.2]),
        0.5,
        material_center,
    )));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([-1.0, 0.0, -1.0]),
        0.5,
        material_left,
    )));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([-1.0, 0.0, -1.0]),
        0.4,
        material_bubble,
    )));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([1.0, 0.0, -1.0]),
        0.5,
        material_right,
    )));

    camera.render(&Hittable::Multiple(world));
}
