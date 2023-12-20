pub fn return_unit(exp: u32) -> String {
    let string = format!("m{exp}");
    string
}

pub fn print_coords_difference(x: i32, y: i32) -> i32 {
    y.abs_diff(x) as i32
}
