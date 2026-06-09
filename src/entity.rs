use macroquad::prelude::*;

pub struct Entity {
    pub position: Vec2,
    pub prev_pos: Vec2,
    speed: Vec2,
    damp: f32,
    pub tex_rect: Rect,
}

impl Entity {
    pub fn new(position: Vec2, speed: Vec2, damp: f32, tex_rect: Rect) -> Self {
        Self {
            position,           // Shorthand for position: position
            prev_pos: position, // Copy or clone the initial position into prev_pos
            speed,
            damp,
            tex_rect,
        }
    }

    pub fn update_physics(&mut self, delta: f32, g: f32, world: Rect) {
        self.speed.y += g * delta;
        self.prev_pos = self.position;
        self.position += self.speed * delta;
        if self.position.y < 0.0 {
            self.position.y = 0.0;
            if self.speed.y.abs() < 0.5 {
                self.speed.y = 0.0;
            } else {
                self.speed.y *= self.damp;
            }
        } else if self.position.y + self.tex_rect.h > world.h {
            self.position.y = world.h - self.tex_rect.h;
            if self.speed.y.abs() < 0.5 {
                self.speed.y = 0.0;
            } else {
                self.speed.y *= self.damp;
            }
        }
        if self.position.x < 0.0 {
            self.position.x = 0.0;
            if self.speed.x.abs() < 0.5 {
                self.speed.x = 0.0;
            } else {
                self.speed.x *= self.damp;
            }
        } else if self.position.x + self.tex_rect.w > world.w {
            self.position.x = world.w - self.tex_rect.w;
            if self.speed.x.abs() < 0.5 {
                self.speed.x = 0.0;
            } else {
                self.speed.x *= self.damp;
            }
        }
    }
}
