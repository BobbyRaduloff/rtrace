use std::sync::Arc;

use super::{
    Camera, DielectricData, Hittable, LambertianData, Material, MetalData, SphereData, Target,
    TargetList, Vector, RGB,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Scene {
    pub image_width: usize,
    pub image_height: usize,
    pub scene: Arc<Hittable>,
    pub camera: Camera,
}

pub fn get() -> Scene {
    let image_width = 480;
    let image_height = 320;
    let samples_per_pixel = 100;
    let max_depth = 25;
    let vfov = 20.0;
    let lookfrom = Vector::new([13.0, 2.0, 3.0]);
    let lookat = Vector::new([0.0, 0.0, 0.0]);
    let vup = Vector::new([0.0, 1.0, 0.0]);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;

    let camera = Camera::new(
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        vfov,
        lookfrom,
        lookat,
        vup,
        defocus_angle,
        focus_dist,
    );

    let mut world = TargetList::new();

    let ground_material = Material::Lambertian(LambertianData::new(RGB::new([0.5, 0.5, 0.5])));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([0.0, -1000.0, 0.0]),
        1000.0,
        ground_material,
    )));

    for a in -2..2 {
        for b in -2..2 {
            let choose_mat = fastrand::f64();
            let center = Vector::new([
                a as f64 + 0.9 * fastrand::f64(),
                0.2,
                b as f64 + 0.9 * fastrand::f64(),
            ]);

            if (center - Vector::new([4.0, 0.2, 0.0])).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = RGB::random() * RGB::random();
                    Material::Lambertian(LambertianData::new(albedo))
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = RGB::new([
                        (fastrand::f64() + 1.0) / 2.0,
                        (fastrand::f64() + 1.0) / 2.0,
                        (fastrand::f64() + 1.0) / 2.0,
                    ]);
                    let fuzz = fastrand::f64() / 2.0;
                    Material::Metal(MetalData::new(albedo, fuzz))
                } else {
                    // Glass
                    Material::Dielectric(DielectricData::new(1.5))
                };

                world.add(Target::Sphere(SphereData::new(
                    center,
                    0.2,
                    sphere_material,
                )));
            }
        }
    }

    let material1 = Material::Dielectric(DielectricData::new(1.5));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([0.0, 1.0, 0.0]),
        1.0,
        material1,
    )));

    let material2 = Material::Lambertian(LambertianData::new(RGB::new([0.4, 0.2, 0.1])));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([-4.0, 1.0, 0.0]),
        1.0,
        material2,
    )));

    let material3 = Material::Metal(MetalData::new(RGB::new([0.7, 0.6, 0.5]), 0.0));
    world.add(Target::Sphere(SphereData::new(
        Vector::new([4.0, 1.0, 0.0]),
        1.0,
        material3,
    )));

    Scene {
        image_width,
        image_height,
        scene: Arc::new(Hittable::Multiple(world)),
        camera,
    }
}
