pub fn return_unit(exp: u32) -> String {
    let string = format!("m{exp}");
    string
}

pub fn print_coords_difference(x: i32, y: i32) -> i32 {
    y.abs_diff(x) as i32
}

pub fn volume_calculator(width: u32, height: u32, depth: u32) -> u32 {
    area_calculator(width, height) * depth
}

pub fn area_calculator(width: u32, height: u32) -> u32 {
    width * height
}
