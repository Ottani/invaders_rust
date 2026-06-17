use crate::bullet::Bullet;
use crate::rock::ROCK_SIZE;
use crate::rock::Rock;
use crate::utils::GAME_WIDTH;
use macroquad::prelude::*;

const NUM_ROCKS: usize = 5;
const MAX_BULLETS: usize = 5;

pub struct GameState {
    rocks: [Option<Rock>; NUM_ROCKS],
    bullets: [Option<Bullet>; MAX_BULLETS],
}

impl GameState {
    pub fn new(sheet_image: &Image) -> Self {
        let rocks = Self::create_rocks(sheet_image);
        Self {
            rocks,
            bullets: Default::default(),
        }
    }

    fn create_rocks(sheet_image: &Image) -> [Option<Rock>; NUM_ROCKS] {
        const ROCK_GAP: f32 = 64.0;
        let mut rocks: [Option<Rock>; NUM_ROCKS] = Default::default();
        let start_pos = vec2(
            (GAME_WIDTH - (NUM_ROCKS as f32 * ROCK_SIZE) - (NUM_ROCKS as f32 - 1.0) * ROCK_GAP)
                / 2.0,
            240.0,
        );

        for i in 0..NUM_ROCKS {
            let image = sheet_image.sub_image(Rect::new(192.0, 32.0, 48.0, 48.0));
            rocks[i] = Some(Rock::new(
                vec2(start_pos.x + i as f32 * (ROCK_SIZE + ROCK_GAP), start_pos.y),
                &image,
            ));
        }

        rocks
    }

    pub fn create_bullet(&mut self, pos: Vec2) {
        if let Some(slot) = self.bullets.iter_mut().find(|s| s.is_none()) {
            *slot = Some(Bullet::new(pos));
        }
    }

    pub fn update_animations(&mut self, frame_time: f32) {
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                bullet.update(frame_time);
            }
        }
    }

    pub fn update_physics(&mut self, delta: f32, world: Rect) {
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                if !bullet.update_physics(delta, world) {
                    *slot = None;
                    continue;
                }
            }
            // TODO move collisions to somewhere here
        }
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        for rock_slot in &self.rocks {
            if let Some(rock) = rock_slot {
                rock.draw();
            }
        }

        for slot in self.bullets.iter() {
            if let Some(bullet) = slot {
                bullet.draw(alpha, texture);
            }
        }
    }
}
