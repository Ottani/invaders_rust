pub const fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + (end - start) * amount
}
