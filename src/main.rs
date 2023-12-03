use std::fs;

use advent_of_code::{calculate_calibration, calculate_ids};

fn main() {
    // Day One
    // let calibration_figures = fs::read_to_string("./advent_of_code_input_files/calibration_figures.txt").expect("Should have been able to read file");
    // let lines: Vec<&str> = calibration_figures.lines().collect();
    // let total = calculate_calibration(lines);
    // println!("{}", total);

    //Day Two

    let game_figures = fs::read_to_string("./advent_of_code_input_files/game_figures.txt").expect("Unable to read file");
    let lines: Vec<&str> = game_figures.lines().collect();
    let (id_total , power_total)= calculate_ids(lines);
    println!("ID Total: {id_total}\nPower Total: {power_total}");

}

