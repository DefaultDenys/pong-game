use macroquad::{
    color::WHITE,
    math::Circle,
    rand,
    shapes::draw_circle,
    window::{screen_height, screen_width},
};

use crate::paddle::Paddle;

pub enum ScoreEvent {
    LeftPlayerScored,
    RightPlayerScored,
}

pub struct Ball {
    bounds: Circle,
    velocity_x: f32,
    velocity_y: f32,
    reset_timer: f32,
}

impl Ball {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            bounds: Circle::new(x, y, 10.0),
            velocity_x: 150.0,
            velocity_y: 150.0,
            reset_timer: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.bounds.x = screen_width() / 2.0;
        self.bounds.y = screen_height() / 2.0;
        let speed = 150.0;
        self.velocity_x = if rand::gen_range(0, 2) == 0 {
            speed
        } else {
            -speed
        };
        self.velocity_y = if rand::gen_range(0, 2) == 0 {
            speed
        } else {
            -speed
        };
        self.reset_timer = 1.0; // 1 second delay before the ball starts moving again
    }

    pub fn bounds(&self) -> &Circle {
        &self.bounds
    }

    pub fn draw(&self) {
        draw_circle(self.bounds.x, self.bounds.y, self.bounds.r, WHITE);
    }

    pub fn update(&mut self, delta_time: f32) -> Option<ScoreEvent> {
        if self.reset_timer > 0.0 {
            self.reset_timer -= delta_time;
            return None;
        }

        self.bounds.x += self.velocity_x * delta_time;
        self.bounds.y += self.velocity_y * delta_time;

        // Check for collision with top and bottom walls
        if self.bounds.y - self.bounds.r <= 0.0 || self.bounds.y + self.bounds.r >= screen_height()
        {
            self.velocity_y = -self.velocity_y;
        }

        // Check for scoring
        if self.bounds.x + self.bounds.r < 0.0 {
            return Some(ScoreEvent::RightPlayerScored);
        }
        if self.bounds.x - self.bounds.r > screen_width() {
            return Some(ScoreEvent::LeftPlayerScored);
        }

        None
    }

    pub fn check_paddle_collision(&mut self, paddle: &Paddle) {
        if self.bounds().overlaps_rect(&paddle.bounds()) {
            self.velocity_x = -self.velocity_x;
        }
    }
}
