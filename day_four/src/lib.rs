use mkagie_utils::*;
use ndarray::prelude::*;
use ndarray::{Array, Ix2};

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_four/day_four.txt";
    let input = file_to_string_vec(filename);

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[derive(Debug)]
struct Board {
    board: Array<u64, Ix2>,
    mapping: Array<u64, Ix2>,
    bingo: bool,
}
impl Board {
    pub fn new(board: Array<u64, Ix2>) -> Self {
        let shape = [board.nrows(), board.ncols()];
        Board {
            board,
            mapping: Array::<u64, _>::zeros(shape),
            bingo: false,
        }
    }

    pub fn update(&mut self, val: u64) -> Option<u64> {
        if !self.bingo {
            let idx_present: Vec<usize> = self
                .board
                .iter()
                .enumerate()
                .map(|(idx, &x)| if x == val { Some(idx) } else { None })
                .flatten()
                .collect();

            if !idx_present.is_empty() {
                let _ = self.mapping.iter_mut().enumerate().for_each(|(idx, val)| {
                    if idx_present.contains(&idx) {
                        *val = 1;
                    }
                });
            }

            // Check to see if any row is all ones
            let sums0 = self.mapping.sum_axis(Axis(0));
            let sums1 = self.mapping.sum_axis(Axis(1));
            if let Some(5) = sums0
                .iter()
                .reduce(|acc, item| if acc >= item { acc } else { item })
            {
                let idx_empty: Vec<usize> = self
                    .mapping
                    .iter()
                    .enumerate()
                    .map(|(idx, &x)| if x == 0 { Some(idx) } else { None })
                    .flatten()
                    .collect();

                let sum = self.board.iter().enumerate().fold(0, |acc, (idx, x)| {
                    if idx_empty.contains(&idx) {
                        acc + x
                    } else {
                        acc
                    }
                });
                self.bingo = true;
                return Some(sum * val);
            }
            if let Some(5) = sums1
                .iter()
                .reduce(|acc, item| if acc >= item { acc } else { item })
            {
                let idx_empty: Vec<usize> = self
                    .mapping
                    .iter()
                    .enumerate()
                    .map(|(idx, &x)| if x == 0 { Some(idx) } else { None })
                    .flatten()
                    .collect();

                let sum = self.board.iter().enumerate().fold(0, |acc, (idx, x)| {
                    if idx_empty.contains(&idx) {
                        acc + x
                    } else {
                        acc
                    }
                });
                self.bingo = true;
                return Some(sum * val);
            }
            None
        } else {
            None
        }
    }
}

fn part_one(input: &[String]) -> u64 {
    let (inputs, mut boards) = parse_input(input);

    for &input in inputs.iter() {
        let solutions: Vec<u64> = boards
            .iter_mut()
            .map(|x| x.update(input))
            .flatten()
            .collect();
        if !solutions.is_empty() {
            return solutions[0];
        }
    }
    0
}

fn part_two(input: &[String]) -> u64 {
    let (inputs, mut boards) = parse_input(input);
    let mut solutions = Vec::new();

    for &input in inputs.iter() {
        let cur_solutions: Vec<u64> = boards
            .iter_mut()
            .map(|x| x.update(input))
            .flatten()
            .collect();
        solutions.extend(cur_solutions);
    }
    *solutions.last().unwrap()
}

fn parse_input(input: &[String]) -> (Vec<u64>, Vec<Board>) {
    // Parse the first one
    let mut iter = input.iter();
    let inputs: Vec<u64> = iter
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut boards = Vec::new();
    // skip white space
    while iter.next().is_some() {
        let mut temp_string = String::new();
        for _ in 0..5 {
            temp_string = temp_string + " " + iter.next().unwrap();
        }
        boards.push(Board::new(
            Array::from_iter(
                temp_string
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<u64>().unwrap()),
            )
            .into_shape((5, 5))
            .unwrap(),
        ));
    }
    (inputs, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let output = part_one(&str_to_string_vec(&input));

        assert_eq!(output, 4512);
    }

    #[test]
    fn test_part_two() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";
        let output = part_two(&str_to_string_vec(&input));

        assert_eq!(output, 1924);
    }
}
