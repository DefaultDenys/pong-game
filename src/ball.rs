use macroquad::{color::WHITE, math::Circle, shapes::draw_circle, window::screen_height};

use crate::paddle::Paddle;

pub struct Ball {
    bounds: Circle,
    velocity_x: f32,
    velocity_y: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            bounds: Circle::new(x, y, 10.0),
            velocity_x: 150.0,
            velocity_y: 150.0,
        }
    }

    pub fn bounds(&self) -> &Circle {
        &self.bounds
    }

    pub fn draw(&self) {
        draw_circle(self.bounds.x, self.bounds.y, self.bounds.r, WHITE);
    }

    pub fn update(&mut self, delta_time: f32) {
        self.bounds.x += self.velocity_x * delta_time;
        self.bounds.y += self.velocity_y * delta_time;

        if self.bounds.y - self.bounds.r <= 0.0 || self.bounds.y + self.bounds.r >= screen_height()
        {
            self.velocity_y = -self.velocity_y;
        }
    }

    pub fn check_paddle_collision(&mut self, paddle: &Paddle) {
        if self.bounds().overlaps_rect(&paddle.bounds()) {
            self.velocity_x = -self.velocity_x;
        }
    }
}
