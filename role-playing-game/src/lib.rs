// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
#[derive(Copy, Clone)]
pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // Check player is dead (health = 0)
        // Return new player instance with health 100
        // If level >=10, mana=100, else none
        let mut newplayer = self.clone();
        if newplayer.health <= 0 {
            newplayer.health = 100;
            if self.level >= 10 {
                newplayer.mana = Some(100)
            } else {
                newplayer.mana = None
            };
            Some(newplayer)
        } else {
            None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // If player does not have access to mana pool, reduce health by mana_cost
        // and return 0 damage
        match self.mana {
            None => {
                if self.health >= mana_cost {
                    self.health = self.health - mana_cost;
                } else {
                    self.health = 0;
                }
                0
            }
            Some(_) => {
                println!("Mana has some? {:?}", self.mana);
                // Returns damage = 2x mana or 0 if not enough mana
                // Modifies player if mana used
                // self is type Option so convert mana_cost to some for compare
                if self.mana >= Some(mana_cost) {
                    // To get the value inside of Some(u32) use unwrap()
                    // Recontextualize with Some
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    mana_cost * 2
                } else {
                    0
                }
            }
        }
    }
}
