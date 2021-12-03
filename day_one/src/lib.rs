use mkagie_utils::file_to_int_vec;

pub fn part_one(input: &[u16]) -> u16 {
    input
        .windows(2)
        .map(|w| (w[1] as i32 - w[0] as i32 > 0))
        .fold(0, |a, b| a as u16 + b as u16)
}

pub fn part_two(input: &[u16]) -> u16 {
    let window: Vec<u16> = input
        .windows(3)
        .map(|w| w.iter().fold(0, |a, b| a + b))
        .collect();
    part_one(&window)
}

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_one/day_one.txt";
    let input: Vec<u16> = file_to_int_vec(filename);

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

        let output = part_one(&input);
        assert_eq!(output, 7);

        let output = part_two(&input);
        assert_eq!(output, 5);
    }
}
