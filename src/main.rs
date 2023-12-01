use std::fs;

fn main() {
    let calibration_figures = fs::read_to_string("./calibration_figures.txt").expect("Should have been able to read file");
    println!("{}", calibration_figures);

}

