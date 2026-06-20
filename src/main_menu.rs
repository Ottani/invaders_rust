use crate::utils::GAME_WIDTH;
use macroquad::prelude::*;

const PRESS_TO_START: &str = "* Press Any Key to Start! *";

pub struct MainMenu {}

impl MainMenu {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self) -> bool {
        if get_keys_pressed().len() > 0 {
            true
        } else {
            false
        }
    }

    pub fn draw(&self) {
        let text_width = measure_text(PRESS_TO_START, None, 16, 1.0).width;
        draw_text(
            PRESS_TO_START,
            (GAME_WIDTH - text_width) / 2.0,
            200.0,
            16.0,
            WHITE,
        );
    }
}
