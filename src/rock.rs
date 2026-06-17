use macroquad::prelude::*;

pub const ROCK_SIZE: f32 = 48.0;
const DATA_SIZE: usize = ROCK_SIZE as usize * ROCK_SIZE as usize;

pub struct Rock {
    position: Rect,
    texture: Texture2D,
    mask: [bool; DATA_SIZE],
}

impl Rock {
    pub fn new(position: Vec2, image: &Image) -> Self {
        let mut mask = [false; DATA_SIZE];
        let size = ROCK_SIZE as u32;
        for i in 0..DATA_SIZE {
            let j = i as u32;
            let pixel = image.get_pixel(j % size, j / size);
            mask[i] = pixel.a > 0.9;
        }
        Self {
            position: Rect::new(position.x, position.y, ROCK_SIZE, ROCK_SIZE),
            texture: Texture2D::from_image(&image),
            mask,
        }
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }
}
