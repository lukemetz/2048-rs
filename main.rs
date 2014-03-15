extern mod extra;
use std::iter::range_step;
use std::cmp::max;
use std::rand;
use std::num::abs;
use extra::time::precise_time_s;

static WIDTH  : int = 4;
static HEIGHT : int = 4;

struct Game
{
    grid: [[int, ..WIDTH], ..HEIGHT],
    score: int
}

impl Game
{
    fn new() -> Game
    {
        Game
        {
            grid: [[0, ..WIDTH], ..HEIGHT],
            score: 0
        }
    }

    fn print(self)
    {
        println!("Score : {}", self.score);
        for j in range(0, HEIGHT)
        {
            for i in range(0, WIDTH)
            {
                print!("{} ", self.grid[i][j]);
            }
            println("");
        }
    }

    fn get_vec(dir: int) -> (int, int) /* x, y */
    {
        match dir
        {
            0 => (1, 0),    /* RIGHT */
            1 => (-1, 0),   /* LEFT */
            2 => (0, 1),    /* DOWN */
            3 => (0, -1),   /* UP */
            _ => (42, 42)   /* Wait what ERROR */
        }
    }

    /* 0022 -> 1 | 0222 -> 2 | 2222 -> 3 */
    fn get_lenght(self, vec: (int, int), i: int, j: int) -> int
    {
        let (x, y) = vec;
        let mut c = 1;

        while i+x*c >= 0 && j+y*c >= 0 && i+x*c < WIDTH && j+y*c < HEIGHT &&
              self.grid[i][j] == self.grid[i+x*c][j+y*c]
        {
            c+=1;
        }

        c-1
    }

    fn move_global(&mut self, vec: (int, int)) /* Move without merge */
    {
        let (x, y) = vec;

        for _ in range(0, max(WIDTH, HEIGHT)/2) /* "Move enough times to move everything" */
        {
            /* WIDTH-1 to 0 if x<0, 0 to WIDTH-1 if x>=0 */
            let mut w = if x < 0 {range_step(WIDTH-1, -1, -1)} else {range_step(0, WIDTH, 1)};
            for i in w
            {
                /* HEIGHT-1 to 0 if x<0, 0 to HEIGHT-1 if x>=0 */
                let mut h = if y < 0 {range_step(HEIGHT-1, -1, -1)} else {range_step(0, HEIGHT, 1)};
                for j in h
                {
                    /* If the current tile is full and the next is empty : swap */
                    if i+x >= 0 && j+y >= 0 && i+x < WIDTH && j+y < HEIGHT &&
                       self.grid[i][j] != 0 && self.grid[i+x][j+y] == 0
                    {
                        let tmp = self.grid[i+x][j+y];
                        self.grid[i+x][j+y] = self.grid[i][j];
                        self.grid[i][j] = tmp;
                    }
                }
            }
        }
    }

    fn merge(&mut self, vec: (int, int))
    {
        let (x, y) = vec;

        /* WIDTH-1 to 0 if x<0, 0 to WIDTH-1 if x>=0 */
        let mut w = if x < 0 {range_step(WIDTH-1, -1, -1)} else {range_step(0, WIDTH, 1)};
        for i in w
        {
            /* HEIGHT-1 to 0 if x<0, 0 to HEIGHT-1 if x>=0 */
            let mut h = if y < 0 {range_step(HEIGHT-1, -1, -1)} else {range_step(0, HEIGHT, 1)};
            for j in h
            {
                if i+x >= 0 && j+y >= 0 && i+x < WIDTH && j+y < HEIGHT && self.grid[i][j] != 0
                {
                    let l = self.get_lenght(vec, i, j);

                    /* 0022 -> ok (min), 0002 -> lolnope */
                    if l >= 1
                    {
                        /* end of the sequence to the start+1 */
                        for k in range_step(l, 0, -2)
                        {
                            self.grid[i+x*k][j+y*k] += self.grid[i+x*(k-1)][j+y*(k-1)];
                            self.score += self.grid[i+x*k][j+y*k];
                            self.grid[i+x*(k-1)][j+y*(k-1)] = 0;
                        }
                    }
                }
            }
        }
    }

    fn move(&mut self, vec: (int, int))
    {
        self.move_global(vec);
        self.merge(vec);
        self.move_global(vec); /* Plug holes \o/ */
    }

    fn is_moved(g1: Game, g2: Game) -> bool
    {
        for i in range(0, WIDTH)
        {
            for j in range(0, HEIGHT)
            {
                if g1.grid[i][j] != g2.grid[i][j]
                {
                    return true;
                }
            }
        }

        false
    }

    fn is_full(self) -> bool
    {
        for i in range(0, WIDTH)
        {
            for j in range(0, HEIGHT)
            {
                if self.grid[i][j] == 0
                {
                    return false;
                }
            }
        }

        true
    }

    fn is_moveable(self) -> bool
    {
        let mut tmp: Game;
        let mut vec: (int, int);

        /* If there is at least one empty tile */
        if self.is_full() == false
        {
            return true;
        }

        /* Tries to move the grid in each direction, and sees if there have been any changes */
        for i in range(0, 4)
        {
            tmp = self;
            vec = Game::get_vec(i);
            tmp.move(vec);

            if Game::is_moved(self, tmp) == true
            {
                return true;
            }
        }

        false
    }

    fn add_random_tile(&mut self)
    {
        let mut tmp : [(int, int), ..(WIDTH*HEIGHT)] = [(0,0), ..(WIDTH*HEIGHT)];
        let mut n = 0;

        /* List the position of all empty tiles */
        for i in range(0, WIDTH)
        {
            for j in range(0, HEIGHT)
            {
                if self.grid[i][j] == 0
                {
                    tmp[n] = (i, j);
                    n+=1;
                }
            }
        }

        if n != 0
        {
            let (a, b) = tmp[(abs(rand::random::<int>())%n)];
            self.grid[a][b] = 2;
        }
    }

    fn run(&mut self)
    {
        let mut i = 0;

        self.add_random_tile();
        self.add_random_tile();

        while self.is_moveable()
        {
            self.move(Game::get_vec(i%4));
            self.add_random_tile();

            i+=1;
            if i == 1000{break;}
        }

        print!("{}, {}\n", i, self.score);
    }
}

fn main()
{
    let mut game = Game::new();

    let s = precise_time_s();
    game.run();
    let e = precise_time_s();

    print!("{}", e-s);
}
