use crate::card::{Card, Rank};
use std::ops::Add;

#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub value: u8,
    pub soft: bool,
}

impl Hand {
    pub fn new() -> Hand {
        Hand {
            cards: Vec::new(),
            value: 0,
            soft: false,
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);

        self.value = self.calculate_value();
    }

    pub(crate) fn calculate_value(&mut self) -> u8 {
        let mut value = 0;
        let mut aces = 0;

        self.soft = false;
        for card in &self.cards {
            let mut card_value = card.get_value();
            if card_value > 10 {
                card_value = 10;
                if match card.rank {
                    Rank::Ace => true,
                    _ => false,
                } {
                    card_value = 11;
                }
            }

            if match card.rank {
                Rank::Ace => true,
                _ => false,
            } {
                aces += 1;
                self.soft = true;
            }

            value += card_value;
        }

        while value > 21 && aces > 0 {
            self.soft = false;
            value -= 10;
            aces -= 1;
        }

        value
    }

    pub fn get_value(&self) -> u8 {
        self.value
    }

    pub fn display(&self) {
        
        let mut string = String::new();
        for card in &self.cards {
            string = string.add(format!("{}, ", card.to_string()).as_str());
        }
        string.remove(string.len() - 2);
        println!("  ({}) {}", self.value, string);
    }

    pub fn display_first_card(&self) {
        let mut value = self.cards[0].get_value();
        if value > 10 {
            value = 10;
            if match self.cards[0].rank {
                Rank::Ace => true,
                _ => false,
            } {
                value = 11;
            }
        }
        println!(
            "  ({}?) {}, ???\n",
            value.to_string(),
            self.cards[0].to_string()
        );
    }

    pub fn is_blackjack(&self) -> bool {
        self.cards.len() == 2 && self.value == 21
    }

    pub fn clear(&mut self) {
        self.cards.clear();
        self.value = 0;
    }
    
    pub fn is_pair(&self) -> bool {
        self.cards.len() == 2 && self.cards[0].get_value() == self.cards[1].get_value()
    }
}
