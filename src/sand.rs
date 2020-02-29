#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Species {
    Empty = 0,
    Wall = 1,
    Sand = 2,
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    species: Species,
    ra: u8,
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
                let n = api.get(0, 1);
                let rx = random_dir2();
                if n.species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(0 ,1, cell);
                } else if api.get(rx, 1).species == Species::Empty && api.get(rx, 0).species == Species::Empty {
                    api.set(0, 0, EMPTY_CELL);
                    api.set(rx,1, cell);
                } else {
                    api.set(0, 0, cell);
                }
            }
        } 
    }
}

impl Cell {
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
    pub fn paint (&mut self, x: u32, y: u32, species: Species) {
        if x as i32 > self.width || y as i32 > self.height
            { panic!("out of bounds error"); }
        let i = self.get_index(x as i32, y as i32);
        if self.cells[i].species == Species::Empty {
            self.cells[i] = Cell {
                species,
                ra: 0,
                clock: self.generation,
            };
        }
    }
    pub fn render (&self, buffer: &mut [u8], pitch: usize) {
        for x in 0..(self.width * self.cell_size) {
            for y in 0..(self.height * self.cell_size) {
                let xi = x / self.cell_size;
                let yi = y / self.cell_size;
                let c = self.get_cell(xi ,yi);
                let colour: u32 = match c.species {Species::Empty => {0xFF_FF_FF},Species::Wall => {0x424242},Species::Sand => {0xEDC9AF},};
                let idx = (y as usize)*pitch + (x as usize)*3;
                buffer[idx] = ((colour & 0xFF_00_00) >> 16) as u8;
                buffer[idx + 1] = ((colour & 0xFF_00) >> 8) as u8;
                buffer[idx + 2] = (colour & 0xFF) as u8;
            }
        }
    }
    fn get_cell (&self, x: i32, y: i32) -> Cell {
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
            return Cell {
                species: Species::Wall,
                ra: 0,
                clock: self.world.generation
            }
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

fn random_dir2 () -> i32 {
    if rand::random::<u32>() % 2 == 0 {
        return -1;
    } else {
        return 1;
    }
}