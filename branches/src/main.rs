fn main() {
    println!("Hello, world!");
    is_smaller_than_five(1);
    let number = 3;
    println!(
        "{number} is a six: {six_or_not}",
        six_or_not = is_six(number)
    );

    let mut i = 0;
    loop {
        if i > 2 {
            break;
        }
        println!("again!{i}");
        i += 1;
    }
}

fn is_smaller_than_five(x: i32) {
    if x < 5 {
        println!("{} is smaller than five", x);
    } else {
        println!("{} is not smaller than five", x);
    }
}

fn is_six(y: i32) -> bool {
    y == 6
}
