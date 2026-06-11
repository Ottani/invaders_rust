use super::enemy::{Enemy, EnemyType};
use crate::bomb::BombManager;
use macroquad::{prelude::*, rand::ChooseRandom};

const DOWNWARDS: f32 = 16.0 * 60.0;
const EMPTY_RECT: Rect = Rect::new(f32::MAX, f32::MAX, 0.0, 0.0);
const MIN_SHOOTERS: usize = 2;
const MAX_SHOOTERS: usize = 6;
const SHOOT_DELAY: f32 = 0.75;

pub struct EnemyManager {
    pub enemies: Vec<Enemy>,
    rect: Rect,
    speed: f32,
    shoot_delay: f32,
}

impl EnemyManager {
    pub const fn new() -> Self {
        EnemyManager {
            enemies: Vec::new(),
            rect: EMPTY_RECT,
            speed: 50.0,
            shoot_delay: 0.0,
        }
    }

    pub fn create_enemies(&mut self) {
        let cols = 10;
        let rows = 5;
        let gap = 8.0;
        for y in 0..rows {
            let enemy_type = match y {
                0 => EnemyType::DieHard,
                1 => EnemyType::Strong,
                2 => EnemyType::Normal,
                _ => EnemyType::Weak,
            };
            for x in 0..cols {
                let point = vec2(x as f32 * 32.0 + gap * x as f32, y as f32 * 32.0);
                self.enemies.push(Enemy::new(point, enemy_type));
            }
        }
    }

    fn calculate_area(&mut self) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        self.enemies.retain_mut(|enemy| {
            if enemy.life > 0 {
                min_x = min_x.min(enemy.position.x);
                min_y = min_y.min(enemy.position.y);
                max_x = max_x.max(enemy.position.right());
                max_y = max_y.max(enemy.position.bottom());
            }
            enemy.life > 0
        });
        if self.enemies.is_empty() {
            self.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
            return;
        }
        self.rect = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);
    }

    pub fn update_physics(&mut self, delta: f32, world: Rect, bomb_manager: &mut BombManager) {
        self.calculate_area();
        let prev_pos = self.rect.point();
        self.rect.x += self.speed * delta;
        if self.rect.x > world.w - self.rect.w {
            self.rect.x = world.w - self.rect.w;
            self.rect.y += DOWNWARDS * delta;
            self.speed = -self.speed;
        } else if self.rect.x < 0.0 {
            self.rect.x = 0.0;
            self.rect.y += DOWNWARDS * delta;
            self.speed = -self.speed;
        }
        let movement = self.rect.point() - prev_pos;

        for enemy in &mut self.enemies {
            enemy.prev_pos = enemy.position;
            enemy.position.x += movement.x;
            enemy.position.y += movement.y;
        }

        self.shoot_delay += delta;
        if self.shoot_delay >= SHOOT_DELAY {
            self.shoot_delay = 0.0;
            if !self.enemies.is_empty() {
                let shooters = self
                    .enemies
                    .choose_multiple(rand::gen_range(MIN_SHOOTERS, MAX_SHOOTERS));
                for shooter in shooters {
                    let shoot_position = vec2(
                        shooter.position.x + shooter.position.w * 0.5f32,
                        shooter.position.y + shooter.position.h,
                    );
                    if !bomb_manager.fire_bomb(shoot_position) {
                        break;
                    }
                }
            }
        }
    }

    pub fn draw(&self, alpha: f32, sheet: &Texture2D) {
        for enemy in &self.enemies {
            enemy.draw(alpha, sheet);
        }
    }
}
