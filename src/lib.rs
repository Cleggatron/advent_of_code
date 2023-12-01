fn extract_calibration(line: &str) -> u32{
    let mut first_val:Option<u32> = None;
    let mut last_val:Option<u32> = None;

    for c in line.chars() {
        if c.is_digit(10) {
            match first_val {
                None => {
                    first_val = Some(c.to_digit(10).unwrap());
                    last_val = Some(c.to_digit(10).unwrap());
                },
                Some(_) => last_val = Some(c.to_digit(10).unwrap())
            }
        }
    }
    
    first_val.unwrap() * 10 + last_val.unwrap()
}

#[test]
fn it_reads_correctly() {
    assert_eq!(12, extract_calibration("1abc2"));
}

#[test]
fn it_handles_multiple_digits(){
    assert_eq!(12, extract_calibration("a12b3n2m"));
}

#[test]
fn it_crashes(){
    extract_calibration("abcdefg");
}