use std::{fs, ops::Deref};

use advent_of_code::day_one::{calculate_calibration, calculate_calibration_two};
use advent_of_code::day_two::calculate_ids;
use advent_of_code::day_four::calculate_points;
use advent_of_code::day_six::{calculate_permutations, calculate_permutations_two};



fn main() {
    // Day One
    let calibration_figures = fs::read_to_string("./advent_of_code_input_files/day_1.txt").expect("Should have been able to read file");
    //let lines: Vec<&str> = calibration_figures.lines().collect();
    let total = calculate_calibration_two(calibration_figures);
    println!("{}", total);

    //Day Two

    // let game_figures = fs::read_to_string("./advent_of_code_input_files/day_2.txt").expect("Unable to read file");
    // let lines: Vec<&str> = game_figures.lines().collect();
    // let (id_total , power_total)= calculate_ids(lines);
    // println!("ID Total: {id_total}\nPower Total: {power_total}");


    //Day Three

    //ToDo

    //Day Four

    // let scratchcard_data = fs::read_to_string("./advent_of_code_input_files/day_4.txt").expect("Unable to read file");
    // let lines :Vec<&str> = scratchcard_data.lines().collect();
    // let mut total = 0;
    // total += calculate_points(lines);
    // println!("Total points: {total}")

    //Day Five

    //Todo

    //Day Six

    // let speed_distance_data = fs::read_to_string("./advent_of_code_input_files/day_6.txt").expect("Unable to read file");
    // let lines :Vec<&str> = speed_distance_data.lines().collect();
    // let times:Vec<u32> = lines.get(0).unwrap().deref().trim_start_matches("Time:").split_ascii_whitespace().into_iter().map(|num :&str| num.parse::<u32>().unwrap()).collect();
    // let distances:Vec<u32> = lines.get(1).unwrap().deref().trim_start_matches("Distance:").split_ascii_whitespace().into_iter().map(|num :&str| num.parse::<u32>().unwrap()).collect();

    // let mut total = 1;

    // for num in 0..times.len(){
    //     total *= calculate_permutations(*times.get(num).unwrap(), *distances.get(num).unwrap());
    // }

    // print!("{total}");
    // let time: usize = 56717999;
    // let distance: usize = 334113513502430;
    // let total = calculate_permutations_two(time, distance);
    // println!("{total}");

    //Day Seven

    // let data = fs::read_to_string("./advent_of_code_input_files/day_7.txt").expect("Unable to read file");
    // let lines: Vec<&str> = data.lines().collect();
    

    // println!("{:?}", lines);
}

