mod exercise;
mod game;
mod io;
mod statistic;

use game::Game;
use tokio::io::Result;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let address: String = std::env::args().skip(1).take(1).collect();
    let game = Game::new(address.as_str());
    game.run().await
}
