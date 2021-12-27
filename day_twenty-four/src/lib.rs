use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use threadpool::ThreadPool;

use mkagie_utils::*;

pub fn run() {
    let filename = "day_twenty-four.txt";
    let input = file_to_string_vec(filename);

    println!("{}", part_one(&input));
    // println!("{}", part_two(&input));
}

fn parse_input(input: &[String]) -> Vec<Instruction> {
    input.iter().map(|x| Instruction::new(x.as_ref())).collect()
}

/// Function to take in a 14 bit number and produce a vector on integers
fn convert_int_to_vec(mut input: i64) -> Vec<i64> {
    let mut v = Vec::new();
    while input > 0 {
        let val = input % 10;
        v.push(val);
        input = input / 10;
    }
    while v.len() < 14 {
        v.push(0);
    }
    v.reverse();
    v
}

#[derive(Debug, Clone)]
enum Var1<'a> {
    Var(&'a str),
    Int(i64),
}

#[derive(Clone)]
struct Instruction<'a> {
    cmd: &'a str,
    var0: &'a str,
    var1: Option<Var1<'a>>,
}
impl<'a> Instruction<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut splits = input.split_ascii_whitespace();
        let cmd = splits.next().unwrap();
        let var0 = splits.next().unwrap();
        let var1 = if let Some(v) = splits.next() {
            if let Ok(int) = v.parse::<i64>() {
                Some(Var1::Int(int))
            } else {
                Some(Var1::Var(v))
            }
        } else {
            None
        };

        Self { cmd, var0, var1 }
    }

    pub fn do_operation<T>(&self, variables: &mut HashMap<&'a str, i64>, inputs: &mut T)
    where
        T: Iterator<Item = i64>,
    {
        if self.cmd == "inp" {
            self.inp_a(variables, inputs);
        } else if self.cmd == "add" {
            self.do_var_op(|a, b| a + b, variables);
        } else if self.cmd == "mul" {
            self.do_var_op(|a, b| a * b, variables);
        } else if self.cmd == "div" {
            self.do_var_op(|a, b| a / b, variables);
        } else if self.cmd == "mod" {
            self.do_var_op(|a, b| a % b, variables);
        } else if self.cmd == "eql" {
            self.do_var_op(|a, b| if a == b { 1 } else { 0 }, variables);
        }
        // if self.var0 == "z" || matches!(self.var1, Some(Var1::Var("z"))) {
        //     println!(
        //         "{} {} {:?} -- {:?}",
        //         self.cmd,
        //         self.var0,
        //         self.var1.as_ref(),
        //         variables.get("z")
        //     );
        // }
    }

    fn inp_a<T>(&self, variables: &mut HashMap<&'a str, i64>, inputs: &mut T)
    where
        T: Iterator<Item = i64>,
    {
        let val = inputs.next().unwrap();
        variables.insert(self.var0, val);
    }

    fn do_var_op(&self, op: fn(i64, i64) -> i64, variables: &mut HashMap<&'a str, i64>) {
        let var1 = self.var1.as_ref().unwrap();
        let b = match var1 {
            Var1::Int(b) => b.to_owned(),
            Var1::Var(s) => variables.get(s).unwrap_or(&0).to_owned(),
        };
        if let Some(a) = variables.get_mut(self.var0) {
            *a = op(*a, b);
        } else {
            let a = op(0, b);
            variables.insert(self.var0, a);
        }
    }
}

#[derive(Clone)]
struct Program<'a> {
    instructions: Vec<Instruction<'a>>,
    variables: HashMap<&'a str, i64>,
}
impl<'a> Program<'a> {
    pub fn new(instructions: Vec<Instruction<'a>>) -> Self {
        Program {
            instructions,
            variables: HashMap::new(),
        }
    }

    pub fn reset(&mut self) {
        self.variables = HashMap::new();
    }

    pub fn run<T>(&mut self, inputs: &mut T) -> i64
    where
        T: Iterator<Item = i64>,
    {
        self.instructions
            .iter()
            .for_each(|i| i.do_operation(&mut self.variables, inputs));
        self.variables.get("z").unwrap().to_owned()
    }
}

fn part_one(input: &'static [String]) -> i64 {
    let instructions = parse_input(input);
    let mut input: i64 = 99999999999999;
    let program = Program::new(instructions);
    let mut counter = 0;
    let pool = ThreadPool::new(20);
    let (tx, rx) = channel();
    while input > 0 {
        let input_ = input;
        let mut p = program.clone();
        let tx = tx.clone();
        let c = counter;
        pool.execute(move || {
            let mut inputs = convert_int_to_vec(input_).into_iter();
            let return_code = p.run(&mut inputs);
            if (c + 1) % 10000 == 0 {
                println!("{} -- {}", input_, return_code);
            }
            tx.send((return_code, input_));
        });
        input -= 1;
        counter += 1;
    }

    let rx_handle = thread::spawn(move || {
        for (return_code, number) in rx {
            if return_code == 0 {
                println!("Solution -- {}", number);
            }
        }
    });

    pool.join();
    rx_handle.join().unwrap();
    input
}

fn part_two(input: &[String]) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_convert_to_vec() {
        let input = 123456789;
        let output = convert_int_to_vec(input);
        assert_eq!(output, vec![0, 0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    fn input() -> &'static str {
        ""
    }

    // #[test]
    // fn test_one() {
    //     let input = input();
    //     let output = part_one(&str_to_string_vec(&input));
    //     let truth = 590784;

    //     assert_eq!(output, truth);
    // }
    // #[test]
    // fn test_two() {
    //     let input = input();
    //     let output = part_two(&str_to_string_vec(&input));
    //     let truth = 2758514936282235;

    //     assert_eq!(output, truth);
    // }
}
