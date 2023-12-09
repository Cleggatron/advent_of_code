fn read_instructions<'a>(instruction: &'a str, choices: (&'a str, &'a str)) -> &'a str {
    match instruction {
        "L" => return choices.0.clone(),
        "R" => return choices.1.clone(),
        _ => return ""
    }
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