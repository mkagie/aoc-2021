use mkagie_utils::*;
use ndarray::{Array, Ix2};

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_five/day_five.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

#[derive(Clone, Copy, Debug)]
struct Xy {
    pub x: i64,
    pub y: i64,
}

#[derive(Debug)]
struct Xyxy {
    pub first: Xy,
    pub second: Xy,
}

impl Xyxy {
    pub fn get_values_vertical_horizontal(&self) -> Vec<Xy> {
        let mut v = Vec::new();
        if self.first.x == self.second.x {
            if self.first.y <= self.second.y {
                // Look only vertically
                (self.first.y..self.second.y + 1).for_each(|y| v.push(Xy { x: self.first.x, y }))
            } else {
                (self.second.y..self.first.y + 1).for_each(|y| v.push(Xy { x: self.first.x, y }))
            }
        } else if self.first.y == self.second.y {
            if self.first.x <= self.second.x {
                (self.first.x..self.second.x + 1).for_each(|x| v.push(Xy { x, y: self.first.y }))
            } else {
                (self.second.x..self.first.x + 1).for_each(|x| v.push(Xy { x, y: self.first.y }))
            }
        }
        v
    }

    fn get_slope(&self) -> f32 {
        (self.second.y as f32 - self.first.y as f32) / (self.second.x as f32 - self.first.x as f32)
    }

    pub fn get_values_including_diagonal(&self) -> Vec<Xy> {
        let mut v = self.get_values_vertical_horizontal();
        let slope = self.get_slope();

        // Compute the slope
        if slope == 1.0 {
            // increment each
            let mut x = self.first.x;
            let mut y = self.first.y;
            let val = if self.first.x <= self.second.x { 1 } else { -1 };

            while x != self.second.x {
                v.push(Xy { x, y });
                x += val;
                y += val;
            }
        } else if slope == -1.0 {
            // increment x, decrement y
            let mut x = self.first.x;
            let mut y = self.first.y;
            let val = if self.first.x <= self.second.x { 1 } else { -1 };

            v.push(Xy { x, y });
            while x != self.second.x {
                x += val;
                y -= val;
                v.push(Xy { x, y });
            }
        }
        v
    }
}

fn parse_input(input: &[String]) -> (Vec<Xyxy>, Array<i64, Ix2>) {
    // parse into vector of xy->xy
    let mut max_val: i64 = 0;
    let xyxys: Vec<Xyxy> = input
        .iter()
        .map(|x| {
            let xys: Vec<Xy> = x
                .split(" -> ")
                .map(|val| {
                    let xy: Vec<i64> = val
                        .split(',')
                        .map(|num| num.parse::<i64>().unwrap())
                        .collect();

                    let x = *xy.get(0).unwrap();
                    let y = *xy.get(1).unwrap();
                    if x > max_val {
                        max_val = x;
                    }
                    if y > max_val {
                        max_val = y;
                    }
                    Xy { x, y }
                })
                .collect::<Vec<Xy>>();
            Xyxy {
                first: *xys.get(0).unwrap(),
                second: *xys.get(1).unwrap(),
            }
        })
        .collect();

    // Create array
    max_val += 1;
    let arr = Array::<i64, _>::zeros((max_val as usize, max_val as usize));
    (xyxys, arr)
}

pub fn part_one(input: &[String]) -> i64 {
    // parse into vector of xy->xy
    let (xyxys, mut arr) = parse_input(input);
    let mut counter = 0;
    // Fill with inputs, increment counter every time 2+
    xyxys.iter().for_each(|xyxy| {
        let values = xyxy.get_values_vertical_horizontal();
        values.iter().for_each(|xy| {
            let val = arr.get_mut((xy.y as usize, xy.x as usize)).unwrap();
            *val += 1;
            if *val == 2 {
                counter += 1;
            }
        })
    });
    counter
}

pub fn part_two(input: &[String]) -> i64 {
    let (xyxys, mut arr) = parse_input(input);
    let mut counter = 0;
    // Fill with inputs, increment counter every time 2+
    xyxys.iter().for_each(|xyxy| {
        let values = xyxy.get_values_including_diagonal();
        values.iter().for_each(|xy| {
            let val = arr.get_mut((xy.y as usize, xy.x as usize)).unwrap();
            *val += 1;
            if *val == 2 {
                counter += 1;
            }
        })
    });
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 5;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 12;

        assert_eq!(output, truth);
    }
}
