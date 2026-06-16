use super::defense;
use crate::utils::GAME_WIDTH;
use macroquad::prelude::*;

const DEFENSE_SIZE: f32 = 48.0;
const DEFENSE_GAP: f32 = 64.0;

pub struct DefenseManager {
    defenses: Vec<defense::Defense>,
}

impl DefenseManager {
    pub fn new() -> Self {
        Self {
            defenses: Vec::new(),
        }
    }

    pub fn create_defenses(&mut self) {
        const NUM_DEFENSES: usize = 5;
        let start_pos = vec2(
            (GAME_WIDTH
                - (NUM_DEFENSES as f32 * DEFENSE_SIZE)
                - (NUM_DEFENSES as f32 - 1.0) * DEFENSE_GAP)
                / 2.0,
            240.0,
        );
        for i in 0..NUM_DEFENSES {
            self.defenses.push(defense::Defense::new(vec2(
                start_pos.x + i as f32 * (DEFENSE_SIZE + DEFENSE_GAP),
                start_pos.y,
            )));
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        for defense in &self.defenses {
            defense.draw(texture);
        }
    }
}
