// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.


#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(_x: i32, _y: i32, _d: Direction) -> Self {
        Robot {
            x: _x,
            y: _y,
            direction: _d,
        }
    }

    pub fn turn_right(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            direction: direction_after_right(self.direction),
        }
    }

    pub fn turn_left(self) -> Self {
        Robot {
            x: self.x,
            y: self.y,
            direction: direction_after_left(self.direction),
        }
    }

    pub fn advance(self) -> Self {
        pos_after_advance((self.x,self.y),self.direction)
    }

    pub fn instructions(self, instructions: &str) -> Self {
        let mut tm = self;
        for c in instructions.chars() {
            match c {
                'R'=>tm = tm.turn_right(),
                'L'=>tm = tm.turn_left(),
                'A'=>tm = tm.advance(),
                _=>(),
            }
        }
        tm
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x,self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

fn direction_after_left(dr: Direction) -> Direction {
    return match dr {
        Direction::East => Direction::North,
        Direction::South => Direction::East,
        Direction::West => Direction::South,
        Direction::North => Direction::West,
    };
}

fn direction_after_right(dr: Direction) -> Direction {
    return match dr {
        Direction::East => Direction::South,
        Direction::South => Direction::West,
        Direction::West => Direction::North,
        Direction::North => Direction::East,
    };
}

fn pos_after_advance((x, y): (i32, i32),dr:Direction) -> Robot {
    let (nx,ny) =  match dr {
        Direction::East => (x + 1, y),
        Direction::South => (x, y - 1),
        Direction::West => (x - 1, y),
        Direction::North => (x, y + 1),
    };
    Robot {
        x:nx,
        y:ny,
        direction:dr,
    }
}