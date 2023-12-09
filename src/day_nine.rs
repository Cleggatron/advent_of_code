fn calculate_difference(nums: Vec<i32>) -> i32{
    let mut first_pointer = 0;
    let mut second_pointer = 1;
    let mut diffs:Vec<i32> = Vec::new();
    if nums[first_pointer] == nums[nums.len()-1]  {
        println!("Return from recursion: {:?}", nums);
        return nums[first_pointer];
    }


    while second_pointer < nums.len(){
        diffs.push(nums[second_pointer] - nums[first_pointer]);
        first_pointer += 1;
        second_pointer += 1;
    }

    return nums[first_pointer] + calculate_difference(diffs)
}

#[test]
fn it_displays_the_difference(){
    let nums = vec![1,3,6,10,15,21];
    assert_eq!(calculate_difference(nums), 28);
}

pub fn sum_next_sequences (file_contents: String) -> i32 {
    let lines :Vec<&str> = file_contents.lines().collect();
    let mut total = 0;
    for line in lines {
        let mut nums :Vec<i32> = line.trim().split_ascii_whitespace().map(|num| num.parse().unwrap()).collect();

        //reverse added for two star solution
        nums.reverse();
        let next = calculate_difference(nums);
        total += next;
    } 
    total
}