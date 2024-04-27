use crate::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let cards = Deck::new_deck();
        Deck { cards }
    }
    
    pub fn shuffle(&mut self) {
        self.cards = Deck::new_deck();
        
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    
    pub fn new_deck() -> Vec<Card> {
        let mut cards = Vec::new();
        for rank in 2..=14 {
            for suit in 1..=4 {
                cards.push(Card::from_int(rank, suit));
            }
        }
        cards
    }
    
    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}