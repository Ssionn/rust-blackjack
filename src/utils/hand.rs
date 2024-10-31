use std::fmt;

use super::{card::Card, suit::Value};

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards: Vec<String> = self.cards.iter().map(|card| card.to_string()).collect();
        write!(f, "[{}]", cards.join(" "))
    }
}

#[derive(Clone, Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn get_visible_card(&self) -> Option<&Card> {
        self.cards.first()
    }

    pub fn get_value(&self) -> u8 {
        let mut sum = 0;
        let mut aces = 0;

        for card in &self.cards {
            match card.value {
                Value::Ace => aces += 1,
                Value::King | Value::Queen | Value::Jack => sum += 10,
                Value::Two => sum += 2,
                Value::Three => sum += 3,
                Value::Four => sum += 4,
                Value::Five => sum += 5,
                Value::Six => sum += 6,
                Value::Seven => sum += 7,
                Value::Eight => sum += 8,
                Value::Nine => sum += 9,
                Value::Ten => sum += 10,
            }
        }

        for _ in 0..aces {
            if sum + 11 <= 21 {
                sum += 11;
            } else {
                sum += 1;
            }
        }

        sum
    }
}
