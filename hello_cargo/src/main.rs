fn square_root(x: f64) -> f64 {
    return x.sqrt();
}

fn main() {
    println!(
        "Hello, cargo! {square_root}",
        square_root = square_root(9.0)
    );
}
