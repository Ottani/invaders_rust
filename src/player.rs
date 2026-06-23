use crate::utils::{GAME_WIDTH, lerp};
use macroquad::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Alive,
    Exploding,
    Dead,
}

pub struct Player {
    pub position: Rect,
    prev_position: Rect,
    pub direction: f32,
    state: PlayerState,
    speed: f32,
    rect: Rect,
    explosion_rects: [Rect; 4],
    explosion_animation_timer: f32,
    explosion_frame: usize,
}

impl Player {
    pub fn new(pos_y: f32) -> Self {
        let position = Rect::new((GAME_WIDTH / 2.0) - 16.0, pos_y, 32.0, 32.0);
        Self {
            position,
            prev_position: position,
            direction: 0.0,
            state: PlayerState::Alive,
            speed: 250.0,
            rect: Rect::new(0.0, 0.0, 32.0, 32.0),
            explosion_rects: [
                Rect::new(0.0, 144.0, 32.0, 32.0),
                Rect::new(32.0, 144.0, 32.0, 32.0),
                Rect::new(64.0, 144.0, 32.0, 32.0),
                Rect::new(96.0, 144.0, 32.0, 32.0),
            ],
            explosion_animation_timer: 0.0,
            explosion_frame: 0,
        }
    }

    pub fn reset(&mut self, pos_y: f32) {
        self.position = Rect::new((GAME_WIDTH / 2.0) - 16.0, pos_y, 32.0, 32.0);
        self.direction = 0.0;
        self.prev_position = self.position;
        self.state = PlayerState::Alive;
        self.explosion_animation_timer = 0.0;
        self.explosion_frame = 0;
    }

    pub fn hitboxes(&self) -> [Rect; 2] {
        [
            Rect::new(self.position.x + 10.0, self.position.y + 4.0, 12.0, 16.0),
            Rect::new(self.position.x, self.position.y + 20.0, 32.0, 10.0),
        ]
    }

    pub fn check_collision(&self, projectile_rect: &Rect) -> bool {
        self.hitboxes()
            .iter()
            .any(|box_segment| box_segment.overlaps(projectile_rect))
    }

    pub fn is_alive(&self) -> bool {
        self.state == PlayerState::Alive
    }

    pub fn is_dead(&self) -> bool {
        self.state == PlayerState::Dead
    }

    pub fn explode(&mut self) {
        self.state = PlayerState::Exploding;
        self.explosion_animation_timer = 0.0;
        self.explosion_frame = 0;
    }

    pub fn update(&mut self, dt: f32) {
        if self.state == PlayerState::Exploding {
            self.explosion_animation_timer += dt * 12.0;
            if self.explosion_animation_timer >= 4.0 {
                self.state = PlayerState::Dead;
                self.explosion_frame = 3;
            } else {
                self.explosion_frame = self.explosion_animation_timer as usize;
            }
        }
    }

    pub fn update_physics(&mut self, dt: f32, world: Rect) {
        if self.state == PlayerState::Alive {
            self.prev_position = self.position;
            self.position.x += self.direction * self.speed * dt;
            if self.position.x < 0.0 {
                self.position.x = 0.0;
            } else if self.position.x > world.w - 32.0 {
                self.position.x = world.w - 32.0;
            }
        }
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        match self.state {
            PlayerState::Alive => {
                draw_texture_ex(
                    &texture,
                    lerp(self.prev_position.x, self.position.x, alpha).floor(),
                    lerp(self.prev_position.y, self.position.y, alpha).floor(),
                    WHITE,
                    DrawTextureParams {
                        source: Some(self.rect),
                        ..Default::default()
                    },
                );
            }
            PlayerState::Exploding => {
                draw_texture_ex(
                    &texture,
                    lerp(self.prev_position.x, self.position.x, alpha).floor(),
                    lerp(self.prev_position.y, self.position.y, alpha).floor(),
                    WHITE,
                    DrawTextureParams {
                        source: Some(self.explosion_rects[self.explosion_frame]),
                        ..Default::default()
                    },
                );
            }
            PlayerState::Dead => {}
        }
    }
}
