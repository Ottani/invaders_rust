pub const GAME_WIDTH: f32 = 640.0;
pub const GAME_HEIGHT: f32 = 360.0;

pub const fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + (end - start) * amount
}
