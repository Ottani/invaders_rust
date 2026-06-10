use crate::utils::lerp;
use macroquad::prelude::*;

const EMPTY_RECT: Rect = Rect::new(f32::MAX, f32::MAX, 0.0, 0.0);
const DOWNWARDS: f32 = 16.0 * 60.0;
const TEX_RECT: Rect = Rect::new(0.0, 96.0, 32.0, 32.0);

#[derive(Clone, Copy)]
enum EnemyType {
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
            EnemyType::DieHard => 8,
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

pub struct EnemyManager {
    enemies: Vec<Enemy>,
    rect: Rect,
    speed: f32,
}

impl EnemyManager {
    pub const fn new() -> Self {
        EnemyManager {
            enemies: Vec::new(),
            rect: EMPTY_RECT,
            speed: 50.0,
        }
    }

    pub fn create_enemies(&mut self) {
        let cols = 10;
        let rows = 5;
        let gap = 8.0;
        for y in 0..rows {
            let enemy_type = match y {
                0 => EnemyType::DieHard,
                1 => EnemyType::Strong,
                2 => EnemyType::Normal,
                _ => EnemyType::Weak,
            };
            for x in 0..cols {
                let point = vec2(x as f32 * 32.0 + gap * x as f32, y as f32 * 32.0);
                self.enemies.push(Enemy::new(point, enemy_type));
            }
        }
    }

    fn calculate_area(&mut self) {
        if self.enemies.is_empty() {
            self.rect = Rect::new(0.0, 0.0, 0.0, 0.0);
            return;
        }
        let first = &self.enemies[0].position;
        let mut min_x = first.x;
        let mut min_y = first.y;
        let mut max_x = first.right();
        let mut max_y = first.bottom();

        for enemy in &self.enemies[1..] {
            min_x = min_x.min(enemy.position.x);
            min_y = min_y.min(enemy.position.y);
            max_x = max_x.max(enemy.position.right());
            max_y = max_y.max(enemy.position.bottom());
        }
        self.rect = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);
    }

    pub fn update_physics(&mut self, delta: f32, world: Rect) {
        self.calculate_area();
        let prev_pos = self.rect.point();
        self.rect.x += self.speed * delta;
        if self.rect.x > world.w - self.rect.w {
            self.rect.x = world.w - self.rect.w;
            self.rect.y += DOWNWARDS * delta;
            self.speed = -self.speed;
        } else if self.rect.x < 0.0 {
            self.rect.x = 0.0;
            self.rect.y += DOWNWARDS * delta;
            self.speed = -self.speed;
        }
        let movement = self.rect.point() - prev_pos;

        for enemy in &mut self.enemies {
            enemy.prev_pos = enemy.position;
            enemy.position.x += movement.x;
            enemy.position.y += movement.y;
        }
    }

    pub fn draw(&self, alpha: f32, sheet: &Texture2D) {
        for enemy in &self.enemies {
            enemy.draw(alpha, sheet);
        }
    }
}

struct Enemy {
    position: Rect,
    prev_pos: Rect,
    life: i32,
    enemy_type: EnemyType,
}

impl Enemy {
    const fn new(point: Vec2, enemy_type: EnemyType) -> Enemy {
        let rect: Rect = Rect::new(point.x, point.y, 32.0, 32.0);
        Enemy {
            position: rect,
            prev_pos: rect,
            life: enemy_type.life(),
            enemy_type,
        }
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
