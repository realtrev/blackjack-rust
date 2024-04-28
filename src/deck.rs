use crate::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let cards = Card::new_deck();
        Deck { cards }
    }
    
    pub fn from(cards: Vec<Card>) -> Deck {
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards = Card::new_deck();

        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
