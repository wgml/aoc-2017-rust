use std::io;
use std::io::prelude::*;
use std::cmp;

fn first(input: &Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;

    for line in input {
        if line.len() > 0 {
            checksum += line.iter().max().unwrap() - line.iter().min().unwrap();
        }
    }
    return checksum;
}

fn second(input: &Vec<Vec<i32>>) -> i32 {
    let mut checksum = 0;

    for line in input {
        if line.len() ==  0 {
            continue;
        }
        let mut found = false;
        for e1 in line {
            for e2 in line {
                if e1 == e2 {
                    continue;
                }

                let min = cmp::min(e1, e2);
                let max = cmp::max(e1, e2);
                if max % min == 0 {
                    checksum += max / min;
                    found = true;
                }

                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }
        assert!(found, "did not find anything for line");
    }
    return checksum;
}

fn parse_stdin() -> Vec<Vec<i32>> {
    let stdin = io::stdin();
    let mut input = Vec::new();

    for line in stdin.lock().lines() {
        let raw = line.unwrap();
        let split = raw.split_whitespace();

        let mut line_vec = Vec::new();
        for e in split {
            line_vec.push(e.parse::<i32>().unwrap());
        }
        input.push(line_vec);
    }
    return input;
}

fn main() {
    let input = parse_stdin();

    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}

