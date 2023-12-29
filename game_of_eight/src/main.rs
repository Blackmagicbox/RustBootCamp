fn main() {

    // Declare a mutable variable to hold the initial number
    let mut initial_number = 8;

    // Print the welcome message
    println!("Welcome to the game of eight!");
    println!("Please enter a number between 1 and 10");

    // It's a two players number;
    // Get the user input and convert it to a number,
    // the input must be number between 1 and 3;
    // Any number greater than 3 is not allowed;
    // If the input is not a number, or outside the range from 1 to 3, print an error message and repeat the input;
    // The following player cannot say the same number as the previous player;
    // If the input is a number, subtract it from the initial number;
    // If the result is 0, print a congratulation message and exit the program;
}
