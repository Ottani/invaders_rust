use macroquad::prelude::*;

struct Defense {
    max_health: i32,
    health: i32,
    position: Vec2,
    sprite: Texture2D,
    size: Rect,
    rects: Vec<Rect>,
}
