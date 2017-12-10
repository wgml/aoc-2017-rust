use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

fn hash_round(lengths: &Vec<usize>, indices: &mut Vec<usize>, mut current_position: usize, mut skip_size: usize) -> (usize, usize) {
    for length in lengths {
        let mut to_process: Vec<usize> = Vec::with_capacity(*length);
        for i in 0..*length {
            to_process.push(indices[(current_position + i) % 256]);
        }

        to_process.reverse();
        for i in 0..*length {
            indices[(current_position + i) % 256] = to_process[i];
        }

        current_position = (current_position + skip_size + length) % 256;
        skip_size += 1;
    }

    return (current_position, skip_size);
}

fn hash<'a>(lengths: &Vec<usize>, mut indices: &'a mut Vec<usize>, rounds: usize) -> &'a Vec<usize> {
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..rounds {
        let result = hash_round(&lengths, &mut indices, current_position, skip_size);
        current_position = result.0;
        skip_size = result.1;
    }

    return indices;
}

fn dense_hash(input: &Vec<usize>) -> String {
    fn xor(input: &[usize]) -> u8 {
        return input.iter().fold(0, |x, v| x ^ v) as u8;
    }

    let mut result = String::new();
    for chunk in input.chunks(16) {
        result += format!("{:x}", xor(&chunk)).as_ref();
    }
    return result;
}

fn first(input: &Vec<usize>) -> usize {
    let mut indices: Vec<usize> = Vec::from_iter(0..256);

    let result = hash(&input, &mut indices, 1);
    return result[0] * result[1];
}

fn second(input: &Vec<usize>) -> String {
    let mut indices: Vec<usize> = Vec::from_iter(0..256);

    let result = hash(&input, &mut indices, 64);
    return dense_hash(&result);
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
