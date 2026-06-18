use crate::utils::lerp;
use macroquad::prelude::*;

const MIN_SPEED: f32 = 50.0;
const MAX_SPEED: f32 = 100.0;
const TEX_RECTS: [Rect; 4] = [
    Rect::new(0.0, 128.0, 8.0, 8.0),
    Rect::new(8.0, 128.0, 8.0, 8.0),
    Rect::new(16.0, 128.0, 8.0, 8.0),
    Rect::new(24.0, 128.0, 8.0, 8.0),
];

pub struct Bomb {
    position: Rect,
    prev_pos: Rect,
    speed: f32,
    frame: usize,
    animation_timer: f32,
}

impl Bomb {
    pub fn new(pos: Vec2) -> Self {
        let pos = Rect::new(pos.x, pos.y, 8.0, 8.0);
        let speed = rand::gen_range(MIN_SPEED, MAX_SPEED);
        Self {
            position: pos,
            prev_pos: pos,
            speed,
            frame: 0,
            animation_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.animation_timer += dt * 8.0;
        if self.animation_timer >= 4.0 {
            self.animation_timer -= 4.0;
        }
        self.frame = self.animation_timer as usize;
    }

    pub fn update_physics(&mut self, dt: f32, world: Rect) -> bool {
        self.prev_pos = self.position;
        self.position.y += self.speed * dt;
        if self.position.y > world.h {
            return false;
        }
        true
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        draw_texture_ex(
            &texture,
            lerp(self.prev_pos.x, self.position.x, alpha).floor(),
            lerp(self.prev_pos.y, self.position.y, alpha).floor(),
            WHITE,
            DrawTextureParams {
                source: Some(TEX_RECTS[self.frame]),
                ..Default::default()
            },
        );
    }
}
