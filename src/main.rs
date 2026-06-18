mod bomb;
mod bullet;
mod enemy;
mod game_state;
mod player;
mod rock;
mod utils;
use crate::game_state::GameState;
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

    let render_target = render_target(utils::GAME_WIDTH as u32, utils::GAME_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let world = Rect::new(0.0, 0.0, utils::GAME_WIDTH, utils::GAME_HEIGHT);
    let mut virtual_camera = Camera2D::from_display_rect(world);
    virtual_camera.render_target = Some(render_target.clone());

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

        game_state.handle_input();
        game_state.update_animations(frame_time);

        while accumulator >= DT {
            game_state.update_physics(DT, world);
            accumulator -= DT;
        }
        let alpha = accumulator / DT;

        set_camera(&virtual_camera);
        clear_background(DARKERGRAY);

        game_state.draw(alpha, &sheet);

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
