#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    rolls: u16,
    frames: Vec<[u16; 2]>,
    bonus_throw: u16,
}

trait SumFrame {
    fn sum_frame(&self) -> u16;
}

impl SumFrame for [u16; 2] {
    fn sum_frame(&self) -> u16 {
        self[0] + self[1]
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        self.rolls += 1;

        // check if more than max throws, 21
        if self.rolls > 21 {
            Err(Error::GameComplete)

        // check if enough pins remaining
        } else if !self.check_enough_pins(pins) {
            Err(Error::NotEnoughPinsLeft)

        // record roll
        } else {
            // check if second throw on frame
            if self.rolls % 2 == 0 {
                let frame_count = self.frames.len() - 1;
                self.frames[frame_count][1] = pins;

                // If rolled open for last frame, no bonus throw, complete by increasing throw count
                if self.rolls == 20 && self.frames[9].sum_frame() < 10 {
                    self.rolls += 1;
                } else {
                };

            // check if bonus throw
            } else if self.rolls == 21 {
                self.bonus_throw = pins;
            } else {
                self.frames.push([pins, 0]);
                // If strike, complete frame by increasing throw count
                if pins == 10 && self.rolls < 19 {
                    self.rolls += 1
                } else {
                };
            }
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.rolls < 21 {
            None
        } else {
            let mut score = 0;
            // let mut roll = 0;
            for _frame in 1..10 {
                // check strike
                if self.is_strike(_frame) {
                    score += 10 + self.frames[_frame].sum_frame();

                    // check if followed by another strike
                    if self.is_strike(_frame + 1) && _frame < 9 {
                        score += self.frames[_frame + 1][0];
                    } else {
                    };
                // check spare
                } else if self.is_spare(_frame) {
                    score += 10 + self.frames[_frame][0];
                // open frame
                } else {
                    score += self.frames[_frame - 1].sum_frame();
                }
            }

            // Frame 10
            score += self.frames[9].sum_frame() + self.bonus_throw;

            Some(score)
        }
    }

    pub fn is_spare(&self, frame_index: usize) -> bool {
        self.frames[frame_index - 1].sum_frame() == 10
    }

    pub fn is_strike(&self, frame_index: usize) -> bool {
        self.frames[frame_index - 1][0] == 10
    }

    pub fn check_enough_pins(&self, pin_count: u16) -> bool {
        // Check for frames 1-9
        if self.frames.len() < 10 && self.rolls < 19 && self.rolls > 1 {
            match self.rolls % 2 {
                0 => {
                    let this_frame = self.frames.last().unwrap_or(&[0,0]);
                    this_frame[0] + pin_count <= 10
                }
                _ => pin_count <= 10,
            }

        } else {
            // Frame 10
            // Unique because could have a third throw
            let this_frame = self.frames.last().unwrap_or(&[0,0]);
            if this_frame[0] == 10 {
                this_frame[1] + pin_count <= 10 || (this_frame[1] == 10 && pin_count <= 10)
            } else {
                this_frame[0] + pin_count <= 10 || this_frame[0] + this_frame[1] == 10
            }
        }
    }
}
