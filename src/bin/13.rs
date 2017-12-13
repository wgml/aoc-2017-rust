use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct Numbers {
    value: usize,
}

impl Iterator for Numbers {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        let value = self.value;
        self.value += 1;
        return Some(value);
    }
}

fn pos(range: usize, tick: usize) -> usize {
    let adj = tick % ((range - 1) * 2);
    return if adj <= range - 1 { adj } else { 2 * (range - 1) - adj };
}

fn first(scanners: &HashMap<usize, usize>) -> usize {
    return scanners.keys().into_iter()
        .filter(|s| pos(scanners[s], **s) == 0)
        .map(|s| s * scanners[s])
        .sum();
}

fn second(scanners: &HashMap<usize, usize>) -> usize {
    let all_non_zero = |delay: &usize| {
        return scanners.keys().into_iter()
            .all(|s| pos(scanners[s], *s + delay) != 0);
    };

    return Numbers { value: 0 }.into_iter()
        .filter(|delay| all_non_zero(delay))
        .nth(0)
        .expect("must be anything, right?");
}

fn parse_stdin() -> HashMap<usize, usize> {
    fn to_us(s: &str) -> usize {
        return s.parse::<usize>().expect("must be a number");
    }

    let stdin = io::stdin();

    return stdin.lock().lines()
        .map(|line| {
            let parts: Vec<_> = line.unwrap()
                .split(": ")
                .map(|e| to_us(e))
                .collect();
            return (parts[0], parts[1]);
        })
        .collect();
}

fn main() {
    let input = parse_stdin();

    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}
