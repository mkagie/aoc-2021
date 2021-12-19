use mkagie_utils::*;
use petgraph::{
    algo::{astar, dijkstra},
    graphmap::DiGraphMap,
};

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_fifteen/day_fifteen.txt";
    let lines = file_chars_to_int_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

fn process_inputs(input: &[Vec<u8>]) -> DiGraphMap<(usize, usize), u32> {
    let mut graph = DiGraphMap::new();
    let n_rows = input.len();
    let n_cols = input[0].len();
    input.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, _)| {
            // Add an edge left, right, up, down
            let mut edge_nodes = Vec::new();
            if col_idx > 0 {
                // Add left
                edge_nodes.push((row_idx, col_idx - 1));
            }
            if row_idx > 0 {
                // Add up
                edge_nodes.push((row_idx - 1, col_idx));
            }
            if col_idx < n_cols - 1 {
                // Add right
                edge_nodes.push((row_idx, col_idx + 1));
            }
            if row_idx < n_rows - 1 {
                // Add down
                edge_nodes.push((row_idx + 1, col_idx));
            }
            edge_nodes.into_iter().for_each(|(r, c)| {
                let weight = input[r][c] as u32;
                graph.add_edge((row_idx, col_idx), (r, c), weight);
            });
        })
    });
    graph
}

fn part_one(input: &[Vec<u8>]) -> u32 {
    let n_rows = input.len();
    let n_cols = input[0].len();
    let graph = process_inputs(input);

    // Let's try dijkstra
    let res = dijkstra(&graph, (0, 0), Some((n_rows - 1, n_cols - 1)), |x| *x.2);
    *res.get(&(n_rows - 1, n_cols - 1)).unwrap()
}

fn increment_tile(tile: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut output = Vec::new();
    // Do the 5 column copies
    tile.iter().for_each(|row| {
        let mut row_vec = Vec::new();
        // Append tile
        row.iter().for_each(|&val| {
            row_vec.push(val);
        });
        (1..5).for_each(|inc| {
            row.iter().for_each(|val| {
                let mut val = val + inc;
                if val > 9 {
                    val -= 9;
                }
                row_vec.push(val);
            });
        });
        output.push(row_vec);
    });
    // Do the 5 row copies
    (1..5).for_each(|inc| {
        tile.iter().for_each(|row| {
            let mut row_vec = Vec::new();

            // Append tile + inc
            row.iter().for_each(|val| {
                let mut val = val + inc;
                if val > 9 {
                    val -= 9;
                }
                row_vec.push(val);
            });
            // Append tile + inc + inc2
            (1..5).for_each(|inc2| {
                row.iter().for_each(|val| {
                    let mut val = val + inc + inc2;
                    if val > 9 {
                        val -= 9;
                    }
                    row_vec.push(val);
                });
            });
            output.push(row_vec);
        });
    });
    assert_eq!(output.len(), 5 * tile.len());
    output.iter().for_each(|x| {
        assert_eq!(x.len(), 5 * tile.len());
    });
    output
}

fn part_two(input: &[Vec<u8>]) -> u32 {
    let input = increment_tile(input);
    let n_rows = input.len();
    let n_cols = input[0].len();
    let graph = process_inputs(&input);

    // Let's try dijkstra
    astar(
        &graph,
        (0, 0),
        |finish| finish == (n_rows - 1, n_cols - 1),
        |x| *x.2,
        |x| (n_rows - 1 - x.0 + n_cols - 1 - x.1) as u32,
    )
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_chars_to_int_vec(&input));
        let truth = 40;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_chars_to_int_vec(&input));
        let truth = 315;

        assert_eq!(output, truth);
    }
}
