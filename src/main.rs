use macroquad::prelude::*;

use paddle::Paddle;

use crate::ball::ScoreEvent;

mod ball;
mod dotted_line;
mod paddle;

#[macroquad::main("Pong")]
async fn main() {
    let mut left_paddle = Paddle::new(
        20.0,
        screen_height() / 2.0 - 50.0,
        paddle::ControlKeys {
            up: KeyCode::W,
            down: KeyCode::S,
        },
        RED,
    );
    let mut right_paddle = Paddle::new(
        screen_width() - 35.0,
        screen_height() / 2.0 - 50.0,
        paddle::ControlKeys {
            up: KeyCode::Up,
            down: KeyCode::Down,
        },
        BLUE,
    );
    let mut ball = ball::Ball::new(screen_width() / 2.0, screen_height() / 2.0);
    let mut dotted_line = dotted_line::DottedLine::new(screen_width() / 2.0, 20.0, 20.0);

    let mut left_score: u32 = 0;
    let mut right_score: u32 = 0;

    let mut prev_screen_height = screen_height();
    let mut prev_screen_width = screen_width();

    loop {
        let delta_time = get_frame_time();
        clear_background(BLACK);
        // Game logic and rendering code would go here
        draw_text(
            &left_score.to_string(),
            screen_width() / 4.0,
            50.0,
            64.0,
            left_paddle.color(),
        );
        draw_text(
            &right_score.to_string(),
            3.0 * screen_width() / 4.0,
            50.0,
            64.0,
            right_paddle.color(),
        );

        let score_event = ball.update(delta_time);
        if let Some(score) = score_event {
            match score {
                ScoreEvent::LeftPlayerScored => left_score += 1,
                ScoreEvent::RightPlayerScored => right_score += 1,
            }
            ball.reset();
            println!("Score: {} - {}", left_score, right_score);
        }
        left_paddle.update(delta_time);
        right_paddle.update(delta_time);

        ball.check_paddle_collision(&left_paddle);
        ball.check_paddle_collision(&right_paddle);

        let current_screen_width = screen_width();
        let current_screen_height = screen_height();

        if current_screen_width != prev_screen_width || current_screen_height != prev_screen_height
        {
            left_paddle.update_if_screen_resize(20.0);
            right_paddle.update_if_screen_resize(current_screen_width - 35.0);
            dotted_line.update_if_screen_resize(current_screen_width / 2.0);
            prev_screen_width = current_screen_width;
            prev_screen_height = current_screen_height;
        }

        left_paddle.draw();
        right_paddle.draw();
        ball.draw();
        dotted_line.draw();
        next_frame().await;
    }
}
