use crate::utils::GAME_WIDTH;
use macroquad::prelude::*;

const PRESS_TO_CONTINUE: &str = "* Press Any Key to Continue! *";

pub fn update() -> bool {
    if !get_keys_pressed().is_empty() {
        clear_input_queue();
        true
    } else {
        false
    }
}

pub fn draw() {
    let text_width = measure_text(PRESS_TO_CONTINUE, None, 16, 1.0).width;
    draw_text(
        PRESS_TO_CONTINUE,
        (GAME_WIDTH - text_width) / 2.0,
        200.0,
        16.0,
        WHITE,
    );
}
