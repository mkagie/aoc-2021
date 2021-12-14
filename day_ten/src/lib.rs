use mkagie_utils::*;
use std::collections::HashMap;

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_ten/day_ten.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

struct Stack {
    vec: Vec<char>,
    mapping: HashMap<char, (char, u32)>,
    reverse_mapping: HashMap<char, (char, u128)>,
}

impl Stack {
    pub fn new() -> Self {
        let mapping = HashMap::from([
            (')', ('(', 3u32)),
            (']', ('[', 57)),
            ('}', ('{', 1197)),
            ('>', ('<', 25137)),
        ]);
        let reverse_mapping = HashMap::from([
            ('(', (')', 1u128)),
            ('[', (']', 2)),
            ('{', ('}', 3)),
            ('<', ('>', 4)),
        ]);
        Stack {
            vec: Vec::new(),
            mapping,
            reverse_mapping,
        }
    }

    pub fn update(&mut self, val: char) -> Option<u32> {
        if ['(', '[', '{', '<'].contains(&val) {
            self.vec.push(val);
            None
        } else {
            let ret = self.mapping.get(&val).unwrap();
            let popped_val = self.vec.pop().unwrap();
            if popped_val != ret.0 {
                Some(ret.1)
            } else {
                None
            }
        }
    }

    pub fn score_line(&mut self, line: &str) -> Option<u128> {
        self.vec.clear();
        if line.chars().map(|x| self.update(x)).flatten().count() > 0 {
            return None;
        }
        if self.vec.is_empty() {
            return None;
        }
        let mut score = 0u128;
        while let Some(value) = self.vec.pop() {
            let unweighted_score = self.reverse_mapping.get(&value).unwrap().1;
            score = score * 5 + unweighted_score;
        }
        Some(score)
    }
}

fn part_one(input: &[String]) -> u32 {
    input
        .iter()
        .map(|x| {
            let mut stack = Stack::new();
            for ch in x.chars() {
                if let Some(score) = stack.update(ch) {
                    return score;
                }
            }
            0
        })
        .sum()
}

fn part_two(input: &[String]) -> u128 {
    let mut scores: Vec<u128> = input
        .iter()
        .map(|x| {
            let mut stack = Stack::new();
            stack.score_line(x)
        })
        .flatten()
        .collect();
    scores.sort_unstable();
    *scores.get(scores.len() / 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 26397;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 288957;

        assert_eq!(output, truth);
    }
}
