use rtx::sample_scene;

pub mod rtx;

fn main() {
    let num_threads = 12;
    let scene = sample_scene::get();
    let results =
        scene
            .camera
            .render_mt(scene.scene, num_threads, scene.image_height / num_threads);
    println!("{}", scene.camera.rgb_array_to_ppm(results));
}
