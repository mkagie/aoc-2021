use mkagie_utils::*;
use ndarray::prelude::*;
use ndarray::Array;

fn part_one(input: &[String]) -> isize {
    let n_cols = input[0].len();
    let n_rows = input.len();

    // Construct binary array
    let mut array = Array::zeros((n_rows, n_cols));
    input.iter().enumerate().for_each(|(r_idx, row)| {
        row.chars()
            .enumerate()
            .for_each(|(c_idx, col)| array[[r_idx, c_idx]] = col.to_digit(10).unwrap() as f64)
    });

    let array_sums = array.sum_axis(Axis(0));
    let msb = array_sums.map(|&x| x / n_rows as f64 > 0.5);
    let lsb = msb.map(|&x| !x);

    let gamma = isize::from_str_radix(
        &String::from_iter(msb.iter().map(|x| if *x { "1" } else { "0" })),
        2,
    )
    .unwrap();

    let epsilon = isize::from_str_radix(
        &String::from_iter(lsb.iter().map(|x| if *x { "1" } else { "0" })),
        2,
    )
    .unwrap();

    gamma * epsilon
}

fn part_two(input: &[String]) -> isize {
    let n_cols = input[0].len();
    let n_rows = input.len();

    // Construct binary array
    let mut array = Array::zeros((n_rows, n_cols));
    input.iter().enumerate().for_each(|(r_idx, row)| {
        row.chars()
            .enumerate()
            .for_each(|(c_idx, col)| array[[r_idx, c_idx]] = col.to_digit(10).unwrap() as f64)
    });

    // Keep only rows in the array where the first column matches og_vec
    let mut col_idx = 0;
    let mut array_two = array.clone();
    while array.len_of(Axis(0)) > 1 && col_idx < n_cols {
        // Compute the msb
        let msb = array
            .sum_axis(Axis(0))
            .map(|&x| (x / array.len_of(Axis(0)) as f64 >= 0.5) as u64 as f64);
        let column = array.column(col_idx);
        let valid_rows_og: Vec<usize> = column
            .iter()
            .enumerate()
            .map(|(idx, &x)| if x == msb[col_idx] { Some(idx) } else { None })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        array = array.select(Axis(0), &valid_rows_og);
        col_idx += 1;
    }
    let oxygen = isize::from_str_radix(
        &String::from_iter(array.iter().map(|&x| if x == 1 as f64 { "1" } else { "0" })),
        2,
    )
    .unwrap();

    let mut col_idx = 0;
    while array_two.len_of(Axis(0)) > 1 && col_idx < n_cols {
        let lsb = array_two
            .sum_axis(Axis(0))
            .map(|&x| (x / (array_two.len_of(Axis(0)) as f64) < 0.5) as u64 as f64);
        let column = array_two.column(col_idx);
        let valid_rows_co: Vec<usize> = column
            .iter()
            .enumerate()
            .map(|(idx, &x)| if x == lsb[col_idx] { Some(idx) } else { None })
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect();
        array_two = array_two.select(Axis(0), &valid_rows_co);
        col_idx += 1;
    }
    let co2 = isize::from_str_radix(
        &String::from_iter(
            array_two
                .iter()
                .map(|&x| if x == 1 as f64 { "1" } else { "0" }),
        ),
        2,
    )
    .unwrap();

    oxygen * co2
}

pub fn run() {
    let filename = "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_three/day_three.txt";
    let input = file_to_string_vec(filename);

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let output = part_one(&str_array_to_vec(&input));
        let truth = 198;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        let output = part_two(&str_array_to_vec(&input));
        let truth = 230;

        assert_eq!(output, truth);
    }
}
