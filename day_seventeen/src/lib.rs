use std::cmp;

use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_seventeen/day_seventeen.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines, (1, 1000), (1, 1000)));
    println!("{:?}", part_two(&lines, (1, 1000), (-500, 1000)));
}

struct Probe {
    pub x: i32,
    pub y: i32,
    pub vel_x: i32,
    pub vel_y: i32,
}
impl Probe {
    pub fn new() -> Self {
        Probe {
            x: 0,
            y: 0,
            vel_x: 0,
            vel_y: 0,
        }
    }

    fn reset(&mut self) {
        self.x = 0;
        self.y = 0;
        self.vel_x = 0;
        self.vel_y = 0;
    }

    pub fn take_step(&mut self) {
        self.x += self.vel_x;
        self.vel_x -= num::signum(self.vel_x);
        self.y += self.vel_y;
        self.vel_y -= 1;
    }

    pub fn should_stop(&self, t: &TargetArea) -> bool {
        // Should stop if our y_velocity is - and our y height is < lowest
        self.vel_y < 0 && self.y < t.get_min_height()
    }

    pub fn set_velocity(&mut self, velocity: &(i32, i32)) {
        self.vel_x = velocity.0;
        self.vel_y = velocity.1;
    }

    pub fn max_height(&mut self, velocity: &(i32, i32), target_area: &TargetArea) -> Option<i32> {
        self.reset();
        self.set_velocity(velocity);
        let mut max_height = 0;
        while !self.should_stop(target_area) {
            self.take_step();
            if self.y > max_height {
                max_height = self.y;
            }
            if target_area.contains(self) {
                return Some(max_height);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct TargetArea {
    start_x: i32,
    end_x: i32,
    start_y: i32,
    end_y: i32,
}
impl TargetArea {
    pub fn new(input: &str) -> Self {
        let start_idx = input.find("x=").unwrap();
        let substring = &input[start_idx..];
        let definitions: Vec<String> = substring
            .split(", ")
            .map(|x| x.replace("x=", "").replace("y=", ""))
            .collect();
        let se_se: Vec<i32> = definitions
            .iter()
            .map(|x| {
                x.split("..")
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .flatten()
            .collect();
        TargetArea {
            start_x: se_se[0],
            end_x: se_se[1],
            start_y: se_se[2],
            end_y: se_se[3],
        }
    }

    pub fn contains(&self, probe: &Probe) -> bool {
        (self.start_x..=self.end_x).contains(&probe.x)
            && (self.start_y..=self.end_y).contains(&probe.y)
    }

    pub fn get_min_height(&self) -> i32 {
        cmp::min(self.start_y, self.end_y)
    }
}

fn part_one(input: &[String], x_vals: (i32, i32), y_vals: (i32, i32)) -> i32 {
    let target_area = TargetArea::new(input[0].as_str());
    let velocities: Vec<(i32, i32)> = (x_vals.0..x_vals.1)
        .map(|x| {
            (y_vals.0..y_vals.1)
                .map(|y| (x, y))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect::<Vec<(i32, i32)>>();
    velocities
        .iter()
        .map(|velocity| {
            let mut probe = Probe::new();
            probe.max_height(velocity, &target_area)
        })
        .flatten()
        .max()
        .unwrap()
}

fn part_two(input: &[String], x_vals: (i32, i32), y_vals: (i32, i32)) -> usize {
    let target_area = TargetArea::new(input[0].as_str());
    let velocities: Vec<(i32, i32)> = (x_vals.0..x_vals.1)
        .map(|x| {
            (y_vals.0..y_vals.1)
                .map(|y| (x, y))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .collect::<Vec<(i32, i32)>>();
    velocities
        .iter()
        .map(|velocity| {
            let mut probe = Probe::new();
            if probe.max_height(velocity, &target_area).is_some() {
                Some(velocity)
            } else {
                None
            }
        })
        .flatten()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "target area: x=20..30, y=-10..-5"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input), (1, 100), (1, 100));
        let truth = 45;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input), (1, 100), (-50, 50));
        let truth = 112;

        assert_eq!(output, truth);
    }
}
