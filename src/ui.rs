use crate::utils::GAME_WIDTH;
use macroquad::prelude::*;

const TOP: f32 = 16.0;
const RECT: Rect = Rect::new(16.0, 80.0, 16.0, 16.0);

pub fn draw(score: i32, lives: i32, texture: &Texture2D) {
    draw_text(&format!("Score: {}", score), 8.0, TOP, 16.0, WHITE);
    if lives > 0 {
        for i in 0..lives {
            draw_texture_ex(
                &texture,
                GAME_WIDTH - (i as f32 * 18.0 + 24.0),
                TOP - 12.0,
                WHITE,
                DrawTextureParams {
                    source: Some(RECT),
                    ..Default::default()
                },
            );
        }
    }
}

#[derive(Clone, Copy)]
pub enum MenuAction {
    Resume,
    Restart,
    Exit,
}

pub struct MenuItem {
    text: String,
    pub action: MenuAction,
    pos: Vec2,
    pub rect: Rect,
}

impl MenuItem {
    pub fn new(text: &str, action: MenuAction, y: f32) -> Self {
        let text_size = measure_text(text, None, 16, 1.0);
        let pos = Vec2::new((GAME_WIDTH - text_size.width) / 2.0, y);
        Self {
            text: text.to_string(),
            action,
            pos,
            rect: Rect::new(
                pos.x,
                pos.y - text_size.offset_y,
                text_size.width,
                text_size.height,
            ),
        }
    }

    pub fn draw(&self, is_selected: bool) {
        draw_text(
            &self.text,
            self.pos.x,
            self.pos.y,
            16.0,
            if is_selected { YELLOW } else { WHITE },
        );
        // draw_rectangle_lines(
        //     self.rect.x,
        //     self.rect.y,
        //     self.rect.w,
        //     self.rect.h,
        //     1.0,
        //     GREEN,
        // );
    }
}
