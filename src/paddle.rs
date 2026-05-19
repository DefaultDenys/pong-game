use macroquad::{color::WHITE, shapes::draw_rectangle};

pub struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Paddle {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            width: 15.0,
            height: 80.0,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }
}
