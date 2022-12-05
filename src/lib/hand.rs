use crate::blackjack::card::Card;


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

    pub fn get_card(& self, index: usize) -> Card {
        return self.cards[index].clone();
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }
}
