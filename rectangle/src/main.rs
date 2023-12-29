use rectangle::Rectangle;

fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("The area of the rectangle is {area}.", area = rect1.area());
    println!(
        "Can rect1 hold rect2? {answer}.",
        answer = rect1.can_hold(&rect2)
    );
    println!(
        "Can rect1 hold rect3? {answer}.",
        answer = rect1.can_hold(&rect3)
    );
}
