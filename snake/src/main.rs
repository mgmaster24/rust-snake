mod command;
mod direction;
mod game;
mod point;
mod render;
mod snake;

fn main() {
    let mut game = game::Game::new(25, 15);
    game.run();
}
