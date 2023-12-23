fn main() {
    let greeting = String::from("Bom, dia!");

    let adjective = get_adjective(&greeting);
    let time = get_time(&greeting);

    println!("Hello, world!");
    println!("The adjective is {adjective}");
    println!("And the time is {time}");
}

fn get_adjective(adjective: &String) -> &str {
    let greeting_bytes = adjective.as_bytes();
    for (i, &item) in greeting_bytes.iter().enumerate() {
        if item == b' ' {
            return &adjective[0..i];
        }
    }

    &adjective[..]
}

fn get_time(time: &String) -> &str {
    let time_bytes = time.as_bytes();
    for (i, &item) in time_bytes.iter().enumerate() {
        if item == b' ' {
            return &time[i + 1..];
        }
    }
    &time[..]
}
