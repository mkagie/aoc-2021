use std::collections::HashMap;

use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_fourteen/day_fourteen.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

struct Rule {
    pub pattern: String,
    pub repl: String,
}
impl Rule {
    pub fn new(input: &str) -> Self {
        let sides: Vec<&str> = input.split(" -> ").collect();
        Rule {
            pattern: sides[0].to_string(),
            repl: sides[1].to_string(),
        }
    }

    pub fn matches(&self, input: &str) -> Option<String> {
        let output = input.replace(&self.pattern, &self.repl);
        if input == output {
            None
        } else {
            Some(output)
        }
    }
}

fn process_inputs(input: &[String]) -> (String, Vec<Rule>) {
    let pattern = input[0].to_string();
    let rules: Vec<Rule> = input.iter().skip(2).map(|x| Rule::new(x)).collect();
    (pattern, rules)
}

fn part_one(input: &[String]) -> u32 {
    let (mut pattern, rules) = process_inputs(input);
    for _ in 0..10 {
        let mut output = "".to_string();
        for idx in 0..pattern.len() - 1 {
            let substring = &pattern[idx..idx + 2];
            let repl: Vec<String> = rules
                .iter()
                .map(|x| x.matches(substring))
                .flatten()
                .collect();
            output = format!("{}{}{}", output, substring.chars().next().unwrap(), repl[0]);
        }
        output = format!(
            "{}{}",
            output,
            pattern.chars().nth(pattern.len() - 1).unwrap()
        );
        pattern = output;
    }
    let mut counter = HashMap::new();
    pattern.chars().for_each(|x| {
        if let Some(c) = counter.get_mut(&x) {
            *c += 1;
        } else {
            counter.insert(x, 1);
        }
    });
    // Create a reverse hashmap
    let mut vec = Vec::new();
    counter.iter().for_each(|(char, count)| {
        vec.push((char, count));
    });
    vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    vec[vec.len() - 1].1 - vec[0].1
}

fn part_two(input: &[String]) -> u128 {
    let (mut pattern, rules) = process_inputs(input);
    for _ in 0..40 {
        let mut output = "".to_string();
        for idx in 0..pattern.len() - 1 {
            let substring = &pattern[idx..idx + 2];
            let repl: Vec<String> = rules
                .iter()
                .map(|x| x.matches(substring))
                .flatten()
                .collect();
            output = format!("{}{}{}", output, substring.chars().next().unwrap(), repl[0]);
        }
        output = format!(
            "{}{}",
            output,
            pattern.chars().nth(pattern.len() - 1).unwrap()
        );
        pattern = output;
    }
    let mut counter = HashMap::new();
    pattern.chars().for_each(|x| {
        if let Some(c) = counter.get_mut(&x) {
            *c += 1;
        } else {
            counter.insert(x, 1);
        }
    });
    // Create a reverse hashmap
    let mut vec = Vec::new();
    counter.iter().for_each(|(char, count)| {
        vec.push((char, count));
    });
    vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    vec[vec.len() - 1].1 - vec[0].1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 1588;

        assert_eq!(output, truth);
    }

    // #[test]
    // fn test_two() {
    //     let input = input();
    //     let output = part_two(&str_to_string_vec(&input));
    //     let truth = 2188189693529u128;

    //     assert_eq!(output, truth);
    // }
}
