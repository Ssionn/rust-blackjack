use std::fmt;

use super::suit::{Suit, Value};

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = match self.value {
            Value::Ace => "A".to_string(),
            Value::King => "K".to_string(),
            Value::Queen => "Q".to_string(),
            Value::Jack => "J".to_string(),
            Value::Ten => "10".to_string(),
            Value::Nine => "9".to_string(),
            Value::Eight => "8".to_string(),
            Value::Seven => "7".to_string(),
            Value::Six => "6".to_string(),
            Value::Five => "5".to_string(),
            Value::Four => "4".to_string(),
            Value::Three => "3".to_string(),
            Value::Two => "2".to_string(),
        };
        let suit = match self.suit {
            Suit::Hearts => "♥",
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Spades => "♠",
        };
        write!(f, "{}{}", value, suit)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Card {
    suit: Suit,
    pub value: Value,
}

impl Card {
    pub fn new(suit: Suit, value: Value) -> Card {
        Card { suit, value }
    }
}
