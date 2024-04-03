// examples/demo.rs

use carnot::App;

#[tokio::main]
async fn main() {
    let app = App::new(800, 600, "Carnot Demo").await;
    app.run();
}
