use std::iter::range_step;
use rand::random;

static WIDTH  : int = 4;
static HEIGHT : int = 4;

pub struct Game
{
    pub grid: [[int, ..WIDTH], ..HEIGHT],
    pub score: int,
    pub move_nb: int,
    pub merged_nb: int,
    pub tile_max: int
}

impl Game
{
    pub fn new() -> Game
    {
        Game
        {
            grid: [[0, ..WIDTH], ..HEIGHT],
            score: 0,
            move_nb: 0,
            merged_nb: 0,
            tile_max: 0,
        }
    }

    pub fn print(&self)
    {
        println!("Moves : {}\nScore : {}\nMerged : {}\nTile max : {}\n",
                  self.move_nb, self.score, self.merged_nb, self.tile_max);

        for j in range(0, HEIGHT)
        {
            for i in range(0, WIDTH)
            {
                print!("{} ", self.grid[i as uint][j as uint]);
            }
            println!("");
        }
        println!("\n");
    }

    pub fn int_to_vec(dir: int) -> (int, int) /* x, y */
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
    pub fn get_lenght(self, vec: (int, int), i: int, j: int) -> int
    {
        let (x, y) = vec;
        let mut c = 1;

        while i+x*c >= 0 && j+y*c >= 0 && i+x*c < WIDTH && j+y*c < HEIGHT &&
              self.grid[i as uint][j as uint] == self.grid[(i+x*c) as uint][(j+y*c) as uint]
        {
            c+=1;
        }

        c
    }

    pub fn move_global(&mut self, vec: (int, int)) /* Move without merge */
    {
        let (x, y) = vec;

        /* if x>=0 : ← | else → */
        let mut w = if x >= 0 {range_step(WIDTH-1, -1, -1)} else {range_step(0, WIDTH, 1)};
        for i in w
        {
            /* if y>=0 : ↑ | else ↓ */
            let mut h = if y >= 0 {range_step(HEIGHT-1, -1, -1)} else {range_step(0, HEIGHT, 1)};
            for j in h
            {
                /* If the current tile is full */
                if self.grid[i as uint][j as uint] != 0
                {
                    let mut i2 = i;
                    let mut j2 = j;

                    /* Find the nearest full tile */
                    while i2+x >= 0 && j2+y >= 0 && i2+x < WIDTH && j2+y < HEIGHT &&
                          self.grid[(i2+x) as uint][(j2+y) as uint] == 0
                    {

                        i2 = i2 + x;
                        j2 = j2 + y;
                    }

                    /* Swap */
                    let tmp = self.grid[i2 as uint][j2 as uint];
                    self.grid[i2 as uint][j2 as uint] = self.grid[i as uint][j as uint];
                    self.grid[i as uint][j as uint] = tmp;
                }
            }
        }
    }

    pub fn merge_seq(&mut self, vec: (int, int), i: int, j: int)
    {
        let l = self.get_lenght(vec, i, j) - 1;
        let (x, y) = vec;

        /* 0022 -> ok (min), 0002 -> lolnope */
        if l >= 1
        {
            /* End of the sequence to the start+1 */
            for k in range_step(l, 0, -2)
            {
                /* If both tiles are equals */
                if self.grid[(i+x*k) as uint][(j+y*k) as uint] == self.grid[(i+x*(k-1)) as uint][(j+y*(k-1)) as uint]
                {
                    self.merged_nb+=1;
                }

                self.grid[(i+x*k) as uint][(j+y*k) as uint] += self.grid[(i+x*(k-1)) as uint][(j+y*(k-1)) as uint];
                self.score += self.grid[(i+x*k) as uint][(j+y*k) as uint];
                self.grid[(i+x*(k-1)) as uint][(j+y*(k-1)) as uint] = 0;

                /* Update the tile max, if needed */
                if self.tile_max < self.grid[(i+x*k) as uint][(j+y*k) as uint]
                {
                    self.tile_max = self.grid[(i+x*k) as uint][(j+y*k) as uint];
                }
            }
        }
    }

    pub fn merge(&mut self, vec: (int, int))
    {
        let (x, y) = vec;

        /* if x>=0 : ← | else → */
        let mut w = if x >= 0 {range_step(WIDTH-1, -1, -1)} else {range_step(0, WIDTH, 1)};
        for i in w
        {
            /* if y>=0 : ↑ | else ↓ */
            let mut h = if y >= 0 {range_step(HEIGHT-1, -1, -1)} else {range_step(0, HEIGHT, 1)};
            for j in h
            {
                if i+x >= 0 && j+y >= 0 && i+x < WIDTH && j+y < HEIGHT && self.grid[i as uint][j as uint] != 0
                {
                    self.merge_seq(vec, i, j);
                }
            }
        }
    }

    pub fn move(&mut self, vec: (int, int))
    {
        self.move_nb+=1;
        self.move_global(vec);
        self.merge(vec);
        self.move_global(vec); /* Plug holes \o/ */
    }

    pub fn is_moved(g1: Game, g2: Game) -> bool
    {
        for i in range(0, WIDTH)
        {
            for j in range(0, HEIGHT)
            {
                if g1.grid[i as uint][j as uint] != g2.grid[i as uint][j as uint]
                {
                    return true;
                }
            }
        }

        false
    }

    pub fn list_move(self) -> ~[int]
    {
        let mut tmp: Game;
        let mut ret: ~[int] = ~[];

        /* Tries to move the grid in each direction, and sees if there have been any changes */
        for i in range(0, 4)
        {
            tmp = self;
            tmp.move(Game::int_to_vec(i));

            if Game::is_moved(self, tmp) == true
            {
                ret.push(i);
            }
        }

        ret
    }

    pub fn list_tile_empty(&mut self) -> ~[(int, int)]
    {
        let mut ret: ~[(int, int)] = ~[];

        /* List the position of all empty tiles */
        for i in range(0, WIDTH)
        {
            for j in range(0, HEIGHT)
            {
                if self.grid[i as uint][j as uint] == 0
                {
                    ret.push((i, j));
                }
            }
        }

        ret
    }

    pub fn add_random_tile(&mut self)
    {
        let tab = self.list_tile_empty();

        /* If there is at least one empty tile */
        if tab.len() > 0
        {
            /* Chooses a random position and add the new tile */
            let (a, b) = tab[random::<uint>()%tab.len()];
            self.grid[a as uint][b as uint] = 2;
        }
    }

    pub fn clone(self) -> Game
    {
        self
    }

    pub fn run(&mut self, get_vec: fn(game: &Game, move: ~[int])->(int, int))
    {
        /* Add 2 random tiles at start */
        self.add_random_tile();
        self.add_random_tile();

        let mut tmp = self.list_move();

        /* Play until it isn't possible to move */
        while tmp.len() > 0
        {
            self.move(get_vec(&self.clone(), tmp));
            self.add_random_tile();
            tmp = self.list_move();
        }
    }
}

