use std::ops::Deref;

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
    red: u32,
    blue: u32,
    green: u32
}

fn read_line(line: &str) -> Option<Game>{

    let mut current_game = Game { id: 0, red: 0, blue: 0, green: 0 };

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
                        if(current_game.red < arr.get(0).unwrap().parse().unwrap()){
                            current_game.red = arr.get(0).unwrap().parse().unwrap();
                        }
    
                    },
                    "blue" => {
                        if(current_game.blue < arr.get(0).unwrap().parse().unwrap()){
                            current_game.blue = arr.get(0).unwrap().parse().unwrap();
                        }
                    },
                    "green" => {
                        if(current_game.green < arr.get(0).unwrap().parse().unwrap()){
                            current_game.green = arr.get(0).unwrap().parse().unwrap();
                        }
                    },
                    _ => continue,
                }
            }
        }

    }
    
    println!("{:?}", current_game);

    match check_game(&current_game) {
        true => Some(current_game),
        false => None 
    }
}

fn check_game(current_game: &Game, ) -> bool {
    true
}

pub fn calculate_ids (lines: Vec<&str>) ->  u32{
    for line in lines {
        read_line(line);
    }
    
    0
}