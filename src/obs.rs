use macroquad::prelude::*;

pub struct Obs {
    pub rect: Rect,
}

impl Obs {
    pub fn new(xpos:f32, ypos:f32) -> Self {
        Self {
            rect: Rect::new(
                xpos,
                ypos,
                40.0,
                40.0
            ),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }
}
