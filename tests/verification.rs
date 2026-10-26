use cs411_project01_raytracer::renderer::{image_hash, render_parallel, render_sequential, Scene};
use std::sync::Arc;

#[test]
fn parallel_matches_sequential_at_boundaries() {
    for (width, height) in [(1, 1), (7, 5), (31, 17)] {
        let mut scene = Scene::default();
        scene.width = width;
        scene.height = height;
        let scene = Arc::new(scene);
        let baseline = render_sequential(&scene);
        for workers in [1, 2, 4, 8] {
            let actual = render_parallel(&scene, workers);
            assert_eq!(
                actual, baseline,
                "dimensions {width}x{height}, workers {workers}"
            );
            assert_eq!(image_hash(&actual), image_hash(&baseline));
        }
    }
}

#[test]
fn repeated_parallel_images_are_identical() {
    let scene = Arc::new(Scene::default());
    let first = render_parallel(&scene, 4);
    assert_eq!(first, render_parallel(&scene, 4));
}
