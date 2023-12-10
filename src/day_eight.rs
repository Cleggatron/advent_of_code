fn read_instructions<'a>(instruction: &'a str, choices: (&'a str, &'a str)) -> &'a str {
    match instruction {
        "L" => return choices.0.clone(),
        "R" => return choices.1.clone(),
        _ => return ""
    }
}

pub fn read_lines(lines: String){
    let mut lines: Vec<&str> = lines.lines().collect();
    let instruction_set = lines.remove(0);
    let blank_line = lines.remove(0);
    let map = build_map(lines);
    println!("{:?}", map);
}

use std::collections::HashMap;

fn build_map(lines: Vec<&str>) -> HashMap<String, (String, String)>{
    let mut map = HashMap::new();

    for line in lines {
        let segment = line.to_string().replace("(", "").replace(")", "").replace(" ","");
        let segments :Vec<&str> = segment.split("=").collect();
        println!("{:?}", segments);
        let map_key = segments.get(0).unwrap().to_owned();
        let map_entry = segments.get(1).unwrap().to_owned();
        let map_tuple = create_map_tuple(map_entry).to_owned();
        map.insert(String::from(map_key), map_tuple);
    }
    map
}


fn create_map_tuple(slice :&str) -> (String, String) {
    let halves :Vec<&str>= slice.split(",").collect();
    let left = *halves.get(0).unwrap();
    let right = *halves.get(1).unwrap();
    (String::from(left), String::from(right))

}

#[test]
fn it_returns_left(){
    let choices = ("abc", "def");
    assert_eq!(read_instructions("L", choices), "abc");
}

#[test]
fn it_returns_right(){
    let choices = ("abc", "def");
    assert_eq!(read_instructions("R", choices), "def");
}