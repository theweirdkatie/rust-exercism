
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
        let mut prev_pins = 0;
        
        if (current_roll%2 != 0 || current_roll>19) && self.rolls.get_mut(0).is_some() {
            prev_pins = self.rolls[current_roll-1];
            if prev_pins == 10 { prev_pins=0 } else {};
        }
        println!("Previous pins: {}", prev_pins);

        if pins > 10 || (pins + prev_pins > 10) {
            Err(Error::NotEnoughPinsLeft)
        // most number of rolls is 10 frames, *2 rolls/frame, +1 extra frame = 21
        } else if current_roll >= 21 {
            Err(Error::GameComplete)
        } else {
            println!("{} roll, {} pins", current_roll, pins);
            self.rolls.push(pins);
            if (pins == 10 && current_roll < 18) 
                    || (current_roll == 19 && (self.rolls[current_roll-1] + pins) < 10) {
                self.rolls.push(0);
            }
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
                    if self.rolls[roll] == 10 && _frame < 10 { score += self.rolls[roll+2] } else {};
                } else if self.was_spare(roll) {
                    score += self.rolls[roll] * 2 + self.rolls[roll + 1];
                } else {
                    score += self.rolls[roll] + self.rolls[roll + 1];
                }
                // println!("Frame: {}, Pins: {}, {}, Score: {}", _frame, self.rolls[roll], self.rolls[roll+1], score);
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
