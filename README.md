# Slot Machine Game in Rust 🎰

This is a simple terminal-based slot machine game built with Rust, created to learn and practice Rust basics. The game allows players to deposit money, place bets, and spin the slot machine to win (or lose) their balance.

---

## Features
1. **Deposit Money:** Start the game by depositing an initial amount.
2. **Bet on Lines:** Choose the number of lines to bet on (maximum of 3).
3. **Set Bet Amount:** Decide how much to bet per line.
4. **Slot Machine Symbols:** Spin the slot machine and see the outcome.
5. **Calculate Winnings:** Check if you've won and update your balance accordingly.
6. **Play Again Option:** Decide whether to continue playing or exit the game.

---

## Why This is a Good Code for Learning Rust Basics
This slot machine game demonstrates several key Rust concepts that are essential for beginners:

1. **Variables and Data Types:**  
   - The code utilizes variables with types such as `u32` and `String`, providing an opportunity to practice type annotations and basic data types in Rust.

2. **Control Flow:**  
   - The game uses conditional statements (`if`/`else`) and loops (`while`), which are fundamental concepts for any Rust programmer. This allows learners to understand how to control the flow of a program based on conditions and repeated execution.

3. **Input Handling:**  
   - The `input_rust::input` function demonstrates how to capture user input, which is essential for interactive Rust programs. Handling user input is a critical skill in Rust, and this example shows how to get and process string input.

4. **Functions and Modularization:**  
   - The code is modular, with different parts of the game encapsulated in separate functions (e.g., `get_deposit`, `get_betline`, `slot_machine`). This teaches beginners how to organize their code into smaller, reusable units of functionality, improving code clarity and maintainability.

5. **Ownership and Borrowing:**  
   - While not deeply complex, the code introduces ownership, a core concept of Rust, when dealing with variables like `balance` and `winnings`. Understanding ownership is crucial for managing memory safely in Rust.

6. **Error Handling:**  
   - Basic error handling is demonstrated with input validation. The `unwrap()` function is used, which simplifies the error handling for beginners but can later be replaced with more advanced techniques like `Result` for handling errors more gracefully.

7. **Libraries and Crates:**  
   - The project introduces how to work with external libraries (or "crates") such as `colored` and `input-rust`. This helps beginners learn how to manage dependencies and integrate third-party tools into their projects.

---

## How to Run
1. **Clone the Repository:**
   ```bash
   git clone https://github.com/your-username/slot-machine-game.git
   cd slot-machine-game
   ```

2. **Install Rust:**  
   Make sure you have Rust installed. You can install it from [rust-lang.org](https://www.rust-lang.org/).

3. **Run the Game:**
   ```bash
   cargo run
   ```

---

## How to Play
1. **Deposit Money:**  
   Enter the amount you want to deposit.

2. **Place Your Bets:**  
   - Choose the number of lines to bet on (1-3).
   - Enter the bet amount per line.

3. **Spin the Slot Machine:**  
   Watch the slot machine outcome and check your winnings.

4. **Play Again or Exit:**  
   Decide whether to play another round or exit with your balance.

---

## Dependencies
- [colored](https://crates.io/crates/colored) - For colorful terminal output.
- [input-rust](https://crates.io/crates/input-rust) - For input handling.

---

## Contribution
This project was created as a learning exercise for Rust basics. However, feel free to fork this repository and contribute to the project. Pull requests are welcome! 🎉

---

## License
This project is licensed under the MIT License.
