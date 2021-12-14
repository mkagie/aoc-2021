use mkagie_utils::*;
use ndarray::prelude::*;
use ndarray::{aview1, Array};
use std::collections::HashMap;

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_eight/day_eight.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

#[derive(Debug)]
struct Combo {
    pub digits: Vec<[u8; 7]>,
    pub output: Vec<[u8; 7]>,
}

impl Combo {
    fn count_unique(&self) -> u32 {
        let mut counter = 0;
        // Find 1, 4, 7, 8 -- map those back to number of unique identifiers
        let positions: Vec<&[u8; 7]> = vec![2, 4, 3, 7]
            .iter()
            .map(|x| {
                self.digits
                    .get(
                        self.digits
                            .iter()
                            .position(|dig| dig.iter().sum::<u8>() == *x)
                            .unwrap(),
                    )
                    .unwrap()
            })
            .collect();
        self.output.iter().for_each(|x| {
            if positions.contains(&x) {
                counter += 1
            }
        });
        counter
    }
    fn decode(&self) -> i32 {
        let mut digit_decoder = HashMap::new();
        // Find 1, 4, 7, 8 -- map those back to number of unique identifiers
        let positions: Vec<[u8; 7]> = vec![2, 4, 3, 7]
            .iter()
            .map(|&x| {
                *self
                    .digits
                    .get(
                        self.digits
                            .iter()
                            .position(|&dig| dig.iter().sum::<u8>() == x)
                            .unwrap(),
                    )
                    .unwrap()
            })
            .collect();
        let position_map = vec![1, 4, 7, 8];
        positions
            .iter()
            .zip(position_map.iter())
            .for_each(|(&position, &value)| {
                let _ = digit_decoder.insert(position, value);
            });

        // Start filling up the decoder -- a is 7 and 8
        let decoder_array = Array::from_iter(positions.clone().into_iter().flatten())
            .into_shape((4, 7))
            .unwrap();

        // Find a
        let a_pos = decoder_array
            .columns()
            .into_iter()
            .position(|x| x == aview1(&[0u8, 0, 1, 1]))
            .unwrap();
        // find b/d positions
        let bd_poss: Vec<usize> = decoder_array
            .columns()
            .into_iter()
            .enumerate()
            .map(|(idx, x)| {
                if x == aview1(&[0u8, 1, 0, 1]) {
                    Some(idx)
                } else {
                    None
                }
            })
            .flatten()
            .collect();
        // Find c/f positions
        let cf_poss: Vec<usize> = decoder_array
            .columns()
            .into_iter()
            .enumerate()
            .map(|(idx, x)| {
                if x == aview1(&[1u8, 1, 1, 1]) {
                    Some(idx)
                } else {
                    None
                }
            })
            .flatten()
            .collect();
        // Find e/g positions
        let eg_poss: Vec<usize> = decoder_array
            .columns()
            .into_iter()
            .enumerate()
            .map(|(idx, x)| {
                if x == aview1(&[0u8, 0, 0, 1]) {
                    Some(idx)
                } else {
                    None
                }
            })
            .flatten()
            .collect();

        // Build other array
        let remaining_array = Array::from_iter(
            self.digits
                .clone()
                .into_iter()
                .filter(|x| !positions.contains(x))
                .flatten(),
        )
        .into_shape((6, 7))
        .unwrap();
        let col_sums = remaining_array.sum_axis(Axis(0));

        // Find b/d
        let (b_pos, d_pos) = {
            if col_sums[bd_poss[0]] == 4 {
                (bd_poss[0], bd_poss[1])
            } else {
                (bd_poss[1], bd_poss[0])
            }
        };
        // Find c/f
        let (c_pos, f_pos) = {
            if col_sums[cf_poss[0]] == 4 {
                (cf_poss[0], cf_poss[1])
            } else {
                (cf_poss[1], cf_poss[0])
            }
        };
        // Find e/g
        let (e_pos, g_pos) = {
            if col_sums[eg_poss[0]] == 3 {
                (eg_poss[0], eg_poss[1])
            } else {
                (eg_poss[1], eg_poss[0])
            }
        };
        // Finish implementing decoder
        digit_decoder.insert(
            positions_to_repr(vec![a_pos, b_pos, c_pos, e_pos, f_pos, g_pos]),
            0,
        );
        digit_decoder.insert(
            positions_to_repr(vec![a_pos, c_pos, d_pos, e_pos, g_pos]),
            2,
        );
        digit_decoder.insert(
            positions_to_repr(vec![a_pos, c_pos, d_pos, f_pos, g_pos]),
            3,
        );
        digit_decoder.insert(
            positions_to_repr(vec![a_pos, b_pos, d_pos, f_pos, g_pos]),
            5,
        );
        digit_decoder.insert(
            positions_to_repr(vec![a_pos, b_pos, d_pos, e_pos, f_pos, g_pos]),
            6,
        );
        digit_decoder.insert(
            positions_to_repr(vec![a_pos, b_pos, c_pos, d_pos, f_pos, g_pos]),
            9,
        );

        // Decode outputs:
        let decoded_digits: Vec<&i32> = self
            .output
            .iter()
            .map(|x| digit_decoder.get(x).unwrap())
            .collect();
        decoded_digits[0] * 1000
            + decoded_digits[1] * 100
            + decoded_digits[2] * 10
            + decoded_digits[3]
    }
}

fn positions_to_repr(positions: Vec<usize>) -> [u8; 7] {
    let mut repr = [0u8; 7];
    positions.iter().for_each(|&x| repr[x] = 1);
    repr
}

fn convert_str_to_array(input: &str) -> Vec<[u8; 7]> {
    input
        .split(' ')
        .map(|x| {
            let mut repr = [0u8; 7];
            x.chars().for_each(|char| {
                match char {
                    'a' => repr[0] += 1,
                    'b' => repr[1] += 1,
                    'c' => repr[2] += 1,
                    'd' => repr[3] += 1,
                    'e' => repr[4] += 1,
                    'f' => repr[5] += 1,
                    'g' => repr[6] += 1,
                    _ => (),
                };
            });
            repr
        })
        .filter(|&x| x.iter().sum::<u8>() > 0)
        .collect()
}

fn parse_input(input: &[String]) -> Vec<Combo> {
    input
        .iter()
        .map(|x| {
            let sides: Vec<&str> = x.split('|').collect();
            // Parse digits
            let digits = convert_str_to_array(sides[0]);
            // Parse Output
            let output = convert_str_to_array(sides[1]);
            Combo { digits, output }
        })
        .collect()
}

pub fn part_one(input: &[String]) -> u32 {
    let input = parse_input(input);
    input.into_iter().map(|x| x.count_unique()).sum()
}

pub fn part_two(input: &[String]) -> i32 {
    let input = parse_input(input);
    input.into_iter().map(|x| x.decode()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 26;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 61229;

        assert_eq!(output, truth);
    }
}
