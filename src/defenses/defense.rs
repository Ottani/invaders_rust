use macroquad::prelude::*;

const DEFENSE_MAX_HEALTH: i32 = 10;
const TEX_RECT: Rect = Rect::new(0.0, 32.0, 48.0, 48.0);

const DEFENSE_SIZE: f32 = 48.0;

pub struct Defense {
    max_health: i32,
    health: i32,
    position: Rect,
}

impl Defense {
    pub fn new(position: Vec2) -> Self {
        Self {
            max_health: DEFENSE_MAX_HEALTH,
            health: DEFENSE_MAX_HEALTH,
            position: Rect::new(position.x, position.y, DEFENSE_SIZE, DEFENSE_SIZE),
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        draw_texture_ex(
            texture,
            self.position.x,
            self.position.y,
            WHITE,
            DrawTextureParams {
                source: Some(TEX_RECT),
                ..Default::default()
            },
        );
    }
}
