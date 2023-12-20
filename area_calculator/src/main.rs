#! allow(unused_variables)

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
        unit = area_calculator::return_unit(3)
    )
}

fn volume_calculator(width: u32, height: u32, depth: u32) -> u32 {
    area_calculator(width, height) * depth
}

fn area_calculator(width: u32, height: u32) -> u32 {
    width * height
}
