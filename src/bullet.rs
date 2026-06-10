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
const MAX_BULLETS: usize = 5;
const EMPTY_BULLET: Option<Bullet> = None;

pub struct Bullet {
    position: Rect,
    prev_pos: Rect,
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
}

pub struct BulletManager {
    bullets: [Option<Bullet>; MAX_BULLETS],
}

impl BulletManager {
    pub fn new() -> BulletManager {
        BulletManager {
            bullets: [EMPTY_BULLET; MAX_BULLETS],
        }
    }

    pub fn create_bullet(&mut self, pos: Vec2) {
        if let Some(slot) = self.bullets.iter_mut().find(|s| s.is_none()) {
            *slot = Some(Bullet::new(pos));
        }
    }

    pub fn update(&mut self, dt: f32) {
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                bullet.animation_timer += dt * 12.0;

                if bullet.animation_timer >= 4.0 {
                    bullet.animation_timer -= 4.0;
                }

                bullet.frame = bullet.animation_timer as usize;
            }
        }
    }

    pub fn update_physics(&mut self, delta: f32, world: Rect) {
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                bullet.prev_pos = bullet.position;
                bullet.speed += ACCEL * delta;
                bullet.position.y += bullet.speed * delta;
                if bullet.position.y < world.y {
                    *slot = None;
                }
            }
        }
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        for slot in self.bullets.iter() {
            if let Some(bullet) = slot {
                draw_texture_ex(
                    &texture,
                    lerp(bullet.prev_pos.x, bullet.position.x, alpha).floor(),
                    lerp(bullet.prev_pos.y, bullet.position.y, alpha).floor(),
                    WHITE,
                    DrawTextureParams {
                        source: Some(TEX_RECTS[bullet.frame]),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
