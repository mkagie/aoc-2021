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

    pub fn gen_new_pattern(&self, input: &str) -> Option<Vec<String>> {
        if input == self.pattern {
            // Create the output pattern
            let output = vec![
                format!("{}{}", self.pattern.chars().next().unwrap(), self.repl),
                format!("{}{}", self.repl, self.pattern.chars().nth(1).unwrap()),
            ];
            Some(output)
        } else {
            None
        }
    }
}

fn process_inputs(input: &[String]) -> (String, Vec<Rule>) {
    let pattern = input[0].to_string();
    let rules: Vec<Rule> = input.iter().skip(2).map(|x| Rule::new(x)).collect();
    (pattern, rules)
}

fn do_work(input: &[String], n_iters: u32) -> u128 {
    let (pattern, rules) = process_inputs(input);
    // Parse pattern into a vector of patterns that match rules
    let mut existing_patterns = HashMap::new();
    for idx in 0..pattern.len() - 1 {
        let substring = &pattern[idx..idx + 2];
        if let Some(counter) = existing_patterns.get_mut(&substring.to_string()) {
            *counter += 1;
        } else {
            existing_patterns.insert(substring.to_string(), 1);
        }
    }

    for _ in 0..n_iters {
        let mut patterns = HashMap::new();
        existing_patterns.iter().for_each(|(ch, count)| {
            let repl: Vec<Vec<String>> = rules
                .iter()
                .map(|x| x.gen_new_pattern(ch))
                .flatten()
                .collect();
            repl[0].iter().for_each(|x| {
                if let Some(counter) = patterns.get_mut(x) {
                    *counter += *count;
                } else {
                    patterns.insert(x.to_owned(), *count);
                }
            })
        });
        existing_patterns = patterns;
    }
    let mut alphabet = HashMap::new();
    existing_patterns.iter().for_each(|(patt, count)| {
        patt.chars().for_each(|c| {
            if let Some(counter) = alphabet.get_mut(&c) {
                *counter += *count;
            } else {
                alphabet.insert(c, *count);
            }
        });
    });
    // Divide by two and round down to actually get the numbers
    alphabet.iter_mut().for_each(|(_, count)| {
        *count /= 2;
    });
    *alphabet.get_mut(&pattern.chars().next().unwrap()).unwrap() += 1;
    *alphabet
        .get_mut(&pattern.chars().nth(pattern.len() - 1).unwrap())
        .unwrap() += 1;
    let mut vec = Vec::new();
    alphabet.iter().for_each(|(char, count)| {
        vec.push((char, count));
    });
    vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
    vec[vec.len() - 1].1 - vec[0].1
}

fn part_one(input: &[String]) -> u32 {
    do_work(input, 10) as u32
}

fn part_two(input: &[String]) -> u128 {
    do_work(input, 40)
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

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 2188189693529u128;

        assert_eq!(output, truth);
    }
}
