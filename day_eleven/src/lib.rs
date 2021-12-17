use mkagie_utils::*;
use ndarray::prelude::*;
use ndarray::Array;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_eleven/day_eleven.txt";
    let lines = file_chars_to_int_vec::<u8>(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

fn process_inputs(input: Vec<Vec<u8>>) -> Array<u8, Ix2> {
    Array::from_iter(input.into_iter().flatten())
        .into_shape((10, 10))
        .unwrap()
}

fn flash(
    input: &mut Array<u8, Ix2>,
    has_flashed: &mut Array<bool, Ix2>,
    row_idx: usize,
    col_idx: usize,
) {
    if has_flashed[(row_idx, col_idx)] {
        return;
    }
    has_flashed[(row_idx, col_idx)] = true;

    let mut idxs = Vec::new();
    if row_idx > 0 && col_idx > 0 {
        idxs.push((row_idx - 1, col_idx - 1));
    }
    if row_idx > 0 {
        idxs.extend(vec![(row_idx - 1, col_idx), (row_idx - 1, col_idx + 1)])
    }
    if col_idx > 0 {
        idxs.extend(vec![(row_idx, col_idx - 1), (row_idx + 1, col_idx - 1)]);
    }
    // The rest, we don't have to worry about zero subtracting
    idxs.extend(vec![
        (row_idx, col_idx + 1),
        (row_idx + 1, col_idx),
        (row_idx + 1, col_idx + 1),
    ]);
    idxs.iter().for_each(|(row_idx, col_idx)| {
        if let Some(val) = input.get_mut((*row_idx, *col_idx)) {
            *val += 1;
            if *val > 9 {
                flash(input, has_flashed, *row_idx, *col_idx)
            }
        }
    });
}

fn take_step(input: &mut Array<u8, Ix2>) -> usize {
    let mut has_flashed = Array::from_iter([false; 100].into_iter())
        .into_shape((10, 10))
        .unwrap();

    for row_idx in 0..input.nrows() {
        for col_idx in 0..input.ncols() {
            let val = input.get_mut((row_idx, col_idx)).unwrap();
            *val += 1;

            if *val > 9 {
                flash(input, &mut has_flashed, row_idx, col_idx);
            }
        }
    }

    // Reset every one that flashed back to zero
    has_flashed
        .indexed_iter()
        .for_each(|((row_idx, col_idx), &val)| {
            if val {
                *input.get_mut((row_idx, col_idx)).unwrap() = 0;
            }
        });

    has_flashed
        .iter()
        .fold(0, |acc, &x| if x { acc + 1 } else { acc })
}

fn part_one(input: &[Vec<u8>]) -> usize {
    // Create a 10 by 10 array
    let mut arr = process_inputs(input.to_vec());
    // Create a function that completes a step
    (0..100).map(|_| take_step(&mut arr)).sum()
}

fn part_two(input: &[Vec<u8>]) -> usize {
    // Create a 10 by 10 array
    let mut arr = process_inputs(input.to_vec());
    let mut counter = 0;
    loop {
        counter += 1;
        if take_step(&mut arr) == 100 {
            break;
        }
    }
    counter
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_chars_to_int_vec(&input));
        let truth = 1656;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_chars_to_int_vec(&input));
        let truth = 195;

        assert_eq!(output, truth);
    }
}
