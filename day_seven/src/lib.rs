use mkagie_utils::*;

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_seven/day_seven.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

fn parse_input(input: &[String]) -> Vec<i32> {
    input[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

fn do_processing(input: Vec<i32>, compute_function: fn(i32) -> i32) -> i32 {
    let max_val = input.iter().max().unwrap();

    (0..*max_val)
        .map(|center_val| {
            input.iter().fold(0, |accum, &x| {
                accum + compute_function((x - center_val).abs())
            })
        })
        .min()
        .unwrap()
}

pub fn part_one(input: &[String]) -> i32 {
    let input = parse_input(input);
    do_processing(input, |x| x)
}

fn sequential_sum(input: i32) -> i32 {
    input * (1 + input) / 2
}

pub fn part_two(input: &[String]) -> i32 {
    let input = parse_input(input);
    do_processing(input, sequential_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "16,1,2,0,4,2,7,1,2,14"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 37;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 168;

        assert_eq!(output, truth);
    }
}
