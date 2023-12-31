use fixedbitset::FixedBitSet;

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoodType {
    Basic,
    Whale,
    Turtle,
    Chameleon,
    Elephant,
    Alien,
    Caterpillar,
}

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Food {
    pub size: u8,
    pub ticks_left: u8,
    pub weight: u8,
    pub shape: FoodType,
    pub location: FieldPoint,
}

impl Food {
    pub fn new(shape: FoodType, p: FieldPoint) -> Food {
        match shape {
            FoodType::Basic => Food {
                ticks_left: 0,
                size: 1,
                weight: 8,
                shape,
                location: p,
            },
            _ => Food {
                ticks_left: 30,
                size: 2,
                weight: 45,
                shape,
                location: p,
            },
        }
    }
    // not generic but does the job
    pub fn is_at(&self, p: &FieldPoint) -> bool {
        match self.shape {
            FoodType::Basic => self.location.eq(p),
            _ => self.location.eq(p) || (self.location.add_tuple((1, 0))).eq(p),
        }
    }
}

#[derive(bincode::Encode, bincode::Decode, Debug, Clone, PartialEq, Eq)]
pub enum GameState {
    None,
    Playing,
    Quit,
    Over,
}

#[derive(Debug)]
pub struct Field {
    pub width: u16,
    pub height: u16,
    pub bit_set: FixedBitSet,
}
impl Field {
    pub fn new(width: u16, height: u16) -> Field {
        let bit_set = FixedBitSet::with_capacity(width as usize * height as usize);
        Field {
            width,
            height,
            bit_set,
        }
    }
    fn to_idx(&self, p: &FieldPoint) -> u16 {
        (p.y * self.width) + p.x
    }
    pub fn from_idx(&self, idx: u16) -> FieldPoint {
        FieldPoint {
            x: idx % self.width,
            y: idx / self.width,
        }
    }
    pub fn filled(&self, p: &FieldPoint) -> bool {
        self.idx_filled(self.to_idx(p))
    }
    pub fn idx_filled(&self, idx: u16) -> bool {
        self.bit_set.contains(idx.into())
    }
    pub fn set(&mut self, p: &FieldPoint, enabled: bool) {
        self.bit_set.set(self.to_idx(p).into(), enabled)
    }
}

#[repr(C)]
#[derive(bincode::Encode, bincode::Decode, Debug, PartialEq, Eq, Copy, Clone)]
pub struct FieldPoint {
    pub x: u16,
    pub y: u16,
}

impl FieldPoint {
    pub fn add_tuple(&self, tuple: (u16, u16)) -> FieldPoint {
        let (x, y) = (self.x + tuple.0, self.y + tuple.1);
        FieldPoint { x, y }
    }
    pub fn wrapping_add(&self, direction: WrappableDirection) -> FieldPoint {
        let x = match direction.to {
            Direction::Right => self.x.wrapping_add(1).wrapping_rem(direction.max.x),
            Direction::Left => self
                .x
                .wrapping_add(direction.max.x - 1)
                .wrapping_rem(direction.max.x),
            _ => self.x,
        };
        let y = match direction.to {
            Direction::Down => self.y.wrapping_add(1).wrapping_rem(direction.max.y),
            Direction::Up => self
                .y
                .wrapping_add(direction.max.y - 1)
                .wrapping_rem(direction.max.y),
            _ => self.y,
        };
        FieldPoint { x, y }
    }
    pub fn wrapping_sub(&self, direction: WrappableDirection) -> FieldPoint {
        let mut direction = direction;
        direction.to = opposite_of(direction.to);
        self.wrapping_add(direction)
    }
}

#[repr(C)]
#[derive(bincode::Encode, bincode::Decode, Debug, Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
pub fn opposite_of(direction: Direction) -> Direction {
    match direction {
        Direction::Left => Direction::Right,
        Direction::Up => Direction::Down,
        Direction::Right => Direction::Left,
        Direction::Down => Direction::Up,
    }
}
#[derive(bincode::Encode, bincode::Decode, Debug, Copy, Clone)]
pub struct WrappableDirection {
    pub to: Direction,
    pub max: FieldPoint,
}

#[derive(bincode::Encode, bincode::Decode, Clone, Debug)]
pub struct GameConfig {
    pub size: u16,
    pub start: (u16, u16),
    pub dim: (u16, u16),
    pub direction: Direction,
}

impl Default for GameConfig {
    fn default() -> GameConfig {
        GameConfig {
            size: 5,
            start: (4, 3),
            dim: (15, 10),
            direction: Direction::Right,
        }
    }
}
