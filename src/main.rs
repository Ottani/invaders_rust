mod bomb;
mod bullet;
mod enemy;
mod game_over;
mod game_state;
mod main_menu;
mod pause_menu;
mod player;
mod rock;
mod ui;
mod utils;
mod victory_screen;
mod wait_screen;
use crate::game_over::GameOverMenu;
use crate::game_state::{GameState, State};
use crate::pause_menu::PauseMenu;
use crate::ui::MenuAction;
use crate::victory_screen::VictoryScreen;

use macroquad::prelude::*;

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

#[macroquad::main(window_conf())]
async fn main() {
    let sheet_image = load_image("assets/sheet01.png")
        .await
        .expect("Failed to load spritesheet!");
    let sheet: Texture2D = Texture2D::from_image(&sheet_image);

    let mut game_state = GameState::new(&sheet_image);
    let mut pause_menu = PauseMenu::new();
    let mut game_over_menu = GameOverMenu::new();
    let mut victory_screen = VictoryScreen::new();

    let render_target = render_target(utils::GAME_WIDTH as u32, utils::GAME_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let world = Rect::new(0.0, 0.0, utils::GAME_WIDTH, utils::GAME_HEIGHT);
    let mut virtual_camera = Camera2D::from_display_rect(world);
    virtual_camera.render_target = Some(render_target.clone());

    const DT: f32 = 1.0 / 60.0;
    let mut accumulator = 0.0;
    let mut is_full = false;
    let mut mouse_pos;

    loop {
        let frame_time = get_frame_time().min(0.25);
        mouse_pos = virtual_camera.screen_to_world(mouse_position().into());

        if is_key_pressed(KeyCode::F11) {
            is_full = !is_full;
            set_fullscreen(is_full);
        }

        match game_state.state {
            State::MainMenu => {
                if main_menu::update() {
                    game_state.state = State::Running;
                }
            }
            State::Running => {
                if is_key_pressed(KeyCode::Escape) {
                    pause_menu.reset();
                    game_state.state = State::Paused;
                }
            }
            State::Paused => {
                if let Some(action) = pause_menu.update(mouse_pos) {
                    match action {
                        MenuAction::Resume => {
                            game_state.state = State::Running;
                        }
                        MenuAction::Restart => {
                            game_state.reset(&sheet_image);
                            game_state.state = State::Running;
                        }
                        MenuAction::Exit => {
                            break;
                        }
                    }
                }
            }
            State::Exploding => {
                if game_state.explosion_complete() {
                    game_state.state = State::Waiting;
                }
            }
            State::Waiting => {
                if wait_screen::update() {
                    game_state.player_reset();
                    game_state.state = State::Running;
                }
            }
            State::GameOver => {
                if let Some(action) = game_over_menu.update(mouse_pos) {
                    match action {
                        MenuAction::Restart => {
                            game_state.reset(&sheet_image);
                            game_state.state = State::Running;
                        }
                        MenuAction::Exit => {
                            break;
                        }
                        MenuAction::Resume => {}
                    }
                }
            }
            State::Victory => {
                if let Some(action) = victory_screen.update(mouse_pos) {
                    match action {
                        MenuAction::Restart => {
                            game_state.reset(&sheet_image);
                            game_state.state = State::Running;
                        }
                        MenuAction::Exit => {
                            break;
                        }
                        MenuAction::Resume => {}
                    }
                }
            }
        }

        if game_state.state == State::Exploding {
            game_state.update_player_only(frame_time);
        }

        let alpha = if game_state.state == State::Running {
            accumulator += frame_time;
            game_state.handle_input();
            game_state.update_animations(frame_time);

            while accumulator >= DT {
                game_state.update_physics(DT, world);
                accumulator -= DT;
            }
            accumulator / DT
        } else {
            1.0
        };

        set_camera(&virtual_camera);
        clear_background(DARKERGRAY);

        match game_state.state {
            State::MainMenu => {
                game_state.draw(alpha, &sheet);
                main_menu::draw();
            }
            State::Running => {
                game_state.draw(alpha, &sheet);
            }
            State::Paused => {
                game_state.draw(alpha, &sheet);
                pause_menu.draw();
            }
            State::Exploding => {
                game_state.draw(alpha, &sheet);
            }
            State::Waiting => {
                game_state.draw(alpha, &sheet);
                wait_screen::draw();
            }
            State::GameOver => {
                game_over_menu.draw();
            }
            State::Victory => {
                victory_screen.draw();
            }
        }
        ui::draw(game_state.score, game_state.lives, &sheet);

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
