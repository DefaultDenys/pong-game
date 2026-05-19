use macroquad::{color::WHITE, shapes::draw_circle};

pub struct Ball {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
    radius: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            velocity_x: 4.0,
            velocity_y: 4.0,
            radius: 10.0,
        }
    }

    pub fn draw(&self) {
        draw_circle(self.x, self.y, self.radius, WHITE);
    }

    pub fn update(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }
}
