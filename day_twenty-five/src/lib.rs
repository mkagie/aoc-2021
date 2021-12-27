use mkagie_utils::*;

pub fn run() {
    let filename = "day_nineteen.txt";
    let input = file_to_string_vec(filename);

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

fn parse_input(input: &[String]) -> Vec<u32> {
    todo!()
}

fn part_one(input: &[String]) -> u32 {
    todo!()
}

fn part_two(input: &[String]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        ""
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 590784;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 2758514936282235;

        assert_eq!(output, truth);
    }
}
