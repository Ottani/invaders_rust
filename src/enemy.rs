use crate::utils::lerp;
use macroquad::prelude::*;

const TEX_RECT: Rect = Rect::new(0.0, 96.0, 32.0, 32.0);

#[derive(Clone, Copy)]
pub enum EnemyType {
    Weak,
    Normal,
    Strong,
    DieHard,
}

impl EnemyType {
    const fn life(&self) -> i32 {
        match self {
            EnemyType::Weak => 1,
            EnemyType::Normal => 2,
            EnemyType::Strong => 4,
            EnemyType::DieHard => 6,
        }
    }

    const fn base_color(&self) -> Color {
        match self {
            EnemyType::Weak => GREEN,
            EnemyType::Normal => YELLOW,
            EnemyType::Strong => ORANGE,
            EnemyType::DieHard => RED,
        }
    }
}

pub struct Enemy {
    pub position: Rect,
    prev_pos: Rect,
    life: i32,
    enemy_type: EnemyType,
}

impl Enemy {
    pub const fn new(point: Vec2, enemy_type: EnemyType) -> Enemy {
        let rect: Rect = Rect::new(point.x, point.y, 32.0, 32.0);
        Enemy {
            position: rect,
            prev_pos: rect,
            life: enemy_type.life(),
            enemy_type,
        }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.life -= amount;
    }

    pub const fn is_dead(&self) -> bool {
        self.life <= 0
    }

    pub const fn score(&self) -> i32 {
        self.enemy_type.life() * 10
    }

    pub fn update_position(&mut self, movement: Vec2) {
        self.prev_pos = self.position;
        self.position.x += movement.x;
        self.position.y += movement.y;
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        draw_texture_ex(
            &texture,
            lerp(self.prev_pos.x, self.position.x, alpha).floor(),
            lerp(self.prev_pos.y, self.position.y, alpha).floor(),
            self.enemy_type.base_color(),
            DrawTextureParams {
                source: Some(TEX_RECT),
                ..Default::default()
            },
        );
    }
}
