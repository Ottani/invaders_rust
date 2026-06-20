use crate::utils::GAME_WIDTH;
use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum MenuAction {
    Resume,
    Restart,
    Exit,
}

pub struct MenuItem {
    text: String,
    action: MenuAction,
}

pub struct PauseMenu {
    items: Vec<MenuItem>,
    selected_index: usize,
}

impl PauseMenu {
    pub fn new() -> Self {
        Self {
            items: vec![
                MenuItem {
                    text: "Resume".to_string(),
                    action: MenuAction::Resume,
                },
                MenuItem {
                    text: "Restart".to_string(),
                    action: MenuAction::Restart,
                },
                MenuItem {
                    text: "Exit".to_string(),
                    action: MenuAction::Exit,
                },
            ],
            selected_index: 0,
        }
    }

    pub fn reset(&mut self) {
        self.selected_index = 0;
    }

    pub fn update(&mut self) -> Option<MenuAction> {
        if is_key_pressed(KeyCode::Up) {
            if self.selected_index > 0 {
                self.selected_index -= 1;
            }
        } else if is_key_pressed(KeyCode::Down) {
            if self.selected_index < self.items.len() - 1 {
                self.selected_index += 1;
            }
        } else if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
            return Some(self.items[self.selected_index].action);
        } else if is_key_pressed(KeyCode::Escape) {
            return Some(MenuAction::Resume);
        }
        None
    }

    pub fn draw(&self) {
        for (index, item) in self.items.iter().enumerate() {
            let text_width = measure_text(&item.text, None, 16, 1.0).width;
            draw_text(
                &item.text,
                (GAME_WIDTH - text_width) / 2.0,
                180.0 + index as f32 * 24.0,
                16.0,
                if index == self.selected_index {
                    YELLOW
                } else {
                    WHITE
                },
            );
        }
    }
}
