use std::str::Chars;

use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_eighteen/day_eighteen.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

#[derive(Debug, Clone)]
struct Pair {
    left: Number,
    right: Number,
    embedding_level: u32,
}
impl Pair {
    pub fn new(input: &str) -> Self {
        Pair::recursive_parse(&mut input.chars(), None).unwrap()
    }

    pub fn increment_embedding_level(&mut self) {
        self.embedding_level += 1;
        if let Number::Pair(ref mut p) = self.left {
            p.increment_embedding_level();
        }
        if let Number::Pair(ref mut p) = self.right {
            p.increment_embedding_level();
        }
    }

    fn recursive_parse(input: &mut Chars, prev_value: Option<char>) -> Option<Pair> {
        let open = if let Some(p) = prev_value {
            Some(p)
        } else {
            input.next()
        };
        if let Some('[') = open {
            // Go to the next step
            let left = input.next().unwrap();
            if let Some(left) = left.to_digit(10) {
                if let Some(',') = input.next() {
                    let right = input.next().unwrap();
                    if let Some(right) = right.to_digit(10) {
                        if let Some(']') = input.next() {
                            Some(Pair {
                                left: Number::Value(left),
                                right: Number::Value(right),
                                embedding_level: 0,
                            })
                        } else {
                            None
                        }
                    } else {
                        let mut right = Pair::recursive_parse(input, Some(right)).unwrap();
                        right.increment_embedding_level();
                        if let Some(']') = input.next() {
                            Some(Pair {
                                left: Number::Value(left),
                                right: Number::Pair(Box::new(right)),
                                embedding_level: 0,
                            })
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            } else {
                let mut left = Pair::recursive_parse(input, Some(left)).unwrap();
                left.increment_embedding_level();
                if let Some(',') = input.next() {
                    let right = input.next().unwrap();
                    if let Some(right) = right.to_digit(10) {
                        if let Some(']') = input.next() {
                            Some(Pair {
                                left: Number::Pair(Box::new(left)),
                                right: Number::Value(right),
                                embedding_level: 0,
                            })
                        } else {
                            None
                        }
                    } else {
                        let mut right = Pair::recursive_parse(input, Some(right)).unwrap();
                        right.increment_embedding_level();
                        if let Some(']') = input.next() {
                            Some(Pair {
                                left: Number::Pair(Box::new(left)),
                                right: Number::Pair(Box::new(right)),
                                embedding_level: 0,
                            })
                        } else {
                            None
                        }
                    }
                } else {
                    None
                }
            }
        } else {
            None
        }
    }

    pub fn add(mut self, mut other: Pair) -> Pair {
        self.increment_embedding_level();
        other.increment_embedding_level();
        let mut new_pair = Pair {
            left: Number::Pair(Box::new(self)),
            right: Number::Pair(Box::new(other)),
            embedding_level: 0,
        };
        new_pair.reduce();
        new_pair
    }

    pub fn reduce(&mut self) {
        let mut did_split_at_all = true;

        while did_split_at_all {
            let mut did_explode_at_all = true;
            while did_explode_at_all {
                did_explode_at_all = self.try_explode(None, None).1;
                if did_explode_at_all {}
            }
            // did_split_at_all = self.try_split().1;
            did_split_at_all = self.try_split().1;
            if did_split_at_all {}
        }
    }

    fn try_split(&mut self) -> (bool, bool) {
        // Try this.left
        match &mut self.left {
            Number::Value(l) => {
                if *l >= 10 {
                    let left = (*l as f32 / 2.0).floor() as u32;
                    let right = (*l as f32 / 2.0).ceil() as u32;
                    self.left = Number::Pair(Box::new(Pair {
                        left: Number::Value(left),
                        right: Number::Value(right),
                        embedding_level: self.embedding_level + 1,
                    }));
                    return (true, true);
                }
            }
            Number::Pair(p) => {
                let (_, did_split_ever) = p.try_split();
                if did_split_ever {
                    return (false, true);
                }
            }
        }
        // Try this.right
        match &mut self.right {
            Number::Value(r) => {
                if *r >= 10 {
                    let left = (*r as f32 / 2.0).floor() as u32;
                    let right = (*r as f32 / 2.0).ceil() as u32;
                    self.right = Number::Pair(Box::new(Pair {
                        left: Number::Value(left),
                        right: Number::Value(right),
                        embedding_level: self.embedding_level + 1,
                    }));
                    return (true, true);
                }
            }
            Number::Pair(p) => {
                let (_, did_split_ever) = p.try_split();
                if did_split_ever {
                    return (false, true);
                }
            }
        }
        (false, false)
    }

    fn can_explode(&self) -> bool {
        self.embedding_level >= 4
            && matches!(self.left, Number::Value(_))
            && matches!(self.right, Number::Value(_))
    }

    fn get_rightmost(&mut self) -> &mut u32 {
        match &mut self.right {
            Number::Value(v) => v,
            Number::Pair(p) => p.get_rightmost(),
        }
    }

    fn get_leftmost(&mut self) -> &mut u32 {
        match &mut self.left {
            Number::Value(v) => v,
            Number::Pair(p) => p.get_leftmost(),
        }
    }

    /// Returns (if did explode, if anything has exploded)
    fn try_explode(&mut self, left: Option<&mut u32>, right: Option<&mut u32>) -> (bool, bool) {
        // Try to explode
        if self.can_explode() {
            self.explode(left, right);
            return (true, true);
        }
        // Try to explode left
        if let Number::Pair(ref mut p) = self.left {
            let right = match &mut self.right {
                Number::Value(v) => Some(v),
                Number::Pair(p) => Some(p.get_leftmost()),
            };
            let (did_explode, did_explode_at_all) = p.try_explode(left, right);
            if did_explode {
                self.left = Number::Value(0);
                return (false, true);
            }
            if did_explode_at_all {
                return (false, true);
            }
        }
        // Try to explode right
        if let Number::Pair(ref mut p) = self.right {
            let left = match &mut self.left {
                Number::Value(v) => Some(v),
                Number::Pair(p) => Some(p.get_rightmost()),
            };
            let (did_explode, did_explode_at_all) = p.try_explode(left, right);
            if did_explode {
                self.right = Number::Value(0);
                return (false, true);
            } else if did_explode_at_all {
                return (false, true);
            }
        }
        (false, false)
    }

    fn explode(&mut self, left: Option<&mut u32>, right: Option<&mut u32>) {
        // Add left value to most immediate value to the left
        if let Number::Value(sv) = &self.left {
            if let Some(l) = left {
                *l += sv;
            }
        }
        // Add the right
        if let Number::Value(sv) = &self.right {
            if let Some(r) = right {
                *r += sv;
            }
        }
    }

    fn compute_magnitude(&self) -> u32 {
        // 3 * magnitude of + 2 * magnitude of right number
        let left_mag = match &self.left {
            Number::Value(x) => *x,
            Number::Pair(p) => p.compute_magnitude(),
        };
        let right_mag = match &self.right {
            Number::Value(x) => *x,
            Number::Pair(p) => p.compute_magnitude(),
        };
        3 * left_mag + 2 * right_mag
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        let left_str = match &self.left {
            Number::Value(v) => format!("{}", v),
            Number::Pair(p) => p.as_string(),
        };
        let right_str = match &self.right {
            Number::Value(v) => format!("{}", v),
            Number::Pair(p) => p.as_string(),
        };
        format!("[{},{}]", left_str, right_str)
    }
}

#[derive(Debug, Clone)]
enum Number {
    Value(u32),
    Pair(Box<Pair>),
}

fn part_one(input: &[String]) -> u32 {
    let p = input
        .iter()
        .map(|x| Pair::new(x.as_str()))
        .reduce(|accum, item| accum.add(item))
        .unwrap();
    p.compute_magnitude()
}

fn part_two(input: &[String]) -> u32 {
    let pairs: Vec<Pair> = input.iter().map(|x| Pair::new(x.as_str())).collect();
    pairs
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx0, p0)| {
            pairs
                .clone()
                .into_iter()
                .enumerate()
                .map(|(idx1, p1)| {
                    if idx0 != idx1 {
                        Some(p0.clone().add(p1).compute_magnitude())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect::<Vec<u32>>()
        })
        .flatten()
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));

        let truth = 4140;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 3993;

        assert_eq!(output, truth);
    }
}
