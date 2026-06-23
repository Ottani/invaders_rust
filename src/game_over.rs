use crate::ui::{MenuAction, MenuItem};
use crate::utils::GAME_WIDTH;
use macroquad::prelude::*;

const GAME_OVER_TEXT: &str = "GAME OVER!";

pub struct GameOverMenu {
    items: Vec<MenuItem>,
    selected_index: usize,
}

impl GameOverMenu {
    pub fn new() -> Self {
        Self {
            items: vec![
                MenuItem::new("Restart", MenuAction::Restart, 180.0),
                MenuItem::new("Exit", MenuAction::Exit, 204.0),
            ],
            selected_index: 0,
        }
    }

    pub fn update(&mut self, mouse_pos: Vec2) -> Option<MenuAction> {
        for (index, item) in self.items.iter().enumerate() {
            if item.rect.contains(mouse_pos) {
                self.selected_index = index;
                if is_mouse_button_pressed(MouseButton::Left) {
                    return Some(item.action);
                }
                break;
            }
        }
        if is_key_pressed(KeyCode::Up) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        } else if is_key_pressed(KeyCode::Down) {
            if self.selected_index < self.items.len() - 1 {
                self.selected_index += 1;
            }
        } else if is_key_pressed(KeyCode::Enter) {
            return Some(self.items[self.selected_index].action);
        } else if is_key_pressed(KeyCode::Escape) {
            return Some(MenuAction::Exit);
        }
        None
    }

    pub fn draw(&self) {
        clear_background(BLACK);

        let text_width = measure_text(GAME_OVER_TEXT, None, 32, 1.0).width;
        draw_text(
            GAME_OVER_TEXT,
            (GAME_WIDTH - text_width) / 2.0,
            100.0,
            32.0,
            WHITE,
        );
        for (index, item) in self.items.iter().enumerate() {
            item.draw(index == self.selected_index);
        }
    }
}
