use crate::blackjack::card::ValueToString;

use self::{deck::Deck, hand::Hand};



#[path = "lib/card.rs"] mod card;
#[path = "lib/deck.rs"] mod deck;
#[path = "lib/hand.rs"] mod hand;
pub struct BlackJack {
    deck: Deck,
    player_hand: Vec<Hand>
}

impl BlackJack {
    pub fn new(players : u32) -> BlackJack {
        let mut player_hands = Vec::new();
        for i in 0..players {
            player_hands.push(Hand::new());
        }
        BlackJack {
            deck: Deck::new(),
            player_hand: player_hands,
        }
    }

    pub fn player_count(&self) -> usize {
        self.player_hand.len()
    }

    pub fn deal(&mut self) {
        self.deck.shuffle_deck();
        for i in 0..self.player_hand.len() {
            self.player_hand[i].add_cards(2, &mut self.deck);
        }

    }

    pub fn get_player_value(& self, index : usize) -> u32 {
        let mut value = 0;
        let hand = &self.player_hand[index];
        for i in 0..hand.size() {
            let card = hand.get_card(i);
            if card.value > 10 {
                value += 10;
            } else {
                value += card.value;
            }
        }
        value
    }

    pub fn deal_player(&mut self, index : usize) {
        self.player_hand[index].add_card(self.deck.get_top_card());
    }


    pub fn print_hand(& self, index : usize) {
        println!("Player {index}'s hand:");
        let hand = &self.player_hand[index];
        for i in 0..self.player_hand[index].size() {
            let card = self.player_hand[index].get_card(i);
            println!("{} of {}", card.value_to_string(), card.suite);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let mut game = BlackJack::new(5);
        game.deal();
        let expected = 52 - 5;
        assert_eq!(game.deck.size(), expected);


    }

    #[test]
    fn test_get_player_value() {
        let mut game = BlackJack::new(3);
        game.deal();
        let cur_value = game.get_player_value(0);
        game.deal_player(0);
        let next_value = game.get_player_value(0);
        assert!(cur_value < next_value);
        game.deal_player(0);
        let next_next_value = game.get_player_value(0);
        assert!(cur_value < next_value && next_value < next_next_value);
    }
}