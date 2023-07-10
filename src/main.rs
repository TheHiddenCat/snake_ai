mod window;
mod snake;
mod apple;
mod game;
mod settings;
mod direction;
mod population;
mod brain;

use crate::game::Game;
use crate::window::config;

#[macroquad::main(config)]
async fn main() {
    let game = Game::new();
    game.run().await;
}