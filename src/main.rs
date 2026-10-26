use cs411_project01_raytracer::renderer::{image_hash, render_parallel, render_sequential, Scene};
use std::sync::Arc;
use std::time::Instant;

fn main() {
    let scene = Arc::new(Scene::default());
    let seq = render_sequential(&scene);
    let expected = image_hash(&seq);
    println!("sequential hash: {expected:016x}");
    println!("workers,median_ms,hash");
    let mut final_image = None;
    for workers in [1, 2, 4, 8] {
        let mut times = Vec::new();
        for _ in 0..3 {
            let start = Instant::now();
            let actual = render_parallel(&scene, workers);
            times.push(start.elapsed().as_secs_f64() * 1000.0);
            if actual != seq {
                eprintln!("pixel mismatch at {workers} workers");
                std::process::exit(1);
            }
            final_image = Some(actual);
        }
        times.sort_by(f64::total_cmp);
        println!("{workers},{:.3},{expected:016x}", times[1]);
    }
    image::save_buffer(
        "render.png",
        &final_image.unwrap(),
        scene.width as u32,
        scene.height as u32,
        image::ColorType::Rgb8,
    )
    .expect("write render.png");
    println!("wrote render.png");
}
