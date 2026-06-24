use macroquad::prelude::*;

pub const ROCK_SIZE: f32 = 48.0;

pub struct Rock {
    pub position: Rect,
    texture: Texture2D,
    image: Image,
    impact: Image,
}

impl Rock {
    pub fn new(position: Vec2, image: Image, impact: Image) -> Self {
        Self {
            position: Rect::new(position.x, position.y, ROCK_SIZE, ROCK_SIZE),
            texture: Texture2D::from_image(&image),
            image,
            impact,
        }
    }

    pub fn draw(&self) {
        draw_texture(&self.texture, self.position.x, self.position.y, WHITE);
    }

    pub fn check_collision(&mut self, other: &Rect) -> bool {
        if !self.position.overlaps(other) {
            return false;
        }
        if let Some(rect) = self.position.intersect(*other) {
            let local_x_start = (rect.x - self.position.x).max(0.0) as u32;
            let local_y_start = (rect.y - self.position.y).max(0.0) as u32;
            let local_x_end = (rect.right() - self.position.x).min(ROCK_SIZE) as u32;
            let local_y_end = (rect.bottom() - self.position.y).min(ROCK_SIZE) as u32;

            for y in local_y_start..local_y_end {
                for x in local_x_start..local_x_end {
                    if self.image.get_pixel(x, y).a > 0.5 {
                        let impact_world_x = rect.x + rect.w / 2.0;
                        let impact_world_y = rect.y + rect.h / 2.0;
                        let local_cx = (impact_world_x - self.position.x) as i32;
                        let local_cy = (impact_world_y - self.position.y) as i32;
                        self.update_image(local_cx, local_cy);
                        return true;
                    }
                }
            }
        }

        false
    }

    fn update_image(&mut self, cx: i32, cy: i32) {
        let start_x = cx - (self.impact.width as i32 / 2);
        let start_y = cy - (self.impact.height as i32 / 2);

        let size = ROCK_SIZE as i32;
        for y in 0..self.impact.height as i32 {
            for x in 0..self.impact.width as i32 {
                let px = start_x + x;
                let py = start_y + y;
                if px >= 0 && px < size && py >= 0 && py < size {
                    let source_pixel = self.impact.get_pixel(x as u32, y as u32);
                    let mut dest_pixel = self.image.get_pixel(px as u32, py as u32);
                    dest_pixel.a = (dest_pixel.a - source_pixel.a).max(0.0);
                    self.image.set_pixel(px as u32, py as u32, dest_pixel);
                }
            }
        }
        self.texture.update(&self.image);
    }
}
