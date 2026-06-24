use crate::utils::{GAME_HEIGHT, GAME_WIDTH};
use macroquad::prelude::*;

const PRESS_TO_START: &str = "* Press Any Key to Start! *";
const AUTHOR: &str = "Paulo Assis - 2026";
const INSTRUCTIONS: &str = "KEYS:
Move: A, D, arrows
Shoot: Spacebar, W, mouse button
Fullscreen: F11";

pub fn update() -> bool {
    if !get_keys_pressed().is_empty() {
        clear_input_queue();
        true
    } else {
        false
    }
}

pub fn draw() {
    let text_width = measure_text(PRESS_TO_START, None, 16, 1.0).width;
    draw_text(
        PRESS_TO_START,
        (GAME_WIDTH - text_width) / 2.0,
        200.0,
        16.0,
        WHITE,
    );
    let text_width = measure_multiline_text(INSTRUCTIONS, None, 16, 1.0, None).width;
    draw_multiline_text(
        INSTRUCTIONS,
        GAME_WIDTH - text_width - 8.0,
        20.0,
        16.0,
        None,
        YELLOW,
    );
    draw_text(AUTHOR, 8.0, GAME_HEIGHT - 8.0, 16.0, ORANGE);
}
