//! Semantic motion functions.

pub fn velocity(_embedding: &[f32], _t: f32) -> Vec<f32> {
    Vec::new()
}

pub fn update_embedding(embedding: &mut [f32], dt: f32) {
    let v = velocity(embedding, 0.0);
    for (e, dv) in embedding.iter_mut().zip(v) {
        *e += dv * dt;
    }
}
