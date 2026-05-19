use macroquad::prelude::*;

use paddle::Paddle;

mod ball;
mod paddle;

#[macroquad::main("Pong")]
async fn main() {
    let mut left_paddle = Paddle::new(
        20.0,
        150.0,
        paddle::ControlKeys {
            up: KeyCode::W,
            down: KeyCode::S,
        },
    );
    let mut right_paddle = Paddle::new(
        750.0,
        150.0,
        paddle::ControlKeys {
            up: KeyCode::Up,
            down: KeyCode::Down,
        },
    );
    let mut ball = ball::Ball::new(400.0, 300.0);
    loop {
        let delta_time = get_frame_time();
        clear_background(BLACK);
        // Game logic and rendering code would go here

        ball.update(delta_time);
        left_paddle.update(delta_time);
        right_paddle.update(delta_time);

        ball.check_paddle_collision(&left_paddle);
        ball.check_paddle_collision(&right_paddle);

        left_paddle.draw();
        right_paddle.draw();
        ball.draw();

        next_frame().await;
    }
}
