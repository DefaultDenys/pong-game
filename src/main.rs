use macroquad::prelude::*;

use paddle::Paddle;

mod ball;
mod paddle;

#[macroquad::main("Pong")]
async fn main() {
    let mut left_paddle = Paddle::new(20.0, 150.0);
    let mut right_paddle = Paddle::new(750.0, 150.0);
    let mut ball = ball::Ball::new(400.0, 300.0);
    loop {
        clear_background(BLACK);
        // Game logic and rendering code would go here

        if is_key_down(KeyCode::W) {
            left_paddle.move_up();
        }

        if is_key_down(KeyCode::S) {
            left_paddle.move_down();
        }

        if is_key_down(KeyCode::Up) {
            right_paddle.move_up();
        }

        if is_key_down(KeyCode::Down) {
            right_paddle.move_down();
        }

        left_paddle.draw();
        right_paddle.draw();
        ball.draw();

        ball.update();

        ball.check_paddle_collision(&left_paddle);
        ball.check_paddle_collision(&right_paddle);

        next_frame().await;
    }
}
