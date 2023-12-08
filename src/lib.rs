fn extract_calibration(line: &str) -> Option<u32>{
    let mut first_val:Option<u32> = None;
    let mut last_val:Option<u32> = None;

    for c in line.chars() {
        match c.to_digit(10) {
            Some(x) => {
                if first_val == None {
                    first_val = Some(x);
                    last_val = Some(x);
                } else {
                    last_val = Some(x);
                }
            },
            None => continue,

        }
    }

    let formatted = match first_val {
        Some(x) => Some(first_val.unwrap() * 10 + last_val.unwrap()),
        None => None
    };

    formatted
}

#[test]
fn it_reads_correctly() {
    assert_eq!(12, extract_calibration("1abc2").unwrap());
}

#[test]
fn it_handles_multiple_digits(){
    assert_eq!(12, extract_calibration("a12b3n2m").unwrap());
}

#[test]
fn it_returns_none(){
    assert_eq!(None,extract_calibration("abcdefg"));
}

pub fn calculate_calibration(file_contents: Vec<&str>) -> u32 {
    let mut total:u32 = 0;

    for line in file_contents {
        match extract_calibration(line){
            Some(x) => total += x,
            None => continue,
        };
    }
    total
}

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_blue: u32,
    max_green: u32,
    valid: bool,
}

impl Game {
    fn new() -> Game{
        Game{
            id: 0,
            max_red: 0,
            max_blue: 0,
            max_green: 0,
            valid: false
        }
    }

    fn check_game(&mut self) {
        if self.max_red > 12 || self.max_green > 13 || self.max_blue > 14{
            self.valid = false;
        } else {
            self.valid = true;
        }
    }
    

    fn calculate_power(&self) -> u32 {
        return self.max_blue * self.max_green * self.max_red;
    }
}

fn read_line(line: &str) -> Game{

    let mut current_game = Game::new();

    let segments: Vec<&str> = line.rsplit(|p| p ==';'|| p==':').collect();

    for segment in segments {
        if segment.contains("Game") {
            current_game.id = segment.strip_prefix("Game ").unwrap().parse().unwrap();
        } else {
            let sub_segments: Vec<&str> = segment.trim().split(",").collect();
            for element in sub_segments {
                let arr: Vec<&str>  = element.trim().split(" ").collect(); 
                match *arr.get(1).unwrap() {
                    "red" => {
                        if current_game.max_red < arr.get(0).unwrap().parse().unwrap(){
                            current_game.max_red = arr.get(0).unwrap().parse().unwrap();
                        }
    
                    },
                    "blue" => {
                        if current_game.max_blue < arr.get(0).unwrap().parse().unwrap(){
                            current_game.max_blue = arr.get(0).unwrap().parse().unwrap();
                        }

                    },
                    "green" => {
                        if current_game.max_green < arr.get(0).unwrap().parse().unwrap(){
                            current_game.max_green = arr.get(0).unwrap().parse().unwrap();
                        }

                    },
                    _ => continue,
                }
            }
        }

    }
    current_game.check_game();
    current_game

}


pub fn calculate_ids (lines: Vec<&str>) ->  (u32, u32){
    let mut games: Vec<Game> = Vec::new();
    for line in lines {
        games.push(read_line(line));
        
    }
    let mut id_total = 0;
    let mut power_total = 0;
    
    for game in games {
        if game.valid {
            id_total += game.id;
        }

        power_total += game.calculate_power();
    }

    (id_total, power_total)
}

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

