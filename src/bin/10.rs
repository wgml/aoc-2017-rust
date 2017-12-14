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
