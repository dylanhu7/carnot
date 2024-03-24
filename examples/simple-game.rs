// examples/simple_game.rs

use carnot::GameEngine;

#[tokio::main]
async fn main() {
    let mut game_engine = GameEngine::new(800, 600, "Simple Game").await;
    game_engine.run();
}
