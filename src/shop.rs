use std::fmt; 
use std::cmp::Ordering;

use crate::{card::Card, FightResult};

pub struct Shop {
    pub cards: Vec<Card>,
}

impl Shop {
    /// Get the price of the most expensive card in the shop
    pub fn most_expensive(&self) -> u32 {
        self.cards.iter().map(|card| card.price).max().unwrap()
    }

    /// Get the total damage of all cards in the shop
    pub fn total_damage(&self) -> u32 {
        self.cards.iter().map(|card| card.damage).sum()
    }

    /// Get the total health of all cards in the shop
    pub fn total_health(&self) -> u32 {
        self.cards.iter().map(|card| card.health).sum()
    }

    /// Simulate a fight against another store. Returns a FightResult::Win if
    /// this store wins, FightResult::Loss if this store loses, and a
    /// FightResult::Tie if both stores win the same number of battles.
    pub fn fight_store(&self, other: &Shop) -> FightResult {
        let mut shop1_wins: i32 = 0;
        for this_card in self.cards.iter() {
            for other_card in other.cards.iter() {
                let temp = this_card.print_fight(other_card);
                match temp {
                    FightResult::Win => shop1_wins += 1,
                    FightResult::Loss => shop1_wins -= 1,
                    _ => (),
                }
            }
        }
        match shop1_wins.cmp(&0) {
            Ordering::Greater => FightResult::Win,
            Ordering::Less => FightResult::Loss,
            Ordering::Equal => FightResult::Tie
        }
    }
}

// Implement the Display trait for Shop so that it can be printed. Print the
// shop's stats, including the most expensive card, the total damage, and the
// total health.
impl fmt::Display for Shop {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "|Shop: {}/{}/{}|",
            self.most_expensive(),
            self.total_damage(),
            self.total_health()
        )
    }
}
