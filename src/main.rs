mod game;
mod trick;
mod utils;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
