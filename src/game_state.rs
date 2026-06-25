use crate::bomb::Bomb;
use crate::bullet::Bullet;
use crate::enemy::{Enemy, EnemyType};
use crate::game_state::State::{MainMenu, Victory};
use crate::player::Player;
use crate::rock::ROCK_SIZE;
use crate::rock::Rock;
use crate::utils::{GAME_HEIGHT, GAME_WIDTH};
use macroquad::audio::{PlaySoundParams, Sound, load_sound, play_sound, play_sound_once};
use macroquad::{prelude::*, rand::ChooseRandom};

const NUM_ROCKS: usize = 5;
const MAX_BULLETS: usize = 5;
const MAX_BOMBS: usize = 15;
const PLAYER_Y: f32 = GAME_HEIGHT - 32.0 - 16.0;
const ROCKS_Y: f32 = 240.0;
const EMPTY_RECT: Rect = Rect::new(f32::MAX, f32::MAX, 0.0, 0.0);
const ENEMY_DOWNWARDS: f32 = 16.0 * 60.0;
const ENEMY_SHOOT_DELAY: f32 = 0.75;
const MIN_SHOOTERS: usize = 2;
const MAX_SHOOTERS: usize = 6;

#[derive(PartialEq, Eq)]
pub enum State {
    MainMenu,
    Running,
    Paused,
    Exploding,
    Waiting,
    GameOver,
    Victory,
}

pub struct GameState {
    pub state: State,
    player: Player,
    rocks: [Option<Rock>; NUM_ROCKS],
    bullets: [Option<Bullet>; MAX_BULLETS],
    bombs: [Option<Bomb>; MAX_BOMBS],
    enemies: Vec<Enemy>,
    enemy_area_rect: Rect,
    enemy_speed: f32,
    enemy_shoot_delay: f32,
    pub lives: i32,
    pub score: i32,
    laser_sound: Sound,
    explosion_sounds: [Sound; 3],
}

impl GameState {
    pub async fn new(sheet_image: &Image) -> Self {
        let rocks = Self::create_rocks(sheet_image);
        let mut enemies: Vec<Enemy> = Vec::new();
        Self::create_enemies(&mut enemies);
        let laser_sound = load_sound("assets/laser1.wav")
            .await
            .expect("Failed to load laser sound");
        let explosion_sounds = [
            load_sound("assets/explosion1.wav")
                .await
                .expect("Failed to load explosion sound"),
            load_sound("assets/explosion2.wav")
                .await
                .expect("Failed to load explosion sound"),
            load_sound("assets/explosion3.wav")
                .await
                .expect("Failed to load explosion sound"),
        ];
        Self {
            state: MainMenu,
            player: Player::new(PLAYER_Y),
            rocks,
            bullets: Default::default(),
            bombs: Default::default(),
            enemies,
            enemy_area_rect: EMPTY_RECT,
            enemy_speed: 50.0,
            enemy_shoot_delay: 0.0,
            lives: 3,
            score: 0,
            laser_sound,
            explosion_sounds,
        }
    }

    fn create_rocks(sheet_image: &Image) -> [Option<Rock>; NUM_ROCKS] {
        const ROCK_GAP: f32 = 64.0;
        let mut rocks: [Option<Rock>; NUM_ROCKS] = Default::default();
        let start_pos = vec2(
            (GAME_WIDTH - (NUM_ROCKS as f32 * ROCK_SIZE) - (NUM_ROCKS as f32 - 1.0) * ROCK_GAP)
                / 2.0,
            ROCKS_Y,
        );

        for i in 0..NUM_ROCKS {
            let image = sheet_image.sub_image(Rect::new(192.0, 32.0, 48.0, 48.0));
            let impact = sheet_image.sub_image(Rect::new(240.0, 16.0, 16.0, 16.0));
            rocks[i] = Some(Rock::new(
                vec2(start_pos.x + i as f32 * (ROCK_SIZE + ROCK_GAP), start_pos.y),
                image,
                impact,
            ));
        }

        rocks
    }

    fn create_enemies(enemies: &mut Vec<Enemy>) {
        const COLS: usize = 10;
        const ROWS: usize = 5;
        enemies.clear();
        enemies.reserve(COLS * ROWS);
        let gap = 8.0;
        for y in 0..ROWS {
            let enemy_type = match y {
                0 => EnemyType::DieHard,
                1 => EnemyType::Strong,
                2 => EnemyType::Normal,
                _ => EnemyType::Weak,
            };
            for x in 0..COLS {
                let point = vec2(x as f32 * 32.0 + gap * x as f32, (y as f32 * 32.0) + 24.0);
                enemies.push(Enemy::new(point, enemy_type));
            }
        }
    }

    pub fn reset(&mut self, sheet_image: &Image) {
        self.player.reset(PLAYER_Y);
        self.bullets = Default::default();
        self.bombs = Default::default();
        Self::create_enemies(&mut self.enemies);
        self.rocks = Self::create_rocks(sheet_image);
        self.enemy_area_rect = EMPTY_RECT;
        self.enemy_speed = 50.0;
        self.enemy_shoot_delay = 0.0;
        self.lives = 3;
        self.score = 0;
    }

    pub fn handle_input(&mut self) {
        let mut direction = 0.0;
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            direction += 1.0;
        }
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            direction -= 1.0;
        }
        self.player.direction = direction;

        if is_key_pressed(KeyCode::Space)
            || is_key_pressed(KeyCode::W)
            || is_mouse_button_pressed(MouseButton::Left)
        {
            play_sound_once(&self.laser_sound);
            self.create_bullet(vec2(
                self.player.position.x + self.player.position.w / 2.0,
                self.player.position.y,
            ));
        }
    }

    fn create_bullet(&mut self, pos: Vec2) {
        if let Some(slot) = self.bullets.iter_mut().find(|s| s.is_none()) {
            *slot = Some(Bullet::new(pos));
        }
    }

    fn create_bomb(&mut self, pos: Vec2) -> bool {
        if let Some(slot) = self.bombs.iter_mut().find(|s| s.is_none()) {
            *slot = Some(Bomb::new(vec2(pos.x + 4.0, pos.y)));
            return true;
        }
        false
    }

    fn calculate_enemy_rect(&mut self) {
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for enemy in &mut self.enemies {
            min_x = min_x.min(enemy.position.x);
            min_y = min_y.min(enemy.position.y);
            max_x = max_x.max(enemy.position.right());
            max_y = max_y.max(enemy.position.bottom());
        }
        if self.enemies.is_empty() {
            self.enemy_area_rect = Rect::new(0.0, 0.0, 0.0, 0.0);
        } else {
            self.enemy_area_rect = Rect::new(min_x, min_y, max_x - min_x, max_y - min_y);
        }
    }

    fn process_enemy_firing(&mut self, delta: f32) {
        self.enemy_shoot_delay += delta;
        if self.enemy_shoot_delay >= ENEMY_SHOOT_DELAY {
            self.enemy_shoot_delay = 0.0;
            if !self.enemies.is_empty() {
                let mut spawn_positions = Vec::new();

                let qty_enemies = self.enemies.len();
                let shooters = self.enemies.choose_multiple(rand::gen_range(
                    MIN_SHOOTERS.min(qty_enemies),
                    MAX_SHOOTERS.min(qty_enemies) + 1,
                ));
                for shooter in shooters {
                    let shoot_position = vec2(
                        shooter.position.x + shooter.position.w * 0.5f32,
                        shooter.position.y + shooter.position.h,
                    );
                    spawn_positions.push(shoot_position);
                }
                for shoot_position in spawn_positions {
                    if !self.create_bomb(shoot_position) {
                        break;
                    }
                }
            }
        }
    }

    pub fn update_player_only(&mut self, frame_time: f32) {
        self.player.update(frame_time);
    }

    pub fn update_animations(&mut self, frame_time: f32) {
        self.player.update(frame_time);
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                bullet.update(frame_time);
            }
        }
        for slot in self.bombs.iter_mut() {
            if let Some(bomb) = slot {
                bomb.update(frame_time);
            }
        }
    }

    pub fn update_physics(&mut self, delta: f32, world: Rect) {
        self.player.update_physics(delta, world);
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                if !bullet.update_physics(delta, world) {
                    *slot = None;
                }
            }
        }

        self.calculate_enemy_rect();
        let prev_pos = self.enemy_area_rect.point();
        self.enemy_area_rect.x += self.enemy_speed * delta;
        if self.enemy_area_rect.x > world.w - self.enemy_area_rect.w {
            self.enemy_area_rect.x = world.w - self.enemy_area_rect.w;
            self.enemy_area_rect.y += ENEMY_DOWNWARDS * delta;
            self.enemy_speed = -self.enemy_speed;
        } else if self.enemy_area_rect.x < 0.0 {
            self.enemy_area_rect.x = 0.0;
            self.enemy_area_rect.y += ENEMY_DOWNWARDS * delta;
            self.enemy_speed = -self.enemy_speed;
        }
        let movement = self.enemy_area_rect.point() - prev_pos;
        for enemy in &mut self.enemies {
            enemy.update_position(movement);
        }
        if self.enemy_area_rect.bottom() > world.h - 16.0 {
            self.state = State::GameOver;
        }
        self.process_enemy_firing(delta);

        for slot in self.bombs.iter_mut() {
            if let Some(bomb) = slot.as_mut() {
                if !bomb.update_physics(delta, world) {
                    *slot = None;
                }
            }
        }

        self.resolve_collisions();
    }

    fn play_explosion_sound(&mut self) {
        play_sound(
            &self.explosion_sounds[macroquad::rand::gen_range(0, self.explosion_sounds.len())],
            PlaySoundParams {
                looped: false,
                volume: 0.3,
            },
        );
    }

    fn resolve_collisions(&mut self) {
        let mut bullet_hit = false;
        for slot in self.bullets.iter_mut() {
            if let Some(bullet) = slot {
                let mut hit_enemy = false;
                let mut hit_rock = false;
                for some_rock in self.rocks.iter_mut() {
                    if let Some(rock) = some_rock {
                        if rock.check_collision(&bullet.position) {
                            hit_rock = true;
                            break;
                        }
                    }
                }
                if !hit_rock {
                    for enemy in &mut self.enemies {
                        if !enemy.is_dead() && bullet.position.overlaps(&enemy.position) {
                            enemy.take_damage(1);
                            self.score += enemy.score();
                            hit_enemy = true;
                            break;
                        }
                    }
                }
                if hit_enemy || hit_rock {
                    *slot = None;
                    bullet_hit = true;
                }
            }
        }
        if bullet_hit {
            self.play_explosion_sound();
        }
        self.enemies.retain(|enemy| !enemy.is_dead());
        if self.enemies.is_empty() {
            self.state = Victory;
        }

        for slot in self.bombs.iter_mut() {
            if let Some(bomb) = slot {
                if bomb.position.y >= ROCKS_Y - bomb.position.h {
                    for some_rock in self.rocks.iter_mut() {
                        if let Some(rock) = some_rock {
                            if rock.check_collision(&bomb.position) {
                                *slot = None;
                                break;
                            }
                        }
                    }
                }
            }
            if let Some(bomb) = slot {
                if bomb.position.y >= PLAYER_Y - bomb.position.h && self.player.is_alive() {
                    if self.player.check_collision(&bomb.position) {
                        self.player.explode();
                        *slot = None;
                        self.update_lives();
                        self.play_explosion_sound();
                        break;
                    }
                }
            }
        }

        for enemy in &self.enemies {
            if !enemy.is_dead() {
                for some_rock in self.rocks.iter_mut() {
                    if let Some(rock) = some_rock {
                        if rock.position.overlaps(&enemy.position) {
                            *some_rock = None;
                            break;
                        }
                    }
                }
                if self.player.check_collision(&enemy.position) {
                    self.player.explode();
                    self.update_lives();
                    break;
                }
            }
        }
    }

    fn update_lives(&mut self) {
        self.lives -= 1;
        if self.lives < 0 {
            self.state = State::GameOver;
        } else {
            self.state = State::Exploding;
        }
    }

    pub fn explosion_complete(&mut self) -> bool {
        self.player.is_dead()
    }

    pub fn player_reset(&mut self) {
        self.player.reset(PLAYER_Y);
    }

    pub fn draw(&self, alpha: f32, texture: &Texture2D) {
        self.player.draw(alpha, texture);

        for rock_slot in &self.rocks {
            if let Some(rock) = rock_slot {
                rock.draw();
            }
        }

        for enemy in &self.enemies {
            enemy.draw(alpha, texture);
        }

        for slot in self.bullets.iter() {
            if let Some(bullet) = slot {
                bullet.draw(alpha, texture);
            }
        }

        for slot in self.bombs.iter() {
            if let Some(bomb) = slot {
                bomb.draw(alpha, texture);
            }
        }
    }
}
