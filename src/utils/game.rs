use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;
use super::deck::Deck;
use super::hand::Hand;

pub struct Game {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
}

impl Game {
    pub fn new() -> Game {
        Game {
            deck: Deck::new(),
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
        }
    }

    pub fn play_round(&mut self) {
        self.player_hand = Hand::new();
        self.dealer_hand = Hand::new();

        print!("\nShuffling the deck");
        io::stdout().flush().unwrap();
        for _ in 0..3 {
            sleep(Duration::from_secs(1));
            print!(".");
            io::stdout().flush().unwrap();
        }
        println!();
        self.deck.shuffle();

        self.player_hand.add_card(self.deck.draw().unwrap());
        self.dealer_hand.add_card(self.deck.draw().unwrap());
        self.player_hand.add_card(self.deck.draw().unwrap());
        self.dealer_hand.add_card(self.deck.draw().unwrap());

        println!(
            "\nDealer shows: {}",
            self.dealer_hand.get_visible_card().unwrap()
        );

        println!("Your hand: {}", self.player_hand);

        self.player_turn();

        if self.player_hand.get_value() <= 21 {
            self.dealer_turn();
            self.determine_winner();
        } else {
            println!("Bust! You lose!");
        }
    }

    fn player_turn(&mut self) {
        loop {
            println!("\nHand value: {}", self.player_hand.get_value());

            if self.player_hand.get_value() == 21 {
                break;
            }

            println!("Hit or Stand? (h/s): ");

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "h" => {
                    let card = self.deck.draw().unwrap();
                    println!("You drew: {}", card);
                    self.player_hand.add_card(card);

                    if self.player_hand.get_value() > 21 {
                        println!("Bust! Your hand: {}", self.player_hand);
                        println!("Hand value: {}", self.player_hand.get_value());
                        break;
                    }
                }
                "s" => break,
                _ => println!("Invalid input! Please enter 'h' or 's'"),
            }
        }
    }

    fn dealer_turn(&mut self) {
        println!("\nDealer's turn:");
        println!("Dealer's hand: {}", self.dealer_hand);

        while self.dealer_hand.get_value() < 17 {
            sleep(Duration::from_secs(1));
            let card = self.deck.draw().unwrap();
            println!("Dealer draws: {}", card);
            self.dealer_hand.add_card(card);
            println!("Dealer's hand value: {}", self.dealer_hand.get_value());
        }
    }

    fn determine_winner(&self) {
        let player_value = self.player_hand.get_value();
        let dealer_value = self.dealer_hand.get_value();

        println!("\nFinal hands:");
        println!("Your hand: {} ({})", self.player_hand, player_value);
        println!("Dealer's hand: {} ({})", self.dealer_hand, dealer_value);

        if dealer_value > 21 {
            println!("Dealer busts! You win! 🎉");
        } else if player_value > dealer_value {
            println!("You win! 🎉");
        } else if dealer_value > player_value {
            println!("Dealer wins! 😔");
        } else {
            println!("It's a tie! 🤝");
        }
    }
}
