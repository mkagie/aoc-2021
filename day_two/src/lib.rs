use mkagie_utils::file_to_string_vec;
pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_two/day_two.txt";
    let lines = file_to_string_vec(filename);
    let mut pos = Position::new();
    lines.iter().for_each(|x| pos.parse_two(x));
    println!("{}", pos.function())
}

#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}
impl Position {
    fn parse(&mut self, value: &str) {
        let splits = value.split(" ").collect::<Vec<&str>>();
        let direction = splits[0];
        let amount: i32 = splits[1].parse().unwrap();
        match direction {
            "forward" => self.horizontal += amount,
            "down" => self.depth += amount,
            "up" => self.depth -= amount,
            other => (),
        }
    }

    fn parse_two(&mut self, value: &str) {
        let splits = value.split(" ").collect::<Vec<&str>>();
        let direction = splits[0];
        let amount: i32 = splits[1].parse().unwrap();
        match direction {
            "forward" => {
                self.horizontal += amount;
                self.depth += self.aim * amount;
            }
            "down" => self.aim += amount,
            "up" => self.aim -= amount,
            other => (),
        }
    }

    fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn function(&self) -> i32 {
        self.horizontal * self.depth
    }
}

pub fn part_one(input: &[&str]) -> i32 {
    let mut pos = Position::new();
    input.iter().for_each(|x| pos.parse(x));
    pos.function()
}

pub fn part_two(input: &[&str]) -> i32 {
    let mut pos = Position::new();
    input.iter().for_each(|x| pos.parse_two(x));
    pos.function()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let output = part_one(&input);
        assert_eq!(output, 150);
    }

    #[test]
    fn test_part_two() {
        let input = [
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let output = part_two(&input);
        assert_eq!(output, 900);
    }
}
