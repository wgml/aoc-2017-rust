use std::collections::HashMap;
use std::io;
use std::io::Read;

fn max_of(input: &Vec<usize>) -> (usize, usize) {
    let mut i_max = 0;
    let mut v_max = input[0];

    for (index, value) in input.iter().cloned().enumerate() {
        if value > v_max {
            i_max = index;
            v_max = value;
        }
    }
    return (i_max, v_max);
}

fn iter(mut result: Vec<usize>) -> Vec<usize> {
    let size = result.len();

    let (index, value) = max_of(&result);

    let increment = value / size;
    let mut rest = value - increment;

    result[index] = 0;
    if increment > 0 {
        result = result.iter().map(|e| e + increment).collect();
    }

    let mut i = index;
    while rest > 0 {
        i = (i + 1) % size;
        result[i] += 1;
        rest -= 1;
    }
    return result;
}

fn task(input: &Vec<usize>) -> (usize, usize) {
    let mut known = HashMap::new();
    let mut current = input.clone();

    let mut iteration = 0;
    loop {
        if known.get(&current).is_some() {
            break;
        }
        known.insert(current.clone(), iteration);
        current = iter(current);
        iteration += 1;
    }

    let loop_size = known.len() - known[&current];
    return (known.len(), loop_size);
}

fn parse_stdin() -> Vec<usize> {
    let mut line = String::new();
    let _ = io::stdin().read_to_string(&mut line);

    return line.split_whitespace()
        .map(|e| e.parse::<usize>().expect("required a usize num"))
        .collect();
}

fn main() {
    let input = parse_stdin();
    let (first, second) = task(&input);
    println!("first = {}", first);
    println!("second = {}", second);
}
