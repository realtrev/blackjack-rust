use std::fmt::Display;

trait Numeric {
    fn from_int(int: u8) -> Self;
    fn as_int(&self) -> u8;
}

#[derive(Copy, Clone)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Numeric for Rank {
    fn from_int(int: u8) -> Rank {
        match int {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }

    fn as_int(&self) -> u8 {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Rank::Two => write!(f, "2"),
            Rank::Three => write!(f, "3"),
            Rank::Four => write!(f, "4"),
            Rank::Five => write!(f, "5"),
            Rank::Six => write!(f, "6"),
            Rank::Seven => write!(f, "7"),
            Rank::Eight => write!(f, "8"),
            Rank::Nine => write!(f, "9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, "Jack"),
            Rank::Queen => write!(f, "Queen"),
            Rank::King => write!(f, "King"),
            Rank::Ace => write!(f, "Ace"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum Suit {
    Clubs = 1,
    Diamonds = 2,
    Hearts = 3,
    Spades = 4,
}

impl Numeric for Suit {
    fn from_int(int: u8) -> Suit {
        match int {
            1 => Suit::Clubs,
            2 => Suit::Diamonds,
            3 => Suit::Hearts,
            4 => Suit::Spades,
            _ => panic!("Invalid suit"),
        }
    }

    fn as_int(&self) -> u8 {
        match self {
            Suit::Clubs => 1,
            Suit::Diamonds => 2,
            Suit::Hearts => 3,
            Suit::Spades => 4,
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Suit::Clubs => write!(f, "Clubs"),
            Suit::Diamonds => write!(f, "Diamonds"),
            Suit::Hearts => write!(f, "Hearts"),
            Suit::Spades => write!(f, "Spades"),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn from_int(rank: u8, suit: u8) -> Card {
        Card {
            rank: Rank::from_int(rank),
            suit: Suit::from_int(suit),
        }
    }

    pub fn to_string(&self) -> String {
        format!("{} of {}", self.rank, self.suit)
    }

    pub fn get_value(&self) -> u8 {
        self.rank.as_int()
    }
    
    pub fn new_deck() -> Vec<Card> {
        let mut deck = Vec::new();
        for suit in 1..=4 {
            for rank in 2..=14 {
                deck.push(Card::from_int(rank, suit));
            }
        }
        deck
    }
}
