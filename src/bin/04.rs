use std::io;
use std::collections::HashSet;
use std::io::prelude::*;
use std::iter::FromIterator;

fn first(input: &Vec<Vec<String>>) -> usize {
    return input.iter()
        .filter(|line| {
            let mut unique = HashSet::new();
            line.into_iter().all(|token| unique.insert(token));
            return line.len() == unique.len();
        })
        .count();
}

fn second(input: &Vec<Vec<String>>) -> usize {
    let sort_str = |str: &String| {
        let mut sorted: Vec<char> = str.chars().collect();
        sorted.sort_by(|a, b| b.cmp(a));
        return String::from_iter(sorted);
    };

    let reordered = input.iter()
        .map(|line| line.iter().map(|token| sort_str(token)).collect())
        .collect();
    return first(&reordered);
}

fn parse_stdin() -> Vec<Vec<String>> {
    let stdin = io::stdin();

    return stdin.lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.split_whitespace().map(|e| e.to_string()).collect())
        .collect();
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
        let input = vec!(
            "aa bb cc dd ee",
            "aa bb cc dd aa",
            "aa bb cc dd aaa",
        ).iter()
            .map(|e| e.to_string().split_whitespace().map(|e| e.to_string()).collect())
            .collect();
        assert_eq!(2, first(&input));
    }

    #[test]
    fn test_second() {
        let input = vec!(
            "abcde fghij",
            "abcde xyz ecdab",
            "a ab abc abd abf abj",
            "iiii oiii ooii oooi oooo",
            "oiii ioii iioi iiio"
        ).iter()
            .map(|e| e.to_string().split_whitespace().map(|e| e.to_string()).collect())
            .collect();
        assert_eq!(3, second(&input));
    }
}
