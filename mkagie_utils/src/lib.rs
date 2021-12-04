use std::{
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

pub fn file_to_string_vec(filename: &str) -> Vec<String> {
    BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|x| x.unwrap())
        .collect()
}

pub fn file_to_int_vec<T>(filename: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    file_to_string_vec(filename)
        .iter()
        .map(|x| x.parse::<T>().unwrap())
        .collect()
}

pub fn split_by_whitespace(input: &str) -> Vec<&str> {
    input.split(' ').collect()
}

pub fn str_array_to_vec(input: &[&str]) -> Vec<String> {
    input.iter().map(|x| x.to_string()).collect()
}

pub fn str_array_to_int_vec(input: &[&str]) -> Vec<i32> {
    input.iter().map(|x| x.parse::<i32>().unwrap()).collect()
}

pub fn str_to_string_vec(input: &str) -> Vec<String> {
    input.lines().into_iter().map(|x| x.to_string()).collect()
}
