mod card;
mod deck;
mod hand;

use crate::card::{Card, Rank, Suit};

use crate::hand::Hand;

use std::io;

use crate::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    println!("EPIC GAMBLING - Trevor (Rust)\nBlackjack - v1.0\n");

    deck.shuffle();

    let mut player = Hand::new();
    let mut dealer = Hand::new();

    let mut balance: i32 = 1000;

    loop {
        clear();
        player.clear();
        dealer.clear();

        if deck.cards.len() < 20 {
            deck.shuffle();
        }

        // balance
        println!("EPIC GAMBLING\nBalance: ${}", balance);

        // wager
        let mut wager = 0;
        loop {
            wager = read_positive("Enter your wager:");

            if wager > balance {
                println!("You cannot wager more than your balance.");
            } else {
                break;
            }
        }
        if wager == 0 {
            break;
        }

        // each draws one card
        player.add_card(deck.deal().unwrap());
        dealer.add_card(deck.deal().unwrap());

        // each draws a second card
        player.add_card(deck.deal().unwrap());
        dealer.add_card(deck.deal().unwrap());

        // player's turn
        while player.get_value() < 21 {
            clear();
            println!("EPIC GAMBLING\nBalance: ${}\nWager: ${}", balance, wager);
            println!("\n*** Player's hand: ***");
            player.display();
            println!("\nDealer's hand:");
            dealer.display_first_card();

            println!("Do you want to hit or stand? (h/s)");
            let mut input = String::new();
            loop {
                io::stdin().read_line(&mut input).unwrap();
                if input.trim() != "h" && input.trim() != "s" {
                    println!("Invalid input. Please enter 'h' or 's'.");
                    input.clear();
                } else {
                    break;
                }
            }

            if input.trim() == "h" {
                player.add_card(deck.deal().unwrap());
                println!("Player's hand:");
                player.display();
            } else {
                break;
            }
        }

        // dealer's turn
        while dealer.get_value() < 17 {
            dealer.add_card(deck.deal().unwrap());
        }

        if dealer.get_value() == 17 && dealer.soft {
            dealer.add_card(deck.deal().unwrap());
        }

        // display final hands
        clear();
        println!("EPIC GAMBLING\nBalance: ${}\nWager: ${}", balance, wager);
        println!("\n*** Player's hand: ***");
        player.display();
        println!("\nDealer's hand:");
        dealer.display();

        println!();

        if player.is_blackjack() {
            println!("*** BLACKJACK!!! ***");
            balance += wager * 3 / 2;
            println!("*** Balance: {} (+${}) ***", balance, wager * 3 / 2);
        } else {
            if player.get_value() > 21 {
                println!("Player busts!");
                if dealer.get_value() <= 21 {
                    println!("Dealer wins!");
                    balance -= wager;
                }
            }

            if dealer.get_value() > 21 {
                println!("Dealer busts!");
                if player.get_value() <= 21 {
                    println!("*** YOU WIN!!! ***");
                    balance += wager;
                    println!("*** Balance: {} (+${}) ***", balance, wager);
                }
            }

            if player.get_value() <= 21 && dealer.get_value() <= 21 {
                if player.get_value() > dealer.get_value() {
                    println!("*** YOU WIN ${}!!! ***", wager);
                    balance += wager;
                    println!("*** Balance: {} (+${}) ***", balance, wager);
                } else if player.get_value() < dealer.get_value() {
                    println!("Dealer wins!");
                    balance -= wager;
                } else {
                    println!("It's a push.");
                }
            }

            if player.get_value() > 21 && dealer.get_value() > 21 {
                println!("It's a push.");
            }
        }
        
        if balance == 0 {
            println!("You are out of money. Game over.");
            break;
        }

        println!("\nNext Game (Press ENTER)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
    }
    println!("Balance is ${}", balance);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
}

fn read_positive(message: &str) -> i32 {
    println!("{}", message);
    let mut input = String::new();
    let mut num = 0;
    loop {
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(number) => {
                if number < 0 {
                    println!("Invalid input. Please enter a positive integer.");
                    input.clear();
                    continue;
                }
                num = number;
                break;
            },
            Err(_) => {
                println!("Invalid input. Please enter an integer.");
                input.clear();
            }
        }
    }

    num
}

fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}