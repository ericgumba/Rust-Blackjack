use crate::blackjack::card::Card;

use super::deck::Deck;


pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: Vec::new() }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }


    pub fn add_cards(&mut self, amount: usize, deck: &mut Deck) {
        for i in 0..amount {
            self.add_card(deck.get_top_card());
        }
    }

    pub fn get_card(& self, index: usize) -> Card {
        return self.cards[index].clone();
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }
}
