pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match &self.health {
            0 => Some(Player { 
                health: 100, 
                mana: match &self.level {
                    level if level >= &10 => Some(100),
                    _ => None
                }, 
                level: self.level
            }),
            _ => None
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match &self.mana {
            Some(mana) => {
                if mana_cost > *mana {
                    return 0
                } else {
                    self.mana = Some(self.mana.unwrap() - mana_cost);
                    return mana_cost * 2
                }
            },
            None => {
                if mana_cost > self.health {
                    self.health = 0;
                } else {
                    self.health = self.health - mana_cost;
                }
                return 0
            },
        }
    }
}
