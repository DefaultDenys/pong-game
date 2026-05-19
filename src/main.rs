use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    loop {
        clear_background(BLACK);
        // Game logic and rendering code would go here

        next_frame().await;
    }
}
