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
