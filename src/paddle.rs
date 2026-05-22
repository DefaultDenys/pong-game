use macroquad::{
    color::WHITE,
    input::{KeyCode, is_key_down},
    math::Rect,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

pub struct ControlKeys {
    pub up: KeyCode,
    pub down: KeyCode,
}
pub struct Paddle {
    bounds: Rect,
    control_keys: ControlKeys,
}
impl Paddle {
    fn move_up(&mut self, delta_time: f32) {
        self.bounds.y -= 300.0 * delta_time;
    }

    fn move_down(&mut self, delta_time: f32) {
        self.bounds.y += 300.0 * delta_time;
    }

    pub fn new(x: f32, y: f32, control_keys: ControlKeys) -> Self {
        Self {
            bounds: Rect::new(x, y, 15.0, 80.0),
            control_keys,
        }
    }

    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.bounds.x,
            self.bounds.y,
            self.bounds.w,
            self.bounds.h,
            WHITE,
        );
    }

    pub fn update(&mut self, delta_time: f32) {
        if is_key_down(self.control_keys.up) {
            self.move_up(delta_time);
        }

        if is_key_down(self.control_keys.down) {
            self.move_down(delta_time);
        }

        self.bounds.y = self.bounds.y.clamp(0.0, screen_height() - self.bounds.h);
    }

    pub fn update_if_screen_resize(&mut self, move_x: f32) {
        self.bounds.x = move_x;
        self.bounds.x = self.bounds.x.clamp(0.0, screen_width() - self.bounds.w);
    }
}
