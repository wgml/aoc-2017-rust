use std::io;
use std::io::prelude::*;
use std::cmp::max;

struct Direction {
    x: i32,
    y: i32,
}
static N: Direction = Direction{x: 0, y:1};
static NW: Direction = Direction{x: -1, y: 1};
static NE: Direction = Direction{x: 1, y: 0};
static S: Direction = Direction{x: 0, y: -1};
static SW: Direction = Direction{x: -1, y: 0};
static SE: Direction = Direction{x: 1, y: -1};

fn from_str(s: &str) -> &'static Direction {
    match s {
        "n" => &N,
        "nw" => &NW,
        "ne" => &NE,
        "s" => &S,
        "sw" => &SW,
        "se" => &SE,
        _ => panic!("unparsable token {}", s),
    }
}

fn task(input: &Vec<&Direction>) -> (usize, usize) {
    fn distance(x: i32, y: i32) -> usize {
        let dist = (x.abs() + y.abs() + (x + y).abs()) / 2;
        return dist as usize;
    }

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut max_dist: usize = 0;

    for m in input {
        x += m.x;
        y += m.y;
        max_dist = max(max_dist, distance(x, y));
    }
    return (distance(x, y), max_dist);
}

fn parse_stdin() -> Vec<&'static Direction> {
    let stdin = io::stdin();

    let mut input = String::new();
    let _ = stdin.lock().read_line(&mut input);

    return input.split(',').map(|e| from_str(e)).collect();
}

fn main() {
    let input = parse_stdin();

    let (first, second) = task(&input);
    println!("first = {}", first);
    println!("second = {}", second);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(3, task(&vec!(&NE, &NE, &NE)).0);
        assert_eq!(0, task(&vec!(&NE, &NE, &SW, &SW)).0);
        assert_eq!(2, task(&vec!(&NE, &NE, &S, &S)).0);
        assert_eq!(3, task(&vec!(&SE, &SW, &SE, &SW, &SW)).0);
    }
}
