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


pub fn calculate_calibration_two(file_contents: String) -> u32 {
    let formatted = file_contents.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    println!("{formatted}");

    let lines :Vec<&str> = formatted.lines().collect();
    return calculate_calibration(lines);

    0
}