use std::collections::HashMap;
use rand::{thread_rng, Rng};

use input_rust::input;
use colored::Colorize;

pub fn get_deposit() -> u32 {
    loop {
        let amount = input("Enter your Deposit Amount $").expect("Failed to take input"); 
        match amount.parse::<u32>() {
            Ok(value) => return value,
            Err(_) => println!("{}", "[Invalid] Please enter a positive integer".red()),
        }
    }
}

pub fn get_betline() -> u8 {
    loop {
        let amount = input("Enter No. of lines you want to bet on (1-3): ")
            .expect("Failed to take input");

        match amount.parse::<u8>() {
            Ok(value) => if value >= 1 && value <= 3 {
                return  value;
            } else {
                println!("{}", "[Invalid] Please enter a integer between 1-3".red())
            },
            _ => println!("{}", "[Invalid] Please enter a valid integer".red()),
        }
    }
}

pub fn get_bet_amount(balance: u32,lines: u8) -> u32 {
    loop {
        let bet = input("Enter the Amount you want to bet per line: ")
            .expect("Failed to take input");

        match bet.parse::<u32>() {
            Ok(value) => if value <= balance / (lines as u32) {
                return  value;
            } else {
                println!("{}", "[Invalid] Your bet amount exceeds your balance and number of lines".red())
            }
            _ => println!("{}", "[Invalid] Please enter a valid positive integer".red()),
        }
    }
}

pub fn slot_machine(bet_lines:u8, bet:u32) -> u32 {
    let symbol_count: HashMap<&str, u32> = HashMap::from([("$",3),("#",5),("@",6),("%",8),("&",10)]);
    let symbol_value: HashMap<&str, u32> = HashMap::from([("$",12),("#",8),("@",6),("%",4),("&",2)]);
    const ROWS: usize = 3;
    const COLS: usize = 3;

    let mut symbols: Vec<&str> = vec![];

    for (&symbol,&count) in symbol_count.iter() {
        for _ in 0..count {
            symbols.push(symbol);
        }
    }

    let mut slots: Vec<Vec<&str>> = vec![];    

    for i in 0..ROWS {
        slots.push(vec![]);
        for _ in 0..COLS {
            let index: usize = thread_rng().gen_range(1..=10);
            slots[i].push(symbols[index]);
            symbols.remove(index);
        }
    }

    for i in 0..ROWS {
        let mut sym = String::new();
        for j in 0..COLS {
            sym.push_str(slots[i][j]);
            if j != 2 {sym.push_str(" | ")}
        }
        println!("{}",sym);
    };

    let mut winnings: u32 = 0;

    for i in 0..(bet_lines as usize) {
        if slots[i].iter().all(|&x| x == slots[i][0]) {
            winnings += bet * symbol_value[slots[i][0]];
        }
    };

    winnings
}

