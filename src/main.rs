// use blackjack::BlackJack;
use crate::blackjack::BlackJack;
pub mod blackjack;

fn over_limit(value: u32) -> bool {
    value > 21
}

fn handle_player_turn(blackjack: &mut BlackJack) -> String {
    blackjack.print_hand();
    println!("(d)eal or (s)tay?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "d" {
        blackjack.deal_player();
    } else if input.trim() != "s" {
        println!("Invalid input");
    }
    input.trim().to_string()
}

fn handle_dealer_turn(blackjack: &mut BlackJack) {
    blackjack.print_hand();
    let mut value = blackjack.get_dealer_value();
    while value < 17 {
        blackjack.deal_dealer();
        value = blackjack.get_dealer_value();
    }
}

fn show_results(game: & BlackJack) {

    println!("Player value: {}", game.get_player_value());
    println!("Dealer value: {}", game.get_dealer_value());
    game.print_hand();
    game.print_dealer_hand();
}

fn main() {
    let mut game = BlackJack::new();
    game.deal();

    while !over_limit(game.get_player_value()) {
        if handle_player_turn(& mut game) == "s" {
            break;
        }
    }

    if over_limit(game.get_player_value()) {
        println!("You lose!");
        game.print_hand();
        return;
    }

    handle_dealer_turn(& mut game);
    if over_limit(game.get_dealer_value()) {
        println!("You win!");
    } else if game.get_player_value() > game.get_dealer_value() {
        println!("You win!");
    } else {
        println!("You lose!");
    }
    println!("Results:");
    show_results(& game);
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_over_limit() {
        let mut game = BlackJack::new();
        game.deal();

    }
}