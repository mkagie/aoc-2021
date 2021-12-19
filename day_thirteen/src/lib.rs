use plotters::prelude::*;
use std::collections::HashSet;

use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_thirteen/day_thirteen.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}
impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }

    pub fn fold(&mut self, fold: &Fold) {
        match fold {
            Fold::X(val) => {
                // Fold left -- if x < val, nothing happens
                if self.x > *val {
                    let dist_from_fold = self.x - *val;
                    self.x = *val - dist_from_fold;
                }
            }
            Fold::Y(val) => {
                // Fold up -- if y < val, nothing happens
                if self.y > *val {
                    let dist_from_fold = self.y - *val;
                    self.y = *val - dist_from_fold;
                }
            }
        }
    }

    pub fn for_plot(&self) -> (i32, i32) {
        (self.x as i32, self.y as i32)
    }
}

#[derive(Clone, Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

enum ParsedLine {
    Point(Point),
    Fold(Fold),
}

fn process_inputs(input: &[String]) -> (Vec<Point>, Vec<Fold>) {
    let parsed_lines: Vec<ParsedLine> = input
        .iter()
        .map(|line| {
            if line.is_empty() {
                None
            } else if line.starts_with("fold") {
                let definition = line.split(' ').collect::<Vec<&str>>()[2];
                let val = definition.split('=').collect::<Vec<&str>>()[1]
                    .parse::<u32>()
                    .unwrap();
                if definition.starts_with('x') {
                    Some(ParsedLine::Fold(Fold::X(val)))
                } else {
                    Some(ParsedLine::Fold(Fold::Y(val)))
                }
            } else {
                let values: Vec<u32> = line.split(',').map(|x| x.parse::<u32>().unwrap()).collect();
                Some(ParsedLine::Point(Point::new(values[0], values[1])))
            }
        })
        .flatten()
        .collect();

    let points: Vec<Point> = parsed_lines
        .iter()
        .map(|x| {
            if let ParsedLine::Point(point) = x {
                Some(point.to_owned())
            } else {
                None
            }
        })
        .flatten()
        .collect();
    let folds: Vec<Fold> = parsed_lines
        .iter()
        .map(|x| {
            if let ParsedLine::Fold(fold) = x {
                Some(fold.to_owned())
            } else {
                None
            }
        })
        .flatten()
        .collect();
    (points, folds)
}

fn part_one(input: &[String]) -> usize {
    let (points, folds) = process_inputs(input);

    // Fold just part one
    let fold = &folds[0];
    let mut set = HashSet::new();
    points.into_iter().for_each(|mut p| {
        p.fold(fold);
        set.insert(p);
    });
    set.len()
}

fn part_two(input: &[String]) {
    let (points, folds) = process_inputs(input);
    let mut points_one = HashSet::new();
    points.into_iter().for_each(|p| {
        let _ = points_one.insert(p);
    });
    let mut points_two = HashSet::new();
    folds.iter().for_each(|fold| {
        if points_two.is_empty() {
            // Iterate through pointsOne, add them to pointsTwo, empty pointsOne
            points_one.iter().for_each(|p| {
                let mut p = p.to_owned();
                p.fold(fold);
                points_two.insert(p);
            });
            points_one.clear();
        } else {
            points_two.iter().for_each(|p| {
                let mut p = p.to_owned();
                p.fold(fold);
                points_one.insert(p);
            });
            points_two.clear();
        }
    });
    let output_points = if points_one.is_empty() {
        points_two
    } else {
        points_one
    };

    // Need to plot it
    let drawing_area = BitMapBackend::new(
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_thirteen/day_thirteen.png",
        (1024, 768),
    )
    .into_drawing_area();
    let mut chart = ChartBuilder::on(&drawing_area)
        .build_cartesian_2d(0..80, 0..40)
        .unwrap();
    chart
        .draw_series(
            output_points
                .iter()
                .map(|x| TriangleMarker::new(x.for_plot(), 5, &BLUE)),
        )
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 17;

        assert_eq!(output, truth);
    }
}
