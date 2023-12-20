use area_calculator::{area_calculator, print_coords_difference, return_unit, volume_calculator};

fn main() {
    let width = 30;
    let height = 50;
    let depth = 10;

    let area = area_calculator(width, height);
    let volume = volume_calculator(width, height, depth);
    println!(
        "The area of {width}x{height} is {area}{unit}",
        unit = area_calculator::return_unit(2)
    );
    println!(
        "and the volume of {width}x{height}x{depth} is {volume}{unit} \n",
        unit = return_unit(3)
    );

    let (x, y) = (-10, 20);
    let diff = print_coords_difference(x, y);

    println!("The difference between {x} and {y} is {diff}");
}
