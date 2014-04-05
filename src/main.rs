extern crate rand;
mod ai;
mod game;

fn main()
{
    let mut game = game::Game::new();

    game::Game::new();
    game.run(ai::Minimax);
    game.print();
}
