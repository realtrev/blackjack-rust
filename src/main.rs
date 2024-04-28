mod input;
mod card;
mod deck;
mod hand;
mod game;

use game::{Game, HomeMenu};

use crate::input::wait_for_keypress;

fn main() {
    println!("Blackjack - Trevor - v2.0\nRust - 4/28/2024");
    let mut game = Game::new();
    game.deck.shuffle();

    loop {
        input::clear();
        let next = game.menu_home();

        match next {
            HomeMenu::NextRound => {
                game.play_round();
                
                println!("Press any key to continue...");
                std::thread::sleep(std::time::Duration::from_millis(200));
                wait_for_keypress();
                
                if game.player.balance <= 0 {
                    println!("You're out of money! Game over.");
                    std::thread::sleep(std::time::Duration::from_millis(200));
                    wait_for_keypress();
                    std::process::exit(0);
                }
            },
            HomeMenu::ViewHistory => {
                game.view_history();

                println!("\nPress any key to continue...");
                std::thread::sleep(std::time::Duration::from_millis(200));
                wait_for_keypress();
            },
            HomeMenu::Quit => std::process::exit(0),
        }
    }
}