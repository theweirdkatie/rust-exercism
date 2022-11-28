// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
use enum_iterator::{first, last, next, previous, Sequence};

#[derive(PartialEq, Eq, Debug, Sequence, Default, Copy, Clone)]
pub enum Direction {
    #[default]
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Debug)]
pub struct Robot{
    pos: (i32, i32),
    dir: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            pos: (x, y),
            dir: d,
        }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        Self {
            pos: self.pos,
            dir: next(&self.dir).unwrap_or(first::<Direction>().unwrap_or_default())
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Self {
            pos: self.pos,
            dir: previous(&self.dir).unwrap_or(last::<Direction>().unwrap_or_default())
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut position = self.pos;
        match self.dir {
            Direction::North | Direction::South => if self.dir==Direction::North {position.1 += 1} else {position.1 -= 1},
            Direction::East | Direction::West => if self.dir==Direction::East {position.0 += 1} else {position.0 -= 1},
        };
        Self {pos: position, dir: self.dir}
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for char in instructions.chars() {
            match char {
                'L' => robot = robot.turn_left(),
                'R' => robot = robot.turn_right(),
                _ => robot = robot.advance(),
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
