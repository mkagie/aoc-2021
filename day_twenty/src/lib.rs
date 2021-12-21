use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_twenty/day_twenty.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

fn convert_to_int(c: char) -> u32 {
    if c == '.' {
        0
    } else {
        1
    }
}

fn parse_input(input: &[String]) -> (Vec<u32>, Vec<Vec<u32>>) {
    let decoder: Vec<u32> = input[0].chars().map(convert_to_int).collect();

    let image: Vec<Vec<u32>> = input[2..]
        .iter()
        .map(|x| x.chars().map(convert_to_int).collect())
        .collect();
    (decoder, image)
}

fn get_value_at_row_col(row: usize, col: usize, image: &[Vec<u32>], val: u32) -> u32 {
    let n_rows = image.len();
    let n_cols = image[0].len();
    let mut bit_vec = Vec::new();
    // UL
    if row > 0 && col > 0 {
        bit_vec.push(image[row - 1][col - 1]);
    } else {
        bit_vec.push(val);
    }
    // U
    if row > 0 {
        bit_vec.push(image[row - 1][col]);
    } else {
        bit_vec.push(val);
    }
    // UR
    if row > 0 && col < n_cols - 1 {
        bit_vec.push(image[row - 1][col + 1]);
    } else {
        bit_vec.push(val);
    }
    // L
    if col > 0 {
        bit_vec.push(image[row][col - 1]);
    } else {
        bit_vec.push(val);
    }
    // C
    bit_vec.push(image[row][col]);
    // R
    if col < n_cols - 1 {
        bit_vec.push(image[row][col + 1]);
    } else {
        bit_vec.push(val);
    }
    // LL
    if row < n_rows - 1 && col > 0 {
        bit_vec.push(image[row + 1][col - 1]);
    } else {
        bit_vec.push(val);
    }
    // D
    if row < n_rows - 1 {
        bit_vec.push(image[row + 1][col]);
    } else {
        bit_vec.push(val);
    }
    // LR
    if row < n_rows - 1 && col < n_cols - 1 {
        bit_vec.push(image[row + 1][col + 1]);
    } else {
        bit_vec.push(val);
    }

    bit_vec.iter().enumerate().fold(0u32, |acc, (idx, x)| {
        acc + *x as u32 * 2u32.pow(bit_vec.len() as u32 - 1 - idx as u32)
    })
}

fn pad(image: &[Vec<u32>], num: usize, val: u32) -> Vec<Vec<u32>> {
    let mut pad = Vec::new();
    (0..num).for_each(|_| pad.push(vec![val; image.len() + 2 * num]));
    image.iter().for_each(|row| {
        let mut output_row = Vec::new();
        (0..num).for_each(|_| output_row.push(val));
        output_row.extend(row);
        (0..num).for_each(|_| output_row.push(val));
        pad.push(output_row);
    });
    (0..num).for_each(|_| pad.push(vec![val; image.len() + 2 * num]));

    pad
}

fn do_transformation(image: &[Vec<u32>], decoder: &[u32], val: u32) -> Vec<Vec<u32>> {
    let mut output = Vec::new();

    // Zero pad input
    let image_pad = pad(image, 1, val);

    image_pad.iter().enumerate().for_each(|(row_idx, row)| {
        let mut output_row = Vec::new();
        row.iter().enumerate().for_each(|(col_idx, _)| {
            let idx = get_value_at_row_col(row_idx, col_idx, &image_pad, val);
            output_row.push(decoder[idx as usize]);
        });
        output.push(output_row);
    });

    output
}

fn part_one(input: &[String]) -> i32 {
    let (decoder, image) = parse_input(input);
    let output = do_transformation(&image, &decoder, 0);
    let output = do_transformation(&output, &decoder, decoder[0]);

    output
        .iter()
        .map(|row| row.iter().sum::<u32>())
        .sum::<u32>() as i32
}

fn part_two(input: &[String]) -> i32 {
    let (decoder, image) = parse_input(input);
    let mut output = image;
    (0..50).for_each(|it| {
        let val = {
            if it == 0 {
                0
            } else if it % 2 == 1 {
                decoder[0]
            } else if decoder[0] == 1 {
                decoder[decoder.len() - 1]
            } else {
                0
            }
        };
        output = do_transformation(&output, &decoder, val);
    });
    output
        .iter()
        .map(|row| row.iter().sum::<u32>())
        .sum::<u32>() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 35;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 3351;

        assert_eq!(output, truth);
    }
}
