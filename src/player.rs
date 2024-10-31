use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 300.0;

pub struct Player {
    pub rect: Rect,
    pub xvel: f32,
    pub yvel: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0,
                screen_height() / 2.0,
                20.0,
                20.0,
            ),
            xvel: 0.0,
            yvel: 0.0,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GREEN);
    }
    
    pub fn update(&mut self, dt: f32) {
        if is_key_down(KeyCode::D) { self.xvel += 1.0 }
        if is_key_down(KeyCode::A) { self.xvel -= 1.0 }
        if is_key_down(KeyCode::W) { self.yvel -= 1.0 }
        if is_key_down(KeyCode::S) { self.yvel += 1.0 }
        
        self.rect.x += self.xvel * dt * PLAYER_SPEED;
        self.rect.y += self.yvel * dt * PLAYER_SPEED;
    }
}

