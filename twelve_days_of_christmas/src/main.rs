use twelve_days_of_christmas::{sing, try_to_reference_two_variables};

fn main() {
    let date = chrono::Local::now();
    println!("Today is {}", date.format("%A, %B %e, %Y"));
    println!("Let's sing a song!\nThe twelve days of Christmas\n");
    sing();

    let s = try_to_reference_two_variables();
    println!("{s}");
}
