use mkagie_utils::*;

fn part_one(input: &[i32]) -> i32 {
    0
}

fn part_two(input: &[i32]) -> i32 {
    0
}

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_three/day_three.txt";
    let input = file_to_int_vec(filename);

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = [];
        let output = part_one(&input);
        let truth = 0;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = [];
        let output = part_two(&input);
        let truth = 0;

        assert_eq!(output, truth);
    }
}
