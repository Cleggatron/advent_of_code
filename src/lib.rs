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