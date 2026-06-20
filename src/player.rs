use crate::utils::lerp;
use macroquad::prelude::*;

pub struct Player {
    pub position: Rect,
    prev_position: Rect,
    pub direction: f32,
    speed: f32,
    rect: Rect,
}

impl Player {
    pub fn new(pos: Vec2) -> Self {
        let position = Rect::new(pos.x, pos.y, 32.0, 32.0);
        Self {
            position,
            prev_position: position,
            direction: 0.0,
            speed: 250.0,
            rect: Rect::new(0.0, 0.0, 32.0, 32.0),
        }
    }

    pub fn update_physics(&mut self, dt: f32, world: Rect) {
        self.prev_position = self.position;
        self.position.x += self.direction * self.speed * dt;
        if self.position.x < 0.0 {
            self.position.x = 0.0;
        } else if self.position.x > world.w - 32.0 {
            self.position.x = world.w - 32.0;
        }
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        draw_texture_ex(
            &texture,
            lerp(self.prev_position.x, self.position.x, alpha).floor(),
            lerp(self.prev_position.y, self.position.y, alpha).floor(),
            WHITE,
            DrawTextureParams {
                source: Some(self.rect),
                ..Default::default()
            },
        );
    }
}
