use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;

type Component = (u64, u64);

fn weight(bridge: &Vec<Component>) -> u64 {
    return bridge.into_iter().map(|c| c.0 + c.1).sum();
}

fn build_bridges(bridge: &Vec<Component>, port: u64, components: &Vec<Component>) -> Vec<Vec<Component>> {
    let candidates: Vec<Component> = components.into_iter()
        .filter(|c| c.0 == port || c.1 == port)
        .map(|c| *c)
        .collect();

    if candidates.len() == 0 {
        return vec!(bridge.clone());
    }

    let mut result = Vec::new();
    for candidate in candidates {
        let remaining: Vec<Component> = components.into_iter()
            .filter(|c| **c != candidate)
            .map(|c| c.clone())
            .collect();

        let port = if candidate.0 == port { candidate.1 } else { candidate.0 };

        let mut new_bridge = bridge.clone();
        new_bridge.push(candidate);
        result.extend(build_bridges(&new_bridge, port, &remaining));
    }
    return result;
}

fn first(input: &Vec<Component>) -> u64 {
    let bridges = build_bridges(&vec![(0, 0)], 0, input);

    return bridges.into_iter()
        .map(|b| weight(&b))
        .max()
        .unwrap();
}

fn second(input: &Vec<Component>) -> u64 {
    let bridges = build_bridges(&vec![(0, 0)], 0, input);

    return bridges.into_iter()
        .map(|b| (b.len(), weight(&b)))
        .max_by(|a, b| {
            let mut ord = a.0.cmp(&b.0);
            if ord == Ordering::Equal {
                ord = a.1.cmp(&b.1);
            }
            return ord;
        })
        .map(|c| c.1)
        .unwrap();
}

fn parse_stdin() -> Vec<Component> {
    fn to_component(s: &String) -> Component {
        let parts: Vec<u64> = s.split('/')
            .into_iter()
            .map(|e| e.parse::<u64>().expect("must be a number"))
            .collect();
        return (parts[0], parts[1]);
    }

    let stdin = io::stdin();

    return stdin.lock().lines()
        .into_iter()
        .map(|l| to_component(&l.unwrap()))
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

    fn input() -> Vec<Component> {
        return vec!(
            (0, 2),
            (2, 2),
            (2, 3),
            (3, 4),
            (3, 5),
            (0, 1),
            (10, 1),
            (9, 10)
        );
    }

    #[test]
    fn test_first() {
        assert_eq!(31, first(&input()));
    }

    #[test]
    fn test_second() {
        assert_eq!(19, second(&input()));
    }
}
