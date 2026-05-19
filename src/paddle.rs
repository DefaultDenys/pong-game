use macroquad::{color::WHITE, math::Rect, shapes::draw_rectangle};

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

    pub fn move_up(&mut self) {
        self.y -= 5.0;
    }

    pub fn move_down(&mut self) {
        self.y += 5.0;
    }

    pub fn get_bounds(&self) -> Rect {
        Rect::new(self.x, self.y, self.width, self.height)
    }
}
