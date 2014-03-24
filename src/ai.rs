use std::num::abs;
use rand::random;
use game::Game;

pub fn get_random_vec(game: &Game, move: ~[int]) -> (int, int)
{
    Game::int_to_vec(move[(abs(random::<int>()) % (move.len() as int))])
}

/* Minimax with depht 1, I think */
pub fn get_best_vec(game: &Game, move: ~[int]) -> (int, int)
{
    let &mut cpy: &Game;
    let mut tab: [int, ..4] = [0, ..4];
    let mut vec_i = 0;

    for &i in move.iter()
    {
        let (a, b, c) = (game.score, game.move_nb, game.merged_nb);

        cpy = *game;
        cpy.move(Game::int_to_vec(i));

        let (a2, b2, c2) = (cpy.score-a, cpy.move_nb-b, cpy.merged_nb-c);
        tab[i] = (a2*c2)/(b2);

        if tab[vec_i] <= tab[i]
        {
            vec_i = i;
        }
    }

    Game::int_to_vec(vec_i)
}

