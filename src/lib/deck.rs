use rand::Rng;

use crate::blackjack::card::{Card, ValueToString};

pub mod card;
// create a structure called deck with a vector of cards
pub struct Deck {
    cards: Vec<Card>,
}

// create a function that prints the deck of cards
pub fn print_deck(deck: &Deck) {
    for card in &deck.cards {
        println!("{} of {}", card.value_to_string(), card.suite);
    }
}


// shuffle the deck

impl Deck {
    
    fn create_deck() -> Deck {
        let mut cards = Vec::new();
        let suites = vec!["Hearts", "Diamonds", "Spades", "Clubs"];
        for suite in suites {
            for value in 1..14 {
                cards.push(Card {
                    suite: suite.to_string(),
                    value: value,
                });
            }
        }
        Deck { cards: cards }
    }

    pub fn new() -> Deck {
        Deck::create_deck()
    }

    pub fn get_top_card(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
    pub fn swap_cards(& mut self, index1: usize, index2: usize) {
        let temp = self.cards[index1].clone();
        self.cards[index1] = self.cards[index2].clone();
        self.cards[index2] = temp;
    }

    pub fn shuffle_deck(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cards.len() {
            let random_index = rng.gen_range(0, self.cards.len());
            self.swap_cards(i, random_index);
        }
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }
}
