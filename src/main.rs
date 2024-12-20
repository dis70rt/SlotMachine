use colored::Colorize;
use input_rust::input;

/*
1. Get Deposit Amount
2. Get Number of lines to bet on. In this case we will take MAX of 3.
3. Get Amount to Bet per Line.
4. Crete a set of Symbols, No. of Symbols, Value of each symbols
5. Display the Slot outcome.
6. Check Winnings + Update the Balance
---------END--------
*/
mod operation;
fn main() {
    let mut balance = operation::get_deposit();

    while balance > 0 {
        println!("{}{}", "Balance: $".green(), balance.to_string().green());
        let lines = operation::get_betline();
        let bet = operation::get_bet_amount(balance, lines);
        balance -= bet*(lines as u32);

        let winnings = operation::slot_machine(lines, bet);
        println!("You won {}{}", "$".yellow(), winnings.to_string().yellow());
        balance += winnings;

        let play_again = input("Do you want to play Again (y/n)? ").unwrap();
        if play_again.to_lowercase() != "y" {break;}
    }

    if balance == 0 {
        println!("{}", "Oops! You are broke now".yellow())
    } else {
        println!("{}{}", "Available Balance: $".cyan(), balance.to_string().cyan());
    }
}
