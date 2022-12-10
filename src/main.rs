// use blackjack::BlackJack;
use crate::blackjack::BlackJack;
pub mod blackjack;

const MAIN_PLAYER : usize = 0;

fn over_limit(value: u32) -> bool {
    value > 21
}

fn handle_player_turn(blackjack: &mut BlackJack) -> String {
    blackjack.print_hand(MAIN_PLAYER);
    println!("(d)eal or (s)tay?");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "d" {
        blackjack.deal_player(MAIN_PLAYER);
    } else if input.trim() != "s" {
        println!("Invalid input");
    }
    input.trim().to_string()
}

fn handle_computers(blackjack: &mut BlackJack) {
    for i in 1..blackjack.player_count() {
        while blackjack.get_player_value(i) < 17 {
            blackjack.deal_player(i);
        }
    }
}

fn compute_winner(blackjack: &BlackJack) -> usize {
    let mut winner = 0;
    let mut winner_value = 0;
    for i in 0..blackjack.player_count() {
        let value = blackjack.get_player_value(i);
        if value > winner_value && !over_limit(value) {
            winner = i;
            winner_value = value;
        }
    }
    winner
}

fn show_results(game: & BlackJack) {
    let winner = compute_winner(game);
    println!("Player {} wins!", winner);
    for i in 0..game.player_count() {
        println!("Player {}'s hand:", i);
        game.print_hand(i);
    }
}


fn main() {
    println!("Welcome to BlackJack!");
    println!("how many players? (1-4)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let player_count = input.trim().parse::<u32>().unwrap();
    let mut game = BlackJack::new(player_count);

    game.deal();

    while !over_limit(game.get_player_value(MAIN_PLAYER)) {
        if handle_player_turn(& mut game) == "s" {
            break;
        }
    }

    if over_limit(game.get_player_value(MAIN_PLAYER)) {
        println!("You lose!");
        game.print_hand(MAIN_PLAYER);
        return;
    }

    handle_computers(& mut game);

    show_results(& game);
    

}