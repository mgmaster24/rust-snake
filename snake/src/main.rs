mod command;
mod direction;
mod game;
mod point;
mod render;
mod snake;

fn main() {
    let game = game::Game {};

    game.run()
}
