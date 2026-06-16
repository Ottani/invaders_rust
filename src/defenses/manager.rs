use super::defense;
use crate::utils::GAME_WIDTH;
use defense::Defense;
use macroquad::prelude::*;

const DEFENSE_SIZE: f32 = 48.0;
const DEFENSE_GAP: f32 = 64.0;
const NUM_DEFENSES: usize = 5;
const EMPTY_DEFENSE: Option<Defense> = None;

pub struct DefenseManager {
    defenses: [Option<Defense>; NUM_DEFENSES],
}

impl DefenseManager {
    pub fn new() -> Self {
        Self {
            defenses: [EMPTY_DEFENSE; NUM_DEFENSES],
        }
    }

    pub fn create_defenses(&mut self) {
        let start_pos = vec2(
            (GAME_WIDTH
                - (NUM_DEFENSES as f32 * DEFENSE_SIZE)
                - (NUM_DEFENSES as f32 - 1.0) * DEFENSE_GAP)
                / 2.0,
            240.0,
        );
        for i in 0..NUM_DEFENSES {
            self.defenses[i] = Some(defense::Defense::new(vec2(
                start_pos.x + i as f32 * (DEFENSE_SIZE + DEFENSE_GAP),
                start_pos.y,
            )));
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        for defense in &self.defenses {
            if let Some(defense) = defense {
                defense.draw(texture);
            }
        }
    }
}
