use std::cmp::Ordering;

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn new(level: u32) -> Self {
        Player {
            health: 100,
            mana: if level >= 10 { Some(100) } else { None },
            level,
        }
    }

    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player::new(self.level)),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana.as_mut() {
            None => {
                self.health = self.health.checked_sub(mana_cost).unwrap_or(0);
                0
            }
            Some(m) => match (*m).cmp(&mana_cost) {
                Ordering::Less => 0,
                Ordering::Greater | Ordering::Equal => {
                    *m -= mana_cost;
                    mana_cost * 2
                }
            },
        }
    }
}
