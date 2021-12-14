use mkagie_utils::*;
use std::collections::HashMap;

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_nine/day_nine.txt";
    let mut lines = file_chars_to_int_vec::<u32>(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&mut lines));
}

#[derive(PartialEq, Clone, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    Min,
    Nine,
}

#[derive(Clone, Debug)]
struct Point {
    pub row: usize,
    pub col: usize,
    pub val: u32,
    pub gradient: Option<Direction>,
    pub center: Option<(usize, usize)>,
}

type CountMap = HashMap<(usize, usize), i32>;
type PointMap = HashMap<(usize, usize), Point>;

fn find_centers_and_counts(input: &[Vec<u32>]) -> (CountMap, PointMap) {
    // Create a grid of Points, which contain a direction vector and a Center
    let num_rows = input.len();
    let num_cols = input[0].len();

    let mut unprocessed = HashMap::new();
    let mut processed = HashMap::new();
    let mut centers = HashMap::new();

    input.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, &val)| {
            unprocessed.insert(
                (row_idx, col_idx),
                Point {
                    row: row_idx,
                    col: col_idx,
                    val,
                    gradient: None,
                    center: None,
                },
            );
        })
    });

    while !unprocessed.is_empty() {
        // Get the first item
        let mut point = unprocessed.iter_mut().next().unwrap().1.clone();
        unprocessed.remove(&(point.row, point.col));

        let mut points_touched: Vec<Point> = Vec::new();
        while point.center.is_none() {
            let val = point.val;
            if val == 9 {
                point.gradient = Some(Direction::Nine);
                processed.insert((point.row, point.col), point.clone());
                break;
            }
            // Check up, down, left, and right -- indicate gradient as largest drop
            let mut grad = 0u32;
            let mut direction = Direction::Min;
            if point.row > 0 {
                let top = input[point.row - 1][point.col];
                if val > top && val - top > grad {
                    grad = val - top;
                    direction = Direction::Up;
                }
            }
            if point.row < num_rows - 1 {
                let bottom = input[point.row + 1][point.col];
                if val > bottom && val - bottom > grad {
                    grad = val - bottom;
                    direction = Direction::Down;
                }
            }
            if point.col > 0 {
                let left = input[point.row][point.col - 1];
                if val > left && val - left > grad {
                    grad = val - left;
                    direction = Direction::Left;
                }
            }
            if point.col < num_cols - 1 {
                let right = input[point.row][point.col + 1];
                if val > right && val - right > grad {
                    direction = Direction::Right;
                }
            }
            point.gradient = Some(direction);

            if point.gradient == Some(Direction::Min) {
                let center = Some((point.row, point.col));
                // Update self
                point.center = center;
                processed.insert((point.row, point.col), point.clone());
                centers.insert((point.row, point.col), 1);

                let center_counter = centers.get_mut(&(point.row, point.col)).unwrap();
                // Update the rest
                for p in points_touched.iter_mut() {
                    p.center = center;
                    *center_counter += 1;
                    processed.insert((p.row, p.col), p.clone());
                }
            } else {
                // Move in direction of gradient
                let (row, col) = match point.gradient {
                    Some(Direction::Left) => (point.row, point.col - 1),
                    Some(Direction::Right) => (point.row, point.col + 1),
                    Some(Direction::Up) => (point.row - 1, point.col),
                    Some(Direction::Down) => (point.row + 1, point.col),
                    _ => (point.row, point.col),
                };
                if let Some(val) = unprocessed.remove(&(row, col)) {
                    points_touched.push(point);
                    point = val;
                } else {
                    // That point has already been processed, just take the center and update all others
                    let center = processed.get(&(row, col)).unwrap().center;
                    point.center = center;
                    let center_counter = centers.get_mut(&center.unwrap()).unwrap();
                    *center_counter += 1;
                    processed.insert((point.row, point.col), point.clone());
                    // Update the rest
                    for p in points_touched.iter_mut() {
                        p.center = center;
                        *center_counter += 1;
                        processed.insert((p.row, p.col), p.clone());
                    }
                }
            }
        }
    }
    (centers, processed)
}

fn part_one(input: &[Vec<u32>]) -> u32 {
    let (centers, points) = find_centers_and_counts(input);
    let center_points: Vec<Point> = centers
        .iter()
        .map(|x| points.get(x.0).unwrap().clone())
        .collect();
    center_points
        .iter()
        .fold(0, |accum, val| accum + input[val.row][val.col] + 1)
}

fn part_two(input: &mut Vec<Vec<u32>>) -> i32 {
    let (centers, _) = find_centers_and_counts(input);
    let mut vec: Vec<i32> = centers.iter().map(|x| *x.1).collect();
    vec.sort_unstable();
    vec.reverse();
    vec.iter().take(3).product::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "2199943210
3987894921
9856789892
8767896789
9899965678"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&mut str_chars_to_int_vec::<u32>(&input));
        let truth = 15;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&mut str_chars_to_int_vec::<u32>(&input));
        let truth = 1134;

        assert_eq!(output, truth);
    }
}
