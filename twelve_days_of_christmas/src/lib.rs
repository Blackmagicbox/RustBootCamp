pub fn sing() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let mut count = 0;
    let gifts = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];
    for day in days.iter() {
        println!("On the {day} day of Christmas my true love sent to me:");
        for number in 0..count + 1 {
            if number > 0 && number < gifts.len() - 1 {
                println!("{gift},", gift = gifts[number]);
            } else {
                println!("{gift}", gift = gifts[number]);
            }
        }
        count += 1;
    }
}

pub fn try_to_reference_two_variables() -> String {
    let mut s1 = String::from("hello");
    let s2 = &mut s1;
    s2.push_str(", world!");
    println!("{s2}");
    s2.to_string()
}
