use macroquad::prelude::*;

use paddle::Paddle;

mod paddle;

#[macroquad::main("Pong")]
async fn main() {
    let paddle = Paddle::new(20.0, 150.0);

    loop {
        clear_background(BLACK);
        // Game logic and rendering code would go here

        paddle.draw();

        next_frame().await;
    }
}
