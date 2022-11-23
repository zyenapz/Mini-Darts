pub fn normalize(val: f32, min: f32, max: f32) -> f32 {
    (val - min) / (max - min)
}

pub fn round_to_two(val: f32) -> f32 {
    (val * 100.0).round() / 100.0
}