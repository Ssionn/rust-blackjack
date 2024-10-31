use std::io;

use utils::game::Game;

mod utils;

fn main() {
    println!("Welcome to Rust Blackjack!");
    let mut game = Game::new();

    loop {
        game.play_round();

        println!("\nWould you like to play again? (y/n): ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim().to_lowercase() != "y" {
            break;
        }
    }

    println!("Thanks for playing!");
}
