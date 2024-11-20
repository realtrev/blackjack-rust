use crate::card::{Card, Rank};
use crate::deck::Deck;
use crate::hand::Hand;
use crate::input;
use crate::input::{clear, clear_lines, wait_for_keypress};

#[derive(Copy, Clone)]
pub enum HomeMenu {
    NextRound,
    ViewHistory,
    Quit,
}

pub struct Game {
    pub player: Player,
    pub dealer: Dealer,
    pub deck: Deck,
}

pub struct Player {
    pub hand: Hand,
    pub split: Hand,
    pub balance: i32,
    pub bet: i32,
    pub split_bet: i32,
    pub history: Vec<Round>,
    pub insurance: bool,
}

pub struct Dealer {
    pub hand: Hand,
}

pub struct Round {
    pub player_hand: Hand,
    pub player_split: Hand,
    pub split: bool,
    pub dealer_hand: Hand,
    pub result: RoundResult,
    pub split_result: RoundResult,
    pub bet: i32,
    pub split_bet: i32,
    pub payout: i32,
    pub insurance: bool,
}

#[derive(Copy, Clone)]
pub enum RoundResult {
    Win,
    Loss,
    Push,
}

impl Game {
    pub fn new() -> Game {
        Game {
            player: Player {
                hand: Hand::new(),
                split: Hand::new(),
                balance: 1000,
                bet: 0,
                split_bet: 0,
                history: Vec::new(),
                insurance: false,
            },
            dealer: Dealer {
                hand: Hand::new(),
            },
            deck: Deck::new()
        }
    }

    pub fn menu_home(&self) -> HomeMenu {
        let options = vec![
            input::Choice { label: "Next Round".to_string(), value: HomeMenu::NextRound },
            input::Choice { label: "View History".to_string(), value: HomeMenu::ViewHistory },
            input::Choice { label: "Quit".to_string(), value: HomeMenu::Quit },
        ];

        println!("EPIC GAMBLING");
        println!("Balance: ${}", self.player.balance);

        println!("\nSelect an option:");
        input::select(options)
    }
    
    pub fn view_history(&self) {
        clear();
        println!("EPIC GAMBLING");
        println!("Balance: ${}", self.player.balance);
        println!("\n  History:");
        
        if self.player.history.len() == 0 {
            println!("  No history yet");
        }
        
        for round in &self.player.history {
            // WIN - $100 - Dealer: 21 (7 of Clubs, 4 of Clubs) - Player: 20 (King of Hearts, Queen of Hearts)
            // SPLIT
            //     WIN - $50 - Dealer: 21 (7 of Clubs, 4 of Clubs) - Player: 20 (King of Hearts, Queen of Hearts)
            //     LOSS - $50 - Dealer: 21 (7 of Clubs, 4 of Clubs) - Player: 20 (King of Hearts, Queen of Hearts)
            // WIN - $100 - Dealer: 21 (7 of Clubs, 4 of Clubs) - Player: 20 (King of Hearts, Queen of Hearts)
            
            if round.split {
                println!("  SPLIT");
                println!("      ${} {} - Dealer: {} - Player: {} (${}) - Insurance: {}",
                    round.bet,
                    match round.result {
                        RoundResult::Win => "WIN",
                        RoundResult::Loss => "LOSS",
                        RoundResult::Push => "PUSH",
                    },
                    round.dealer_hand.get_value(),
                    round.player_hand.get_value(),
                    round.payout,
                    round.insurance
                );
                println!("      ${} {} - Dealer: {} - Player: {} (${}) - Insurance: {}",
                    round.split_bet,
                    match round.split_result {
                        RoundResult::Win => "WIN",
                        RoundResult::Loss => "LOSS",
                        RoundResult::Push => "PUSH",
                    },
                    round.dealer_hand.get_value(),
                    round.player_split.get_value(),
                    round.payout,
                    round.insurance
                );
                continue;
            }
            
            println!("  ${} {} - Dealer: {} - Player: {} (${}) - Insurance: {}",
                round.bet,
                match round.result {
                    RoundResult::Win => "WIN",
                    RoundResult::Loss => "LOSS",
                    RoundResult::Push => "PUSH",
                },
                round.dealer_hand.get_value(),
                round.player_hand.get_value(),
                round.payout,
                round.insurance
            );
        }
    }

    pub fn play_round(&mut self) {
        if self.deck.cards.len() < 26 {
            self.deck.shuffle();
        }
        clear();
        println!("EPIC GAMBLING");
        println!("Balance: ${}", self.player.balance);

        let wager = self.request_wager();
        self.player.split_bet = 0;
        self.player.insurance = false;

        self.player.hand.clear();
        self.dealer.hand.clear();
        self.player.split.clear();

        self.player.hand.add_card(self.deck.deal().unwrap());
        self.dealer.hand.add_card(self.deck.deal().unwrap());
        self.player.hand.add_card(self.deck.deal().unwrap());
        self.dealer.hand.add_card(self.deck.deal().unwrap());
        
        let mut split = false;
        let mut insurance = 0;

        loop {
            clear();
            println!("EPIC GAMBLING");
            println!("Balance: ${}", self.player.balance);
            println!("Current bet: ${}", self.player.bet);
            if split {
                println!("Split bet: ${}", self.player.split_bet);
            }
            if self.player.insurance {
                println!("Insurance: ${}", insurance);
            }

            println!("\n  Dealer's hand:");
            self.dealer.hand.display_first_card();

            println!("> *** Your hand: ***");
            self.player.hand.calculate_value();
            self.player.hand.display();

            let mut options = vec![
                input::Choice { label: "Hit".to_string(), value: "hit" },
                input::Choice { label: "Stand".to_string(), value: "stand" },
                input::Choice { label: "Double Down".to_string(), value: "double" },
            ];

            // insurance
            if self.dealer.hand.cards[0].get_value() == Rank::Ace as u8
                && self.player.balance >= self.player.bet / 2
                && !self.player.insurance
            {
                options.push(input::Choice { label: "Insurance".to_string(), value: "insurance" });
            }
            
            if self.player.hand.is_pair() && !split {
                options.push(input::Choice { label: "Split".to_string(), value: "split" });
            }

            if self.player.hand.get_value() >= 21 {
                break;
            }

            if split {
                println!("\n  *** Your split hand: ***");
                self.player.split.display();
                println!("\nSelect an option: (first hand)");
            } else {
                println!("\nSelect an option:");
            }
            let choice = input::select(options);

            match choice {
                "hit" => {
                    self.player.hand.add_card(self.deck.deal().unwrap());

                    println!("Your hand:");
                },
                "stand" => {
                    break;
                },
                "double" => {
                    if self.player.balance < insurance + self.player.bet * 2 {
                        println!("You don't have enough money to double down");
                        wait_for_keypress();
                        continue;
                    }
                    self.player.bet *= 2;
                    self.player.hand.add_card(self.deck.deal().unwrap());
                    break;
                },
                "split" => {
                    // check if the player has enough money to split
                    if self.player.balance < insurance + self.player.bet * 2 {
                        println!("You don't have enough money to split");
                        wait_for_keypress();
                        continue;
                    }
                    self.player.split_bet = self.player.bet;
                    let card: Card = self.player.hand.cards.pop().unwrap();
                    self.player.split.add_card(card);
                    self.player.hand.calculate_value();
                    split = true;
                },
                "insurance" => {
                    // check if the player has enough money to insure
                    if self.player.balance < self.player.bet + self.player.split_bet + self.player.bet / 2 {
                        println!("You don't have enough money to insure");
                        wait_for_keypress();
                        continue;
                    }
                    insurance = self.player.bet / 2;
                    self.player.insurance = true;
                },
                _ => panic!("Invalid choice"),
            }

            if self.player.hand.get_value() >= 21 {
                break;
            }
        }
        
        if split {
            loop {
                clear();
                println!("EPIC GAMBLING");
                println!("Balance: ${}", self.player.balance);
                println!("First bet: ${}", self.player.bet);
                println!("Split bet: ${}", self.player.split_bet);
                if self.player.insurance {
                    println!("Insurance: ${}", insurance);
                }

                println!("\n  Dealer's hand:");
                self.dealer.hand.display_first_card();

                println!("  *** Your hand: ***");
                self.player.hand.display();

                println!("\n> *** Your split hand: ***");
                self.player.split.display();

                let mut options = vec![
                    input::Choice { label: "Hit".to_string(), value: "hit" },
                    input::Choice { label: "Stand".to_string(), value: "stand" },
                    input::Choice { label: "Double Down".to_string(), value: "double" },
                ];

                if self.player.split.get_value() >= 21 {
                    break;
                }

                if self.dealer.hand.cards[0].get_value() == Rank::Ace as u8
                    && self.player.balance >= self.player.bet / 2
                    && !self.player.insurance
                {
                    options.push(input::Choice { label: "Insurance".to_string(), value: "insurance" });
                }

                println!("\nSelect an option: (split hand)");
                let choice = input::select(options);

                match choice {
                    "hit" => {
                        self.player.split.add_card(self.deck.deal().unwrap());
                    },
                    "stand" => {
                        break;
                    },
                    "double" => {
                        if self.player.balance < insurance + self.player.bet + self.player.split_bet * 2 {
                            println!("You don't have enough money to double down");
                            wait_for_keypress();
                            continue;
                        }
                        self.player.split_bet *= 2;
                        self.player.split.add_card(self.deck.deal().unwrap());
                        continue;
                    },
                    "insurance" => {
                        // check if the player has enough money to insure
                        if self.player.balance < self.player.bet + self.player.split_bet + self.player.bet / 2 {
                            println!("You don't have enough money to insure");
                            wait_for_keypress();
                            continue;
                        }
                        insurance = self.player.bet / 2;
                        self.player.insurance = true;
                    },
                    _ => panic!("Invalid choice"),
                }

                if self.player.split.get_value() >= 21 {
                    break;
                }
            }
        }

        while self.dealer.hand.get_value() < 17 {
            self.dealer.hand.add_card(self.deck.deal().unwrap());
        }

        if self.dealer.hand.soft && self.dealer.hand.get_value() == 17 {
            self.dealer.hand.add_card(self.deck.deal().unwrap());
        }

        clear();

        println!("EPIC GAMBLING");
        println!("Balance: ${}", self.player.balance);
        println!("Current bet: ${}", self.player.bet);

        println!("\n  Dealer hand:");
        self.dealer.hand.display();

        println!("\n  Your hand:");
        self.player.hand.display();
        
        if split {
            println!("\n  Your split hand:");
            self.player.split.display();
        }

        let result = self.get_round_result(&self.player.hand, &self.dealer.hand);
        let mut split_result = self.get_round_result(&self.player.split, &self.dealer.hand);
        if !split {
            split_result = RoundResult::Push;
        }
        
        let mut payout = 0;
        
        match result {
            RoundResult::Win => {
                payout += self.player.bet;
            },
            RoundResult::Loss => {
                payout -= self.player.bet;
            },
            RoundResult::Push => {},
        }
        
        if split {
            match split_result {
                RoundResult::Win => {
                    payout += self.player.split_bet;
                },
                RoundResult::Loss => {
                    payout -= self.player.split_bet;
                },
                RoundResult::Push => {},
            }
        }

        if self.player.insurance && self.dealer.hand.is_blackjack() {
            payout += insurance;
        } else {
            payout -= insurance;
        }

        let round = Round {
            player_hand: self.player.hand.clone(),
            player_split: self.player.split.clone(),
            split,
            dealer_hand: self.dealer.hand.clone(),
            result,
            split_result,
            bet: wager,
            split_bet: self.player.split_bet,
            payout,
            insurance: self.player.insurance,
        };
        
        self.print_winner(round.result, self.player.bet, insurance, split);
        if split {
            self.print_winner(round.split_result, self.player.split_bet, insurance,false);
        }

        self.player.history.push(round);
    }
    
    pub fn print_winner(&mut self, result: RoundResult, bet: i32, insurance: i32, split: bool) {
        if self.player.insurance && self.dealer.hand.is_blackjack() && split == false {
            self.player.balance += insurance;
            println!("\n**** INSURANCE PAYS ${} ****", insurance);
            println!("**** Balance: ${} (+${}) ****", self.player.balance, insurance);
        }
        
        if self.player.insurance && !self.dealer.hand.is_blackjack() && split == false {
            println!("\nInsurance Loses ${}", insurance);
            println!("Balance: ${} (-${})", self.player.balance, insurance);
        }
        
        match result {
            RoundResult::Win => {
                if self.player.hand.is_blackjack() {
                    self.player.balance += bet * 3 / 2;
                    println!("\n**** BLACKJACK! YOU WIN ${} ****", bet * 3 / 2);
                    println!("**** Balance: ${} (+${}) ****", self.player.balance, bet * 3 / 2);
                } else {
                    self.player.balance += bet;
                    println!("\n**** YOU WIN ${} ****", bet);
                    println!("**** Balance: ${} (+${}) ****", self.player.balance, bet);
                }
            },
            RoundResult::Loss => {
                self.player.balance -= bet;
                println!("\nDealer Wins!");
                println!("Balance: ${} (-${})", self.player.balance, bet);
            },
            RoundResult::Push => {},
        }
    }
    
    pub fn get_round_result(&self, player: &Hand, dealer: &Hand) -> RoundResult {
        let player_value = player.get_value();
        let dealer_value = dealer.get_value();

        return if player_value > 21 {
            RoundResult::Loss
        } else if dealer_value > 21 {
            RoundResult::Win
        } else if player_value > dealer_value {
            RoundResult::Win
        } else if player_value < dealer_value {
            RoundResult::Loss
        } else if dealer.is_blackjack() && !player.is_blackjack() {
            RoundResult::Loss
        } else if player.is_blackjack() && !dealer.is_blackjack() {
            RoundResult::Win
        } else {
            RoundResult::Push
        };
    }

    pub fn request_wager(&mut self) -> i32 {
        // get last bet
        let mut last_round: Option<&Round> = None; 
        
        if self.player.history.len() > 0 {
            last_round = Some(&self.player.history[self.player.history.len() - 1]);
        }
        
        match last_round {
            Some(round) => {
                if round.split {
                    println!("Last bet: (${}) SPLIT ${} {} ${} {}", round.payout, round.bet,
                             match round.result {
                                 RoundResult::Win => { "WIN".to_string() }
                                 RoundResult::Loss => { "LOSS".to_string() }
                                 RoundResult::Push => { "PUSH".to_string() }
                             },
                            round.split_bet,
                             match round.split_result {
                                 RoundResult::Win => { "WIN".to_string() }
                                 RoundResult::Loss => { "LOSS".to_string() }
                                 RoundResult::Push => { "PUSH".to_string() }
                             },
                    );
                } else {
                    println!("Last bet: (${}) ${} {}", round.payout, round.bet,
                             match round.result {
                                 RoundResult::Win => { "WIN".to_string() }
                                 RoundResult::Loss => { "LOSS".to_string() }
                                 RoundResult::Push => { "PUSH".to_string() }
                             });
                }
            },
            None => {},
        }
        println!("Enter your bet:");
        
        loop {
            let bet = input::read_money();
    
            if bet > self.player.balance {
                clear_lines(2);
                println!("Enter your bet: You don't have enough money");
            } else if bet <= 0 {
                clear_lines(2);
                println!("Enter your bet: Invalid input");
            } else {
                self.player.bet = bet;
                break;
            }
        }
        
        self.player.bet
    }
}