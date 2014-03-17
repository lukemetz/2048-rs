use std::num::abs;
use rand::random;
use game::Game;

pub fn get_vec(game: &Game) -> (int, int)
{
    Game::int_to_vec(abs(random::<int>())%4)
}
