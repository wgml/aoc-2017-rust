use std::io;
use std::io::prelude::*;
use std::u8;
use std::collections::HashSet;

extern crate advent_of_code_2017;

use advent_of_code_2017::knot_hash;

mod bits {
    pub fn is_bit_set(byte: &u8, bit: &u8) -> bool {
        return (byte & (1u8 << bit)) >> bit == 1;
    }

    pub fn in_u8(input: &u8) -> usize {
        return (0..8).into_iter()
            .filter(|i| is_bit_set(input, i))
            .count();
    }

    pub fn in_row(input: &Vec<u8>) -> usize {
        return input.iter()
            .map(|u| in_u8(u))
            .sum();
    }
}

fn matrix(input: &String) -> Vec<Vec<u8>> {
    fn u(input: &String) -> Vec<u8> {
        let mut result = Vec::with_capacity(8);
        for i in 0..16 {
            let chars: &str = &input[(2 * i)..(2 * i + 2)];
            result.push(u8::from_str_radix(chars, 16).unwrap());
        }
        return result;
    }

    let mut result = Vec::with_capacity(128);
    for i in 0..128 {
        let hash = knot_hash::for_str(&format!("{}-{}", input, i));
        result.push(u(&hash));
    }

    return result;
}

fn first(input: &String) -> usize {
    return matrix(input)
        .iter()
        .map(|row| bits::in_row(row))
        .sum();
}

fn second(input: &String) -> usize {
    fn is_set(matrix: &Vec<Vec<u8>>, (row, col): (usize, usize)) -> bool {
        let el = matrix[row][col / 8];
        let bit = 7 - col as u8 % 8;
        return bits::is_bit_set(&el, &bit);
    }

    fn neighbours((x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::with_capacity(4);
        if x > 0 {
            neighbours.push((x - 1, y));
        }
        if y > 0 {
            neighbours.push((x, y - 1));
        }
        if x < 127 {
            neighbours.push((x + 1, y));
        }
        if y < 127 {
            neighbours.push((x, y + 1));
        }
        return neighbours;
    }

    fn fill_group(coords: (usize, usize), matrix: &Vec<Vec<u8>>, seen: &mut HashSet<(usize, usize)>) {
        let mut candidates = neighbours(coords);
        while !candidates.is_empty() {
            let candidate = candidates.pop().unwrap();
            if is_set(matrix, candidate) && seen.insert(candidate) {
                candidates.extend(neighbours(candidate));
            }
        }
    }

    let mat = matrix(input);
    let mut count = 0;
    let mut seen = HashSet::new();

    for row in 0..128 {
        for col in 0..128 {
            let coords = (row, col);
            if seen.insert(coords) && is_set(&mat, coords) {
                count += 1;
                fill_group(coords, &mat, &mut seen);
            }
        }
    }
    return count;
}

fn parse_stdin() -> String {
    let stdin = io::stdin();

    let mut input = String::new();
    let _ = stdin.lock().read_line(&mut input);

    return input;
}

fn main() {
    let input = parse_stdin();

    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        let input = "flqrgnkx".to_string();
        assert_eq!(8108, first(&input));
    }

    #[test]
    fn test_second() {
        let input = "flqrgnkx".to_string();
        assert_eq!(1242, second(&input));
    }
}
