
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