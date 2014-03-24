extern crate rand;
mod ai;
mod game;

fn main()
{
    let mut game = game::Game::new();

    game::Game::new();
    //game.run(ai::get_random_vec);
    game.run(ai::get_best_vec);
    game.print();
}
