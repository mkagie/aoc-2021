use std::{cmp, collections::HashMap, hash::Hash};

use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_twenty-one/day_twenty-one.txt";
    let input = file_to_string_vec(filename);

    println!("{}", part_one(&input));
    println!("{}", part_two(&input));
}

trait Die {
    fn roll(&mut self) -> u32;
    fn roll_3x(&mut self) -> u32;
    fn n_rolls(&self) -> u32;
}

struct DeterministicDie {
    value: u32,
    n_rolls: u32,
}
impl DeterministicDie {
    pub fn new() -> Self {
        DeterministicDie {
            value: 1,
            n_rolls: 0,
        }
    }

    pub fn roll_x_times(&mut self, times: usize) -> Vec<u32> {
        let mut vec = Vec::new();
        (0..times).for_each(|_| {
            vec.push(self.roll());
        });
        vec
    }
}

impl Die for DeterministicDie {
    fn roll(&mut self) -> u32 {
        let value = self.value;
        self.value += 1;
        self.n_rolls += 1;
        value
    }

    fn roll_3x(&mut self) -> u32 {
        self.roll_x_times(3).iter().sum()
    }

    fn n_rolls(&self) -> u32 {
        self.n_rolls
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
struct Player {
    position: u32,
    score: u32,
}
impl Player {
    pub fn new(position: u32, score: Option<u32>) -> Self {
        let score = score.map_or_else(|| 0, |s| s);
        Player { position, score }
    }

    pub fn move_spaces(&mut self, n_spaces: u32) {
        let mut new_position = self.position + n_spaces;
        while new_position > 10 {
            new_position -= 10;
        }
        self.position = new_position;
        self.score += new_position;
    }
}

struct Game<D: Die> {
    player0: Player,
    player1: Player,
    die: D,
    end_score: u32,
}
impl<D> Game<D>
where
    D: Die,
{
    pub fn new(position_0: u32, position_1: u32, end_score: u32, die: D) -> Self {
        Game {
            player0: Player::new(position_0, None),
            player1: Player::new(position_1, None),
            die,
            end_score,
        }
    }

    fn take_turn(&mut self) -> bool {
        // Player 1 takes a turn
        let moves0 = self.die.roll_3x();
        self.player0.move_spaces(moves0);
        if self.player0.score >= self.end_score {
            return true;
        }
        // Player 2 takes a turn
        let moves1 = self.die.roll_3x();
        self.player1.move_spaces(moves1);
        if self.player1.score >= self.end_score {
            return true;
        }
        false
    }

    pub fn play_game(&mut self) -> (u32, u32, u32) {
        while !self.take_turn() {}
        (self.player0.score, self.player1.score, self.die.n_rolls())
    }
}

fn parse_input(input: &[String]) -> Vec<u32> {
    input
        .iter()
        .map(|x| x.chars().last().unwrap().to_digit(10).unwrap())
        .collect::<Vec<u32>>()
}

fn compute_likelihoods() -> HashMap<u64, u64> {
    let mut h = HashMap::new();
    (1..4).for_each(|r0| {
        (1u64..4).for_each(|r1| {
            (1..4).for_each(|r2| {
                let s = r0 + r1 + r2;
                if let Some(v) = h.get_mut(&s) {
                    *v += 1;
                } else {
                    h.insert(s, 1u64);
                }
            })
        })
    });
    h
}

fn part_one(input: &[String]) -> u32 {
    let positions = parse_input(input);
    let mut game = Game::new(positions[0], positions[1], 1000, DeterministicDie::new());
    let (score0, score1, n_rolls) = game.play_game();
    let min_score = cmp::min(score0, score1);
    min_score * n_rolls
}

fn part_two(input: &[String]) -> u64 {
    let positions = parse_input(input);
    let player0 = Player::new(positions[0], None);
    let player1 = Player::new(positions[1], None);
    let mut n_wins_0 = 0u64;
    let mut n_wins_1 = 0u64;
    let likelihoods = compute_likelihoods();
    let points_to_win = 21;

    let mut hash = HashMap::from([((player0, player1), 1u64)]);
    while !hash.is_empty() {
        // Iterate through each game, remove from the hash table
        let mut new_hash = HashMap::new();
        for ((player0, player1), n_games) in hash.into_iter() {
            // Roll for player0
            for (position_adjustment0, n_games_position_adjusted0) in likelihoods.iter() {
                let mut p0 = player0.clone();
                p0.move_spaces(*position_adjustment0 as u32);
                let n_games_to_this_point = n_games * n_games_position_adjusted0;
                if p0.score >= points_to_win {
                    n_wins_0 += n_games_to_this_point;
                } else {
                    // Roll for player1
                    for (position_adjustemnt1, n_games_position_adjusted1) in likelihoods.iter() {
                        let mut p1 = player1.clone();
                        p1.move_spaces(*position_adjustemnt1 as u32);
                        let n_games_to_this_point =
                            n_games_to_this_point * n_games_position_adjusted1;
                        if p1.score >= points_to_win {
                            n_wins_1 += n_games_to_this_point;
                        } else {
                            if let Some(v) = new_hash.get_mut(&(p0, p1)) {
                                *v += n_games_to_this_point;
                            } else {
                                new_hash.insert((p0, p1), n_games_to_this_point);
                            }
                        }
                    }
                }
            }
        }
        hash = new_hash;
    }
    cmp::max(n_wins_0, n_wins_1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> &'static str {
        "Player 1 starting position: 4
Player 2 starting position: 8
"
    }

    #[test]
    fn test_one() {
        let input = input();
        let output = part_one(&str_to_string_vec(&input));
        let truth = 739785;

        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = input();
        let output = part_two(&str_to_string_vec(&input));
        let truth = 444356092776315;

        assert_eq!(output, truth);
    }
}
