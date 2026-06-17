use crate::utils::lerp;
use macroquad::prelude::*;

const INITIAL_SPEED: f32 = -100.0;
const ACCEL: f32 = -75.0;
const TEX_RECTS: [Rect; 4] = [
    Rect::new(0.0, 80.0, 6.0, 4.0),
    Rect::new(0.0, 84.0, 6.0, 4.0),
    Rect::new(0.0, 88.0, 6.0, 4.0),
    Rect::new(0.0, 92.0, 6.0, 4.0),
];

pub struct Bullet {
    pub position: Rect,
    pub prev_pos: Rect,
    speed: f32,
    frame: usize,
    animation_timer: f32,
}

impl Bullet {
    pub const fn new(position: Vec2) -> Bullet {
        let rect: Rect = Rect::new(
            position.x - TEX_RECTS[0].w / 2.0,
            position.y,
            TEX_RECTS[0].w,
            TEX_RECTS[0].h,
        );
        Bullet {
            position: rect,
            prev_pos: rect,
            speed: INITIAL_SPEED,
            frame: 0,
            animation_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.animation_timer += dt * 12.0;
        if self.animation_timer >= 4.0 {
            self.animation_timer -= 4.0;
        }
        self.frame = self.animation_timer as usize;
    }

    pub fn update_physics(&mut self, delta: f32, world: Rect) -> bool {
        self.prev_pos = self.position;
        self.speed += ACCEL * delta;
        self.position.y += self.speed * delta;
        if self.position.y < world.y {
            return false;
        }
        // for enemy in &mut enemy_manager.enemies {
        //     if enemy.life > 0 && self.position.overlaps(&enemy.position) {
        //         enemy.life -= 1;
        //         return false;
        //     }
        // }
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
