use rand::random;
use game::Game;

static DEPTH : int = 7;
static INF : int = 100000;

pub fn get_random_vec(game: &Game, move: ~[int]) -> (int, int)
{
    Game::int_to_vec(move[random::<uint>() % move.len()])
}

pub fn Minimax(game: &Game, move: ~[int]) -> (int, int)
{
    let mut cpy: Game;
    let mut best_move = (0, 0);
    let mut val_max = -INF;
    let mut val;

    for &i in move.iter()
    {
        cpy = game.clone();
        cpy.move(Game::int_to_vec(i));
        val = min(&cpy, cpy.list_move(), DEPTH-1);

        if val > val_max
        {
            val_max = val;
            best_move = Game::int_to_vec(i);
        }
    }

    best_move
}

fn min(game: &Game, move: ~[int], depth: int) -> int
{
    if depth == 0 || move.len() == 0
    {
        return eval(game, move.len() == 0);
    }

    let mut cpy: Game;
    let mut val_min = INF;
    let mut val;
    let empty = game.clone().list_tile_empty();

    for i in range(0, empty.len())
    {
        let (a, b) = empty[i as uint];
        cpy = game.clone();
        cpy.grid[a as uint][b as uint] = 2;

        val = max(&cpy, cpy.list_move(), depth-1);

        if val < val_min
        {
            val_min = val;
        }
    }

    val_min
}

fn max(game: &Game, move: ~[int], depth: int) -> int
{
    if depth == 0 || move.len() == 0
    {
        return eval(game, move.len() == 0);
    }

    let mut cpy: Game;
    let mut val_max = -INF;
    let mut val;

    for &i in move.iter()
    {
        cpy = game.clone();
        cpy.move(Game::int_to_vec(i));

        val = min(&cpy, cpy.list_move(), depth-1);

        if val > val_max
        {
            val_max = val;
        }
    }

    val_max
}

fn eval(game: &Game, lost: bool) -> int
{
    if lost == true
    {
        return -INF + (game.score * game.merged_nb) / game.move_nb
    }

    (game.score * game.merged_nb) / game.move_nb
}
