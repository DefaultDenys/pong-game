use macroquad::{color::WHITE, shapes::draw_circle, window::screen_height};

use crate::paddle::Paddle;

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

        if self.y - self.radius <= 0.0 || self.y + self.radius >= screen_height() {
            self.velocity_y = -self.velocity_y;
        }
    }

    pub fn check_paddle_collision(&mut self, paddle: &Paddle) {
        let paddle_bounds = paddle.get_bounds();
        if self.x - self.radius <= paddle_bounds.x + paddle_bounds.w
            && self.x + self.radius >= paddle_bounds.x
            && self.y >= paddle_bounds.y
            && self.y <= paddle_bounds.y + paddle_bounds.h
        {
            self.velocity_x = -self.velocity_x;
        }
    }
}
