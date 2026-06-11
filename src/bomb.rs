use crate::utils::lerp;
use macroquad::prelude::*;

const MAX_BOMBS: usize = 15;
const EMPTY_BOMB: Option<Bomb> = None;
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
    pub const fn new(pos: Vec2, speed: f32) -> Self {
        let pos = Rect::new(pos.x, pos.y, 8.0, 8.0);
        Self {
            position: pos,
            prev_pos: pos,
            speed,
            frame: 0,
            animation_timer: 0.0,
        }
    }

    pub fn update_physics(&mut self, dt: f32, world: Rect) -> bool {
        self.prev_pos = self.position;
        self.position.y += self.speed * dt;
        if self.position.y > world.h {
            return false;
        }
        self.animation_timer += dt * 8.0;
        if self.animation_timer >= 4.0 {
            self.animation_timer -= 4.0;
        }
        self.frame = self.animation_timer as usize;
        true
    }
}

pub struct BombManager {
    bombs: [Option<Bomb>; MAX_BOMBS],
}

impl BombManager {
    pub fn new() -> Self {
        Self {
            bombs: [EMPTY_BOMB; MAX_BOMBS],
        }
    }

    pub fn fire_bomb(&mut self, pos: Vec2) -> bool {
        if let Some(slot) = self.bombs.iter_mut().find(|s| s.is_none()) {
            *slot = Some(Bomb::new(
                vec2(pos.x + 4.0, pos.y),
                rand::gen_range(MIN_SPEED, MAX_SPEED),
            ));
            return true;
        }
        false
    }

    pub fn update_physics(&mut self, dt: f32, world: Rect) {
        for slot in self.bombs.iter_mut() {
            if let Some(bomb) = slot.as_mut() {
                if !bomb.update_physics(dt, world) {
                    *slot = None;
                }
            }
        }
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        for slot in self.bombs.iter() {
            if let Some(bomb) = slot {
                draw_texture_ex(
                    &texture,
                    lerp(bomb.prev_pos.x, bomb.position.x, alpha).floor(),
                    lerp(bomb.prev_pos.y, bomb.position.y, alpha).floor(),
                    WHITE,
                    DrawTextureParams {
                        source: Some(TEX_RECTS[bomb.frame]),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
