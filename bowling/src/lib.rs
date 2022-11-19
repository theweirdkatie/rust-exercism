#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Default, Debug)]
pub struct BowlingGame {
    rolls: Vec<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            ..Default::default()
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        let current_roll = self.rolls.len();

        if pins > 10 {
            Err(Error::NotEnoughPinsLeft)
        } else if current_roll >= 20 && !self.was_spare(current_roll) {
            Err(Error::GameComplete)
        } else {
            self.rolls.push(pins);
            if let 10 = pins {self.rolls.push(0)};
            Ok(())
        }
    }

    pub fn score(&self) -> Option<u16> {
        if self.rolls.len() < 20 {
            None
        } else {
            let mut score = 0;
            let mut roll = 0;
            for _frame in 1..=10 {
                if self.was_strike(roll) {
                    score += (self.rolls[roll] + self.rolls[roll+1])*2;
                } else if self.was_spare(roll) {
                    score += self.rolls[roll] * 2 + self.rolls[roll + 1];
                } else {
                    score += self.rolls[roll] + self.rolls[roll + 1];
                }
                println!("Frame: {}, Pins: {}, {}, Score: {}", _frame, self.rolls[roll], self.rolls[roll+1], score);
                roll += 2;
            }
            if self.rolls.len() > 20 { score += self.rolls[20] };
            //dbg!(self);
            Some(score)
        }
    }

    pub fn was_spare(&self, roll_index: usize) -> bool {
        if roll_index > 1 {
            self.rolls[roll_index - 2] + self.rolls[roll_index - 1] == 10
        } else {
            false
        }
    }

    pub fn was_strike(&self, roll_index: usize) -> bool {
        if roll_index > 1 {
            self.rolls[roll_index-2] == 10
        } else {
            false
        }
    }
}
