
pub fn calculate_permutations(time :u32, distance :u32) -> u32 {
    let mut permutations = 0;

    let mut current_speed :u32= 1;
    let mut current_time :u32 = time-1;
    while current_speed < time {
        if current_speed * current_time > distance{
            permutations +=1;
        }
        current_speed += 1;
        current_time -= 1;
    }
    permutations
}

#[test]
fn it_calculates_correctly(){
    assert_eq!(calculate_permutations(7, 9), 4)
}

pub fn calculate_permutations_two(time :usize, distance :usize) -> usize {
    let mut permutations = 0;

    let mut current_speed :usize= 1;
    let mut current_time :usize = time-1;
    while current_speed < time {
        if current_speed * current_time > distance{
            permutations +=1;
        }
        current_speed += 1;
        current_time -= 1;
    }
    permutations
}
