use rtx::sample_scene;

pub mod rtx;

fn main() {
    let scene = sample_scene::get();
    let results = scene.camera.render_st(scene.scene);
    println!("{}", scene.camera.rgb_array_to_ppm(results));
}
