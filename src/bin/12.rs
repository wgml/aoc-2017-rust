use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::Extend;

fn size(id: usize, ids: &mut HashMap<usize, Vec<usize>>) -> usize {
    let mut to_scan: Vec<usize> = vec!(id);
    let mut visited: HashSet<usize> = HashSet::new();
    let mut sum: usize = 0;

    while to_scan.len() > 0 {
        let id = to_scan.pop().expect("must be not empty");
        if !visited.insert(id) {
            continue;
        }
        sum += 1;
        let next = ids.remove(&id).expect("must be in map");
        to_scan.extend(next);
    }
    return sum;
}

fn task(mut ids: &mut HashMap<usize, Vec<usize>>) -> (usize, usize) {
    let group_size = size(0, &mut ids);
    let mut groups = 1;
    while !ids.is_empty() {
        let _ = size(*ids.keys().into_iter().nth(0).expect("must be not empty"), &mut ids);
        groups += 1;
    }
    return (group_size, groups);
}

fn parse_line(line: &String) -> (usize, Vec<usize>) {
    fn to_us(s: &str) -> usize {
        return s.parse::<usize>().expect("must be a number");
    }

    let parts: Vec<&str> = line.split(" <-> ").collect();
    let id = to_us(parts[0]);
    let ids: Vec<usize> = parts[1].split(", ").map(|i| to_us(i)).collect();
    return (id, ids);
}

fn parse_stdin() -> HashMap<usize, Vec<usize>> {
    let mut result: HashMap<usize, Vec<usize>> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let (id, ids) = parse_line(&l);
        result.insert(id, ids);
    }

    return result;
}

fn main() {
    let mut input = parse_stdin();

    let (first, second) = task(&mut input);
    println!("first = {}", first);
    println!("second = {}", second);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> HashMap<usize, Vec<usize>> {
        let lines = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

        let mut result: HashMap<usize, Vec<usize>> = HashMap::new();

        for line in lines.split('\n') {
            let (id, ids) = parse_line(&line.to_string());
            result.insert(id, ids);
        }

        return result;
    }

    #[test]
    fn test() {
        assert_eq!((6, 2), task(&mut input()));
    }
}
