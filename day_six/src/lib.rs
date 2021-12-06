use mkagie_utils::*;

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_six/day_six.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

fn parse_input(input: &[String]) -> [u128; 9] {
    let mut counter = [0u128; 9];
    input[0]
        .split(',')
        .for_each(|x| counter[x.parse::<u128>().unwrap() as usize] += 1);
    counter
}

fn do_processing(counter: &mut [u128; 9], n_days: u16) -> u128 {
    (0..n_days).for_each(|_| {
        // Save 0 -- this many will need to be added to a counter with 6
        let reset = counter[0];
        counter.rotate_left(1);
        counter[6] += reset;
    });
    counter.iter().sum()
}

pub fn part_one(input: &[String]) -> u128 {
    // parse the input
    let mut input = parse_input(input);
    do_processing(&mut input, 80)
}

pub fn part_two(input: &[String]) -> u128 {
    do_processing(&mut parse_input(input), 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "3,4,3,1,2"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 5934;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 26984457539;

        assert_eq!(output, truth);
    }
}
