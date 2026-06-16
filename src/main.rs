mod bomb;
mod bullets;
mod enemies;
mod utils;
use macroquad::prelude::*;
mod defenses;

use crate::bomb::BombManager;
use bullets::BulletManager;
use defenses::DefenseManager;
use enemies::EnemyManager;

pub const DARKERGRAY: Color = Color::new(0.15, 0.15, 0.15, 1.00);

fn window_conf() -> Conf {
    Conf {
        window_title: "Invaders!".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true,
        platform: miniquad::conf::Platform {
            webgl_version: miniquad::conf::WebGLVersion::WebGL2,
            ..Default::default()
        },
        ..Default::default()
    }
}

struct Player {
    position: Vec2,
    prev_position: Vec2,
    direction: Vec2,
    speed: f32,
    rect: Rect,
}

fn handle_input(player: &mut Player) {
    let mut direction = vec2(0.0, 0.0);
    if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
        direction.x += 1.0;
    }
    if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
        direction.x -= 1.0;
    }
    player.direction = direction;
}

#[macroquad::main(window_conf())]
async fn main() {
    let sheet: Texture2D = load_texture("assets/sheet01.png")
        .await
        .unwrap_or_else(|_| {
            println!("Failed to load sheet01");
            Texture2D::empty()
        });

    let pos = vec2(
        (utils::GAME_WIDTH / 2.0) - 16.0,
        utils::GAME_HEIGHT - 32.0 - 16.0,
    );
    let mut player = Player {
        position: pos,
        prev_position: pos,
        direction: vec2(0.0, 0.0),
        speed: 250.0,
        rect: Rect::new(0.0, 0.0, 32.0, 32.0),
    };

    let render_target = render_target(utils::GAME_WIDTH as u32, utils::GAME_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let world = Rect::new(0.0, 0.0, utils::GAME_WIDTH, utils::GAME_HEIGHT);
    let mut virtual_camera = Camera2D::from_display_rect(world);
    virtual_camera.render_target = Some(render_target.clone());

    let mut enemy_manager = EnemyManager::new();
    enemy_manager.create_enemies();

    let mut bullet_manager = BulletManager::new();
    let mut bomb_manager = BombManager::new();
    let mut defense_manager = DefenseManager::new();
    defense_manager.create_defenses();

    const DT: f32 = 1.0 / 60.0;
    let mut accumulator = 0.0;
    let mut is_full = false;

    loop {
        let frame_time = get_frame_time().min(0.25);
        accumulator += frame_time;

        if is_key_pressed(KeyCode::F11) {
            is_full = !is_full;
            set_fullscreen(is_full);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        handle_input(&mut player);
        bullet_manager.update(frame_time);

        if is_key_pressed(KeyCode::Space) {
            bullet_manager.create_bullet(vec2(
                player.position.x + player.rect.w / 2.0,
                player.position.y,
            ));
        }

        while accumulator >= DT {
            player.prev_position = player.position;
            player.position += player.direction * player.speed * DT;
            if player.position.x < 0.0 {
                player.position.x = 0.0;
            } else if player.position.x > utils::GAME_WIDTH - 32.0 {
                player.position.x = utils::GAME_WIDTH - 32.0;
            }
            enemy_manager.update_physics(DT, world, &mut bomb_manager);
            bullet_manager.update_physics(DT, world, &mut enemy_manager);
            bomb_manager.update_physics(DT, world);
            accumulator -= DT;
        }
        let alpha = accumulator / DT;

        set_camera(&virtual_camera);
        clear_background(DARKERGRAY);

        bullet_manager.draw(alpha, &sheet);
        bomb_manager.draw(alpha, &sheet);
        defense_manager.draw(&sheet);

        draw_texture_ex(
            &sheet,
            utils::lerp(player.prev_position.x, player.position.x, alpha).floor(),
            utils::lerp(player.prev_position.y, player.position.y, alpha).floor(),
            WHITE,
            DrawTextureParams {
                source: Some(player.rect),
                ..Default::default()
            },
        );

        enemy_manager.draw(alpha, &sheet);

        draw_text("Hello, Macroquad!", 20.0, 20.0, 16.0, WHITE);

        set_default_camera();
        clear_background(BLACK);

        let scale = (screen_width() / utils::GAME_WIDTH)
            .min(screen_height() / utils::GAME_HEIGHT)
            .floor()
            .max(1.0);

        let dest_w = utils::GAME_WIDTH * scale;
        let dest_h = utils::GAME_HEIGHT * scale;
        let x_offset = (screen_width() - dest_w) / 2.0;
        let y_offset = (screen_height() - dest_h) / 2.0;

        draw_texture_ex(
            &render_target.texture,
            x_offset,
            y_offset,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dest_w, dest_h)),
                flip_y: true,
                ..Default::default()
            },
        );

        next_frame().await;
    }
}
