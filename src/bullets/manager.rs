use super::bullet::Bullet;
use crate::enemies::EnemyManager;
use macroquad::prelude::*;

const MAX_BULLETS: usize = 5;
const EMPTY_BULLET: Option<Bullet> = None;

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
                bullet.update(dt);
            }
        }
    }

    pub fn update_physics(&mut self, delta: f32, world: Rect, enemy_manager: &mut EnemyManager) {
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                if !bullet.update_physics(delta, world, enemy_manager) {
                    *slot = None;
                    continue;
                }
            }
        }
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        for slot in self.bullets.iter() {
            if let Some(bullet) = slot {
                bullet.draw(alpha, texture);
            }
        }
    }
}
