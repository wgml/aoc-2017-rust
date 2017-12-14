use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

extern crate advent_of_code_2017;
use advent_of_code_2017::knot_hash;

fn first(input: &Vec<usize>) -> usize {
    let mut indices: Vec<usize> = Vec::from_iter(0..256);

    let result = knot_hash::hash(&input, &mut indices, 1);
    return result[0] * result[1];
}

fn second(input: &Vec<usize>) -> String {
    return knot_hash::dense_hash(&input);
}

fn parse_stdin() -> String {
    let stdin = io::stdin();

    let mut input = String::new();
    let _ = stdin.lock().read_line(&mut input);

    return input;
}

fn parse_stdin1(input: &String) -> Vec<usize> {
    return input.split(',').map(|c| c.parse::<usize>().expect("must be a number")).collect();
}

fn parse_stdin2(input: &String) -> Vec<usize> {
    let mut result = input.chars().map(|c| c as usize).collect::<Vec<usize>>();
    result.extend(vec!(17, 31, 73, 47, 23));
    return result;
}

fn main() {
    let input = parse_stdin();
    let input1 = parse_stdin1(&input);
    println!("first = {}", first(&input1));

    let input2 = parse_stdin2(&input);

    println!("second = {}", second(&input2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_second() {
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", second(&parse_stdin2(&"".to_string())));
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", second(&parse_stdin2(&"AoC 2017".to_string())));
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", second(&parse_stdin2(&"1,2,3".to_string())));
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", second(&parse_stdin2(&"1,2,4".to_string())));
    }
}
