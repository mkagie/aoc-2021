use std::{array::IntoIter, cell::RefCell, str::Chars};

use mkagie_utils::*;

pub fn run() {
    let filename =
        "/home/mkagie/code/personal/adventOfCoding/advent-2021/day_sixteen/day_sixteen.txt";
    let lines = file_to_string_vec(filename);
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
}

#[derive(Debug, Clone)]
struct Message {
    version: u128,
    type_id: u128,
    data: Data,
    length_in_bits: u128,
}
impl Message {
    pub fn new(version: u128, type_id: u128, data: Data, length_in_bits: u128) -> Self {
        Message {
            version,
            type_id,
            data,
            length_in_bits,
        }
    }

    pub fn verify(&self) -> bool {
        if let Data::Operation(op) = &self.data {
            op.verify()
        } else {
            true
        }
    }

    pub fn get_versions(&self) -> Vec<u128> {
        let mut versions = vec![self.version];

        if let Data::Operation(ref op) = self.data {
            versions.extend(
                op.subpackets
                    .iter()
                    .map(|x| x.get_versions().iter().sum::<u128>()),
            );
        }
        versions
    }

    pub fn add_subpacket(&mut self, subpacket: Box<Message>) -> bool {
        if let Data::Operation(ref mut op) = self.data {
            self.length_in_bits += subpacket.length_in_bits;
            op.add_subpacket(subpacket)
        } else {
            true
        }
    }

    pub fn do_operation(&self) -> u128 {
        if let Data::Operation(op) = &self.data {
            op.do_operation()
        } else {
            println!("This is bad");
            0
        }
    }
}

#[derive(Debug, Clone)]
enum Data {
    Value(u128),
    Operation(Operation),
}

#[derive(Debug, Clone)]
struct Operation {
    length_in_bits: Option<u128>,
    number_subpackets: Option<u128>,
    operation: Op,
    subpackets: Vec<Box<Message>>,
    subpacket_bits: u128,
}

impl Operation {
    pub fn new(
        length_in_bits: Option<u128>,
        number_subpackets: Option<u128>,
        operation: u128,
    ) -> Self {
        Operation {
            length_in_bits,
            number_subpackets,
            operation: Operation::convert_operation(operation).unwrap(),
            subpackets: Vec::new(),
            subpacket_bits: 0,
        }
    }

    pub fn verify(&self) -> bool {
        if let Some(l) = self.length_in_bits {
            l == self.subpacket_bits
        } else {
            self.subpackets.len() as u128 == self.number_subpackets.unwrap()
        }
    }

    fn convert_operation(type_id: u128) -> Option<Op> {
        match type_id {
            0 => Some(Op::Sum),
            1 => Some(Op::Product),
            2 => Some(Op::Minimum),
            3 => Some(Op::Maximum),
            5 => Some(Op::GreaterThan),
            6 => Some(Op::LessThan),
            7 => Some(Op::Equal),
            _ => None,
        }
    }

    pub fn add_subpacket(&mut self, subpacket: Box<Message>) -> bool {
        if let Some(l) = self.length_in_bits {
            let future_l = self.subpacket_bits + subpacket.length_in_bits;
            if future_l <= l {
                self.subpackets.push(subpacket);
                self.subpacket_bits = future_l;
            }
        } else {
            let n_sp = self.number_subpackets.unwrap();
            if self.subpackets.len() < n_sp as usize {
                self.subpackets.push(subpacket);
            }
        }
        self.verify()
    }

    pub fn do_operation(&self) -> u128 {
        let subpacket_values: Vec<u128> = self
            .subpackets
            .iter()
            .map(|x| {
                if let Data::Value(y) = x.data {
                    y
                } else {
                    println!("Doing operation");
                    let ret = x.do_operation();
                    println!("Operation created {}", ret);
                    ret
                }
            })
            .collect();
        let output = match self.operation {
            Op::Sum => {
                let ret = subpacket_values.iter().sum::<u128>().to_owned();
                println!("{:?}.sum() = {}", subpacket_values, ret);
                ret
            }
            Op::Product => {
                let ret = subpacket_values.iter().product::<u128>().to_owned();
                println!("{:?}.prod() = {}", subpacket_values, ret);
                ret
            }
            Op::Minimum => {
                let ret = *subpacket_values.iter().min().unwrap();
                println!("{:?}.min() = {}", subpacket_values, ret);
                ret
            }
            Op::Maximum => {
                let ret = *subpacket_values.iter().max().unwrap();
                println!("{:?}.max() = {}", subpacket_values, ret);
                ret
            }
            Op::GreaterThan => {
                let ret = if subpacket_values[0] > subpacket_values[1] {
                    1
                } else {
                    0
                };
                println!("{:?}.greaterThan() = {}", subpacket_values, ret);
                ret
            }
            Op::LessThan => {
                let ret = if subpacket_values[0] < subpacket_values[1] {
                    1
                } else {
                    0
                };
                println!("{:?}.lessThan() = {}", subpacket_values, ret);
                ret
            }
            Op::Equal => {
                let ret = if subpacket_values[0] == subpacket_values[1] {
                    1
                } else {
                    0
                };
                println!("{:?}.eq() = {}", subpacket_values, ret);
                ret
            }
        };
        output
    }
}

#[derive(Debug, Clone)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

struct Factory<'a> {
    buffer: Vec<char>,
    chars: Chars<'a>,
}

impl<'a> Factory<'a> {
    pub fn new(input: &'a str) -> Self {
        Factory {
            buffer: Vec::new(),
            chars: input.chars(),
        }
    }

    fn convert_to_binary(c: &char) -> &str {
        match c {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => "",
        }
    }

    fn convert_binary_to_int(input_chars: &[char]) -> u128 {
        let n_chars = input_chars.len();

        input_chars
            .iter()
            .enumerate()
            .map(|(idx, c)| {
                if *c == '1' {
                    2u128.pow((n_chars - 1 - idx) as u32)
                } else {
                    0
                }
            })
            .sum()
    }

    fn expand_buffer(&mut self, size: usize) -> Option<()> {
        while self.buffer.len() < size {
            if let Some(c) = self.chars.next() {
                self.buffer.extend(
                    Factory::convert_to_binary(&c)
                        .chars()
                        .collect::<Vec<char>>(),
                );
            } else {
                return None;
            }
        }
        Some(())
    }

    fn convert_field(&mut self, size: usize) -> Option<u128> {
        if let Some(_) = self.expand_buffer(size) {
            let ret = Factory::convert_binary_to_int(&self.buffer[0..size]);
            self.buffer.drain(0..size);
            Some(ret)
        } else {
            None
        }
    }

    pub fn parse_input(&mut self) -> Vec<Message> {
        let mut output = Vec::new();
        loop {
            let mut bit_counter = 0;
            let version = if let Some(v) = self.convert_field(3) {
                v
            } else {
                println!("No version");
                break;
            };
            bit_counter += 3;

            let type_id = if let Some(t) = self.convert_field(3) {
                t
            } else {
                println!("No type_id");
                break;
            };
            bit_counter += 3;

            if type_id == 4 {
                // Value
                let mut bits = Vec::new();
                let mut is_last = false;
                while !is_last {
                    let indicator = if let Some(t) = self.convert_field(1) {
                        t
                    } else {
                        println!("No indicator");
                        break;
                    };
                    bit_counter += 1;

                    is_last = indicator == 0;
                    if let Some(_) = self.expand_buffer(4) {
                        bits.extend(&self.buffer[0..4]);
                        self.buffer.drain(0..4);
                    } else {
                        println!("No value data");
                        break;
                    }
                    bit_counter += 4;
                }
                let value = Factory::convert_binary_to_int(&bits);
                let message = Message::new(version, type_id, Data::Value(value), bit_counter);
                output.push(message);
            } else {
                // Operator
                let length_type_id = if let Some(l) = self.convert_field(1) {
                    l
                } else {
                    println!("No length_type_id");
                    break;
                };
                bit_counter += 1;

                let op = if length_type_id == 0 {
                    let total_length_bits = if let Some(t) = self.convert_field(15) {
                        t
                    } else {
                        println!("No ltotal_length");
                        break;
                    };
                    bit_counter += 15;
                    Operation::new(Some(total_length_bits), None, type_id)
                } else {
                    let total_number_subpackets = if let Some(t) = self.convert_field(11) {
                        t
                    } else {
                        println!("No ltotal_subpackets_num");
                        break;
                    };
                    bit_counter += 11;
                    Operation::new(None, Some(total_number_subpackets), type_id)
                };
                let message = Message::new(version, type_id, Data::Operation(op), bit_counter);
                output.push(message);
            }
        }
        // output
        Factory::fix_output(output)
    }

    fn fix_output(input: Vec<Message>) -> Vec<Message> {
        let mut output = Vec::new();
        let mut it = input.into_iter();
        while let Some(message) = it.next() {
            output.push(Factory::make_valid(message, &mut it));
        }
        output
    }

    fn make_valid<I>(mut input: Message, it: &mut I) -> Message
    where
        I: Iterator<Item = Message>,
    {
        if input.verify() {
            println!("Yay! is valid");
            input
        } else {
            if let Some(n) = it.next() {
                if n.verify() {
                    input.add_subpacket(Box::new(n));
                    return Factory::make_valid(input, it);
                } else {
                    let n = Factory::make_valid(n, it);
                    input.add_subpacket(Box::new(n));
                    return Factory::make_valid(input, it);
                }
            } else {
                println!("We in trouble");
                input
            }
        }
    }
}

fn part_one(input: &[String]) -> u128 {
    let mut factory = Factory::new(input[0].as_str());
    let messages = factory.parse_input();
    messages.iter().map(|x| x.get_versions()).flatten().sum()
}

fn part_two(input: &[String]) -> u128 {
    let mut factory = Factory::new(input[0].as_str());
    let messages = factory.parse_input();
    let outputs: Vec<u128> = messages.iter().map(|x| x.do_operation()).collect();
    outputs[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let input = "8A004A801A8002F478";
        let output = part_one(&str_to_string_vec(&input));
        let truth = 16;
        assert_eq!(output, truth);
        let input = "620080001611562C8802118E34";
        let output = part_one(&str_to_string_vec(&input));
        let truth = 12;
        assert_eq!(output, truth);
        let input = "C0015000016115A2E0802F182340";
        let output = part_one(&str_to_string_vec(&input));
        let truth = 23;
        assert_eq!(output, truth);
        let input = "A0016C880162017C3686B18A3D4780";
        let output = part_one(&str_to_string_vec(&input));
        let truth = 31;
        assert_eq!(output, truth);
    }

    #[test]
    fn test_two() {
        let input = "C200B40A82";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 3;
        assert_eq!(output, truth);
        let input = "04005AC33890";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 54;
        assert_eq!(output, truth);
        let input = "880086C3E88112";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 7;
        assert_eq!(output, truth);
        let input = "CE00C43D881120";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 9;
        assert_eq!(output, truth);
        let input = "D8005AC2A8F0";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 1;
        assert_eq!(output, truth);
        let input = "F600BC2D8F";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 0;
        assert_eq!(output, truth);
        let input = "9C005AC2F8F0";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 0;
        assert_eq!(output, truth);
        let input = "9C0141080250320F1802104A08";
        let output = part_two(&str_to_string_vec(&input));
        let truth = 1;
        assert_eq!(output, truth);
    }
}
