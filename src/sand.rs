#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
    Water = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub species: Species,
    pub ra: u8,
    clock: u8,
}

#[derive(Debug)]
pub struct World {
    width: i32,
    height: i32,
    cell_size: i32,
    pub cells: Vec<Cell>,
    generation: u8,
}

pub struct Api<'a> {
    world: &'a mut World,
    x: i32,
    y: i32,
}

static EMPTY_CELL: Cell = Cell {
    species: Species::Empty,
    ra: 0,
    clock: 0,
};

impl Species {
    pub fn update (&self, cell: Cell, mut api: Api) {
        match self {
            Species::Empty => {},
            Species::Wall => {},
            Species::Sand => {
                let mut n = api.get(0, 1);
                let rx = random_dir2();
                if n.species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(0 ,1, cell);
                } else if api.get(rx, 1).species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(rx,1, cell);
                } else if n.species == Species::Water {
                    api.set(0, 0, n);
                    api.set(0 ,1, cell);
                } else {
                    n = api.get(rx, 1);
                    if n.species == Species::Water {
                        api.set(0, 0, n);
                        api.set(rx,1, cell);
                    } else {
                        api.set(0, 0, cell);
                    }
                }
            }
            Species::Water => {
                for &i in random_dir_vec().iter() {
                    let n = api.get(i, 1);
                    if n.species == Species::Empty {
                        api.set(0, 0, EMPTY_CELL);
                        api.set(i, 1, cell);
                        return;
                    }

                }
                let rx = random_dir2();
                if api.get(rx, 0).species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(rx, 0, cell);
                } else if api.get(-rx, 0).species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(-rx, 0, cell);
                }
            },
        } 
    }
}

impl Cell {
    pub fn new (species: Species, clock: u8) -> Cell {
        Cell {
            species,
            ra: rand::random::<u8>(),
            clock,
        }
    }
    pub fn update (&self, api: Api) {
        self.species.update(*self, api);
    }
}

impl World {
    pub fn new (width: u32, height: u32, size: u32) -> World {
        World {
            width: width as i32,
            height: height as i32,
            cell_size: size as i32,
            cells: (0..height*width).map(|_| {EMPTY_CELL}).collect(),
            generation: 0
        }
    }
    pub fn reset (&mut self) {
        for i in 0..self.height*self.width {
            self.cells[i as usize] = EMPTY_CELL;
        }
    }
    pub fn tick (&mut self) {
        self.generation = self.generation.wrapping_add(1);
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell (x, y);
                if cell.clock - self.generation == 1
                    { continue; }
                cell.update(Api {
                    world: self,
                    x,
                    y,
                });
            }
        }
        self.generation = self.generation.wrapping_add(1);
    }
    pub fn paint (&mut self, x: u32, y: u32, s: u32, species: Species, erase: bool) {
        let x = x as i32;
        let y = y as i32;
        let r = (s/2) as i32;
        if s == 1 {
            let i = self.get_index(x, y);
            if erase {
                self.cells[i] = Cell::new(species, self.generation);
            } else if self.cells[i].species == Species::Empty {
                self.cells[i] = Cell::new(species, self.generation);
            }
            return;
        }
        for rx in -r..r {
            for ry in -r..r {
                let px = x + rx;
                let py = y + ry;
                if px >= self.width || px < 0 || py >= self.height || py < 0
                    {continue;}
                let i = self.get_index(px, py);
                if erase {
                    self.cells[i] = Cell::new(species, self.generation);
                } else if self.cells[i].species == Species::Empty {
                    self.cells[i] = Cell::new(species, self.generation);
                }
            }
        }
    }
    pub fn get_cell (&self, x: i32, y: i32) -> Cell {
        self.cells[((y*self.width)+x) as usize]
    }
    fn get_index (&self, x: i32, y: i32) -> usize {
        ((y*self.width)+x) as usize
    }
}

impl<'a> Api<'a> {
    pub fn get (&mut self, dx: i32, dy: i32) -> Cell {
        if dx > 1 || dx < -1 || dy > 1 || dy < -1 
            { panic!("out of bounds error"); }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx >= self.world.width || ny < 0 || ny >= self.world.height {
            return Cell::new(Species::Wall, self.world.generation);
        }
        self.world.get_cell(nx, ny)
    }
    pub fn set (&mut self, dx: i32, dy: i32, c: Cell) {
        if dx > 1 || dx < -1 || dy > 1 || dy < -1
            { panic!("out of bounds error"); }
        let nx = self.x + dx;
        let ny = self.y + dy;
        if nx < 0 || nx >= self.world.width || ny < 0 || ny >= self.world.height
            { return; }
        let idx = self.world.get_index(nx, ny);
        self.world.cells[idx] = c;
        self.world.cells[idx].clock = self.world.generation.wrapping_add(1);
    }
}

fn random_dir () -> i32 {
    return (rand::random::<i32>() % 3) -1;
}

fn random_dir_vec () -> [i32;3] {
    let r = rand::random::<i32>() % 6;
    return match r {
        0 => [-1,0,1],
        1 => [0,-1,1],
        2 => [1,-1,0],
        3 => [-1,1,0],
        4 => [0,1,-1],
        5 => [1,0,-1],
        _ => [0,0,0],
    }
}

fn random_dir2 () -> i32 {
    if rand::random::<u32>() % 2 == 0 {
        return -1;
    } else {
        return 1;
    }
}