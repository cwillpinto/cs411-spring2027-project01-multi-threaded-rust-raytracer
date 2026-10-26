use crate::renderer::Scene;

// TODO: return RGB bytes for rows [start_row, end_row), in row-major order.
// Use renderer::pixel for each pixel. Do not mutate a shared frame buffer.
pub fn render_rows(scene: &Scene, start_row: usize, end_row: usize) -> Vec<u8> {
    let _ = (scene, start_row, end_row);
    todo!("render this worker's exclusive row band")
}
