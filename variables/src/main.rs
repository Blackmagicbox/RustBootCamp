fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;
    let missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT;

    println!("Firing {ready} of my {missiles} missiles...");

    let missiles = missiles - ready;
    println!("{missiles} missiles left");
}
