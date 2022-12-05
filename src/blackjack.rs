use crate::blackjack::card::ValueToString;

use self::{deck::Deck, hand::Hand};



#[path = "lib/card.rs"] mod card;
#[path = "lib/deck.rs"] mod deck;
#[path = "lib/hand.rs"] mod hand;
pub struct BlackJack {
    deck: Deck,
    player_hand: Hand,
    dealer_hand: Hand,
}

impl BlackJack {
    pub fn new() -> BlackJack {
        BlackJack {
            deck: Deck::new(),
            player_hand: Hand::new(),
            dealer_hand: Hand::new(),
        }
    }

    pub fn deal(&mut self) {
        self.deck.shuffle_deck();
        self.player_hand.add_card(self.deck.get_top_card());
        self.dealer_hand.add_card(self.deck.get_top_card());
        self.player_hand.add_card(self.deck.get_top_card());
        self.dealer_hand.add_card(self.deck.get_top_card());
    }

    pub fn get_player_value(& self) -> u32 {
        let mut value = 0;
        for i in 0..self.player_hand.size() {
            let card = self.player_hand.get_card(i);
            value += card.value;
        }
        value
    }

    pub fn get_dealer_value(& self) -> u32 {
        let mut value = 0;
        for i in 0..self.dealer_hand.size() {
            let card = self.dealer_hand.get_card(i);
            value += card.value;
        }
        value
    }

    pub fn deal_player(&mut self) {
        self.player_hand.add_card(self.deck.get_top_card());
    }

    pub fn deal_dealer(&mut self) {
        self.dealer_hand.add_card(self.deck.get_top_card());
    }

    pub fn print_hand(& self) {
        println!("Your hand:");
        for i in 0..self.player_hand.size() {
            let card = self.player_hand.get_card(i);
            println!("{} of {}", card.value_to_string(), card.suite);
        }
    }

    pub fn print_dealer_hand(& self) {
        println!("Dealer hand:");
        for i in 0..self.dealer_hand.size() {
            let card = self.dealer_hand.get_card(i);
            println!("{} of {}", card.value_to_string(), card.suite);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let mut game = BlackJack::new();
        game.deal();
        assert_eq!(game.player_hand.size(), 2);
        assert_eq!(game.dealer_hand.size(), 2);
        assert_eq!(game.deck.size(), 48);
        game.deal_dealer();
        assert_eq!(game.dealer_hand.size(), 3);
        assert_eq!(game.deck.size(), 47);
        assert_eq!(game.player_hand.size(), 2);

    }

    #[test]
    fn test_get_player_value() {
        let mut game = BlackJack::new();
        game.deal();
        let cur_value = game.get_player_value();
        game.deal_player();
        let next_value = game.get_player_value();
        assert!(cur_value < next_value);
        game.deal_player();
        let next_next_value = game.get_player_value();
        assert!(cur_value < next_value && next_value < next_next_value);
    }

    #[test]
    fn test_get_dealer_value() {
        let mut game = BlackJack::new();
        game.deal();
        let cur_value = game.get_dealer_value();
        game.deal_dealer();
        let next_value = game.get_dealer_value();
        assert!(cur_value < next_value);
        game.deal_dealer();
        let next_next_value = game.get_dealer_value();
        assert!(cur_value < next_value && next_value < next_next_value);
    }
}