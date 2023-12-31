fn main() {
    println!("Ask me to do something!");

    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        if &arg == "sum" {
            sum();
        } else if &arg == "double" {
            double();
        } else {
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;

    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times before it is larger than 500",
        count
    )
}

fn count(_arg: String) {
    let mut count = 0;

    loop {
        if count == 8 {
            break;
        }
        count += 1;
        println!("Say {} what?", &_arg);
    }
}
