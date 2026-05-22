use macroquad::{color::WHITE, shapes::draw_line, window::screen_height};

pub struct DottedLine {
    x: f32,
    segment_length: f32,
    gap_length: f32,
}

impl DottedLine {
    pub fn new(x: f32, segment_length: f32, gap_length: f32) -> Self {
        Self {
            x,
            segment_length,
            gap_length,
        }
    }

    pub fn draw(&self) {
        let mut y = 0.0;
        while y < screen_height() {
            draw_line(
                self.x,
                y,
                self.x,
                (y + self.segment_length).min(screen_height()),
                1.0,
                WHITE,
            );
            y += self.segment_length + self.gap_length;
        }
    }

    pub fn update_if_screen_resize(&mut self, new_x: f32) {
        self.x = new_x;
    }
}
