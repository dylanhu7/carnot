// examples/simple_game.rs

use carnot::App;

#[tokio::main]
async fn main() {
    let mut game_engine = App::new(800, 600, "Carnot Demo").await;
    game_engine.run();
}
