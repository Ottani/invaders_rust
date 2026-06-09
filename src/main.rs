mod entity;
use entity::Entity;
use macroquad::prelude::*;

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

fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    start + (end - start) * amount
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
    let virtual_width = 640.0;
    let virtual_height = 360.0;
    const QTY: usize = 500;
    const TEXTURES: usize = 5;

    let sheet: Texture2D = load_texture("assets/sheet01.png")
        .await
        .unwrap_or_else(|_| {
            println!("Failed to load sheet01");
            Texture2D::empty()
        });
    let sprite_balls: Texture2D =
        load_texture("assets/spritesheet.png")
            .await
            .unwrap_or_else(|_| {
                println!("Failed to load sheet01");
                Texture2D::empty()
            });

    let pos = vec2((virtual_width / 2.0) - 16.0, virtual_height - 32.0 - 16.0);
    let mut player = Player {
        position: pos,
        prev_position: pos,
        direction: vec2(0.0, 0.0),
        speed: 250.0,
        rect: Rect::new(0.0, 0.0, 32.0, 32.0),
    };

    let ball_rects: Vec<Rect> = (0..TEXTURES)
        .map(|i| Rect::new(i as f32 * 16.0, 0.0, 16.0, 16.0))
        .collect();

    let mut balls: Vec<Entity> = Vec::with_capacity(QTY);

    for _ in 0..QTY {
        let pos = vec2(
            rand::gen_range(0.0, virtual_width - 16.0),
            rand::gen_range(0.0, virtual_height - 16.0),
        );
        let speed = vec2(rand::gen_range(0.0, 1.0), rand::gen_range(0.0, 1.0)).normalize()
            * rand::gen_range(-50.0, 50.0);
        let damp = rand::gen_range(-0.99, -0.8);
        let tex_rect = ball_rects[rand::gen_range(0, TEXTURES)];
        balls.push(Entity::new(pos, speed, damp, tex_rect));
    }

    let render_target = render_target(virtual_width as u32, virtual_height as u32);
    render_target.texture.set_filter(FilterMode::Nearest);
    let world = Rect::new(0.0, 0.0, virtual_width, virtual_height);
    let mut virtual_camera = Camera2D::from_display_rect(world);
    virtual_camera.render_target = Some(render_target.clone());

    const DT: f32 = 1.0 / 60.0;
    const G: f32 = 9.8 * 60.0;
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

        while accumulator >= DT {
            player.prev_position = player.position;
            player.position += player.direction * player.speed * DT;
            if player.position.x < 0.0 {
                player.position.x = 0.0;
            } else if player.position.x > virtual_width - 32.0 {
                player.position.x = virtual_width - 32.0;
            }

            for ball in &mut balls {
                ball.update_physics(DT, G, world);
            }
            accumulator -= DT;
        }
        let alpha = accumulator / DT;

        set_camera(&virtual_camera);
        clear_background(DARKGRAY);

        for ball in &balls {
            draw_texture_ex(
                &sprite_balls,
                lerp(ball.prev_pos.x, ball.position.x, alpha).floor(),
                lerp(ball.prev_pos.y, ball.position.y, alpha).floor(),
                WHITE,
                DrawTextureParams {
                    source: Some(ball.tex_rect),
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            &sheet,
            lerp(player.prev_position.x, player.position.x, alpha).floor(),
            lerp(player.prev_position.y, player.position.y, alpha).floor(),
            WHITE,
            DrawTextureParams {
                source: Some(player.rect),
                ..Default::default()
            },
        );
        draw_text("Hello, Macroquad!", 20.0, 20.0, 30.0, WHITE);

        set_default_camera();
        clear_background(BLACK);

        let scale = (screen_width() / virtual_width)
            .min(screen_height() / virtual_height)
            .floor()
            .max(1.0);

        let dest_w = virtual_width * scale;
        let dest_h = virtual_height * scale;
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
