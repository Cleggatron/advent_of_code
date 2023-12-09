


pub fn calculate_points(lines: Vec<&str>) -> u32 {
    let mut total = 0;

    for line in lines {
        total += calculate_scratchcard_points(line);
        
    }
    total
}

use std::{collections::HashSet, ops::Deref};

fn calculate_scratchcard_points(line: &str) -> u32{

    let segments :Vec<&str>= line.split(|c| c == ':' || c == '|').collect();

    let mut winning_string: Vec<&str> = segments.get(1).unwrap().deref().split_ascii_whitespace().collect();
    let mut ticket_string: Vec<&str> = segments.get(2).unwrap().deref().split_ascii_whitespace().collect();


    let winning_string_nums:HashSet<u32> = winning_string.into_iter().map(|x| x.parse::<u32>().unwrap()).collect();
    let ticket_nums:HashSet<u32> = ticket_string.into_iter().map(|x| x.parse::<u32>().unwrap()).collect();

    let commonalities:HashSet<&u32> = winning_string_nums.intersection(&ticket_nums).collect();


    if commonalities.len() > 0 {
        return u32::pow(2, (commonalities.len() as u32)-1);
    }
    0

}
