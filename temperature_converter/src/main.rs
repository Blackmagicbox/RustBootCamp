use temperature_converter::{celsius_to_fahrenheit, fahrenheit_to_celsius, fibonacci};

fn main() {
    let celsius = 6.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{celsius}°C = {fahrenheit}°F");

    let fahrenheit = 42.8;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F = {}°C", celsius.ceil());
    println!("The 10th Fibonacci number is: {}", fibonacci(12));
}
