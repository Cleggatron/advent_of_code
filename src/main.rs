use std::fs;

use advent_of_code::{calculate_calibration, calculate_ids, calculate_points};

fn main() {
    // Day One
    // let calibration_figures = fs::read_to_string("./advent_of_code_input_files/day_1.txt").expect("Should have been able to read file");
    // let lines: Vec<&str> = calibration_figures.lines().collect();
    // let total = calculate_calibration(lines);
    // println!("{}", total);

    //Day Two

    // let game_figures = fs::read_to_string("./advent_of_code_input_files/day_2.txt").expect("Unable to read file");
    // let lines: Vec<&str> = game_figures.lines().collect();
    // let (id_total , power_total)= calculate_ids(lines);
    // println!("ID Total: {id_total}\nPower Total: {power_total}");


    //Day Three

    //ToDo

    //Day Four

    let scratchcard_data = fs::read_to_string("./advent_of_code_input_files/day_4.txt").expect("Unable to read file");
    let lines :Vec<&str> = scratchcard_data.lines().collect();
    let mut total = 0;
    total += calculate_points(lines);
    println!("Total points: {total}")

}

