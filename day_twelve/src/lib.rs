use petgraph::graphmap::UnGraphMap;
use std::collections::HashMap;

use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_twelve/day_twelve.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

#[allow(clippy::derive_hash_xor_eq)]
#[derive(Debug, Copy, Clone, PartialOrd, Eq, Ord, Hash)]
enum CaveSize {
    Big,
    Small(u8),
}
impl PartialEq for CaveSize {
    fn eq(&self, other: &Self) -> bool {
        use CaveSize::*;
        matches!((self, other), (&Big, &Big) | (&Small(_), &Small(_)))
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Copy, Clone, Ord, Hash)]
struct Node<'a> {
    id: &'a str,
    size: CaveSize,
}

impl Node<'_> {
    pub fn new(id: &str) -> Node {
        let size = {
            if id.to_lowercase() == id {
                CaveSize::Small(0)
            } else {
                CaveSize::Big
            }
        };
        Node { id, size }
    }
}

fn parse_input(input: &[String]) -> UnGraphMap<Node, ()> {
    let mut graph = UnGraphMap::<Node, ()>::new();
    input.iter().for_each(|x| {
        let splits: Vec<&str> = x.split('-').collect();
        let idx0 = splits[0];
        let p0 = Node::new(idx0);
        let idx1 = splits[1];
        let p1 = Node::new(idx1);
        graph.add_edge(p0, p1, ());
    });
    graph
}

fn recursive_traversal<'a, 'b>(
    mut node: Node<'a>,
    graph: &UnGraphMap<Node<'a>, ()>,
    hash: &'b mut HashMap<&'a str, Node<'a>>,
    small_cave_has_been_traveled_twice: &mut bool,
) -> usize {
    // If we are at the end, mark that this is a viable path
    if node.id == "end" {
        return 1;
    }

    // Look at each neighbor
    let neighbors = graph.neighbors(node);
    let mut this_node_visited_twice = false;

    // Check to see if we have an updated node in the hash
    if hash.contains_key(node.id) {
        node = hash.get_mut(&node.id).unwrap().to_owned();
    }

    // Indicate that we've visited here
    if let CaveSize::Small(n_visits) = node.size {
        if n_visits == 0 {
            node.size = CaveSize::Small(1);
            // Put it back in the hash as having visited
            hash.insert(node.id, node);
        } else if n_visits == 1 {
            if *small_cave_has_been_traveled_twice || node.id == "start" {
                // Cannot visit here again
                return 0;
            } else {
                node.size = CaveSize::Small(2);
                hash.insert(node.id, node);
                *small_cave_has_been_traveled_twice = true;
                this_node_visited_twice = true;
            }
        } else {
            return 0;
        }
    }

    // Compute the number of viable paths from each neighbor
    let n_viable_paths: usize = neighbors
        .filter(|x| x.id != "start")
        .map(|n| recursive_traversal(n, graph, hash, small_cave_has_been_traveled_twice))
        .sum();

    // Revert that we've been here, so other searches can also see this
    if let CaveSize::Small(count) = node.size {
        if count == 2 {
            node.size = CaveSize::Small(1);
            hash.insert(node.id, node);
        } else {
            hash.remove(node.id);
        }
    }
    if this_node_visited_twice {
        *small_cave_has_been_traveled_twice = false;
    }

    n_viable_paths
}

fn part_one(input: &[String]) -> usize {
    let graph = parse_input(input);
    let node = Node::new("start");
    let mut hash: HashMap<&str, Node> = HashMap::new();
    let mut b = true;
    recursive_traversal(node, &graph, &mut hash, &mut b)
}

fn part_two(input: &[String]) -> usize {
    let graph = parse_input(input);
    let node = Node::new("start");
    let mut hash: HashMap<&str, Node> = HashMap::new();
    let mut b = false;
    recursive_traversal(node, &graph, &mut hash, &mut b)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 226;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_enum_eq() {
        assert_eq!(CaveSize::Small(0), CaveSize::Small(1));
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 3509;

        assert_eq!(output, truth);
    }
}
