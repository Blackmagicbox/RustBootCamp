use std::{io, process};

fn main() {
    // Declare a mutable variable to hold the initial number
    let initial_number = 8;
    let mut current_number = initial_number;

    let mut turn = 1;

    // Print the welcome message
    println!("Welcome to the game of 🎱! (Eights)");

    let mut last_number_typed = 0;

    loop {
        let player = if turn == 1 { 1 } else { 2 };
        let mut input = String::new();
        println!("🕹️ Player-{} Enter a number", player);
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q" {
            break;
        }

        let input_num: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number, please enter a number");
                continue;
            }
        };

        if input_num > 3 || input_num < 1 {
            println!("You need to enter a number between 1 and 3 you lost your turn! 🤦🏻‍♂️");
            turn ^= 1;
            continue;
        }

        if input_num == last_number_typed {
            println!("You cannot enter the same number of the previous player");
            continue;
        }

        if current_number - input_num == 0 {
            println!("\n🕹️ Player-{} won!🎉", player);
            println!("The initial number was {}", initial_number);
            break;
        }

        if current_number - input_num < 0 {
            println!("Too high lost your turn!  🤦🏻‍♂️");
            turn ^= 1;
            continue;
        }
        last_number_typed = input_num;
        current_number -= input_num;
        turn ^= 1;
    }

    process::exit(0)
}
