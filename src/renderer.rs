use std::sync::Arc;

#[derive(Clone)]
pub struct Scene {
    pub width: usize,
    pub height: usize,
    pub spheres: Vec<Sphere>,
}

#[derive(Clone)]
pub struct Sphere {
    pub center: [f64; 3],
    pub radius: f64,
    pub color: [u8; 3],
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            width: 640,
            height: 360,
            spheres: vec![
                Sphere {
                    center: [-0.9, 0.0, -3.2],
                    radius: 0.7,
                    color: [236, 91, 58],
                },
                Sphere {
                    center: [0.8, -0.15, -4.0],
                    radius: 1.0,
                    color: [58, 169, 204],
                },
                Sphere {
                    center: [0.0, -1001.0, -3.0],
                    radius: 1000.0,
                    color: [140, 160, 120],
                },
            ],
        }
    }
}

// Fixed pinhole camera and one diffuse light; the identical pixel function is
// used by both implementations so this project focuses on work decomposition.
pub fn pixel(scene: &Scene, x: usize, y: usize) -> [u8; 3] {
    let aspect = scene.width as f64 / scene.height as f64;
    let dx = ((x as f64 + 0.5) / scene.width as f64 * 2.0 - 1.0) * aspect;
    let dy = 1.0 - (y as f64 + 0.5) / scene.height as f64 * 2.0;
    let mut direction = [dx, dy, -1.7];
    let length = (direction.iter().map(|v| v * v).sum::<f64>()).sqrt();
    for v in &mut direction {
        *v /= length;
    }
    let mut nearest = f64::INFINITY;
    let mut hit = None;
    for sphere in &scene.spheres {
        let c = sphere.center;
        let b = -2.0 * (direction[0] * c[0] + direction[1] * c[1] + direction[2] * c[2]);
        let q = c.iter().map(|v| v * v).sum::<f64>() - sphere.radius * sphere.radius;
        let discriminant = b * b - 4.0 * q;
        if discriminant < 0.0 {
            continue;
        }
        let t = (-b - discriminant.sqrt()) / 2.0;
        if t > 0.001 && t < nearest {
            nearest = t;
            hit = Some(sphere);
        }
    }
    if let Some(sphere) = hit {
        let point = direction.map(|v| v * nearest);
        let normal = [
            (point[0] - sphere.center[0]) / sphere.radius,
            (point[1] - sphere.center[1]) / sphere.radius,
            (point[2] - sphere.center[2]) / sphere.radius,
        ];
        let light = [0.4, 0.8, 0.4];
        let diffuse = (normal[0] * light[0] + normal[1] * light[1] + normal[2] * light[2]).max(0.0);
        let brightness = 0.2 + 0.8 * diffuse;
        return sphere.color.map(|v| (v as f64 * brightness) as u8);
    }
    [18, 24, 42]
}

pub fn render_sequential(scene: &Scene) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(scene.width * scene.height * 3);
    for y in 0..scene.height {
        for x in 0..scene.width {
            bytes.extend_from_slice(&pixel(scene, x, y));
        }
    }
    bytes
}

// TODO: partition rows into bounded, nonoverlapping bands; spawn no more than
// workers threads, each with an Arc clone of the immutable scene; assemble
// returned bands by row index. `worker::render_rows` owns each band's bytes.
pub fn render_parallel(scene: &Arc<Scene>, workers: usize) -> Vec<u8> {
    let _ = (scene, workers);
    todo!("assemble independently rendered row bands")
}

pub fn image_hash(bytes: &[u8]) -> u64 {
    bytes.iter().fold(0xcbf29ce484222325u64, |hash, byte| {
        (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3)
    })
}
