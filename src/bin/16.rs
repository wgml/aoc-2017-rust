use std::io;
use std::io::prelude::*;
use std::iter::Iterator;
use std::collections::HashMap;

enum Action {
    Spin { n: usize },
    Exchange { a: usize, b: usize },
    Partner { a: char, b: char }
}

impl Action {
    fn act(&self, line: &mut Vec<char>) {
        match self {
            &Action::Spin { n } => {
                for _ in 0..n {
                    let el = line.pop().unwrap();
                    line.insert(0, el);
                }
            }
            &Action::Exchange { a, b } => {
                let a_v = line[a];
                line[a] = line[b];
                line[b] = a_v;
            }
            &Action::Partner { a, b } => {
                for i in 0..line.len() {
                    let e = line[i];
                    if e == a {
                        line[i] = b;
                    } else if e == b {
                        line[i] = a;
                    }
                }
            }
        }
    }

    fn from_str(act: &str) -> Action {
        if act.starts_with("s") {
            let n = act[1..].parse::<usize>().expect("must be a number");
            return Action::Spin { n };
        } else if act.starts_with("x") {
            let ab: Vec<usize> = act[1..].split('/')
                .into_iter()
                .take(2)
                .map(|e| e.parse::<usize>().expect("must be a number"))
                .collect();
            return Action::Exchange { a: ab[0], b: ab[1] };
        } else if act.starts_with("p") {
            let chars: Vec<char> = act.chars().collect();
            return Action::Partner { a: chars[1], b: chars[3] };
        } else {
            panic!("bad token: {}", act);
        }
    }
}

fn task(line: String, input: &Vec<Action>) -> String {
    let mut line: Vec<char> = line.chars().collect();

    for action in input {
        action.act(&mut line);
    }

    return line.iter().fold(String::new(), |mut c, a| {
        c.push(*a);
        return c;
    });
}

fn first(input: &Vec<Action>) -> String {
    return task("abcdefghijklmnop".to_string(), input);
}

fn second(input: &Vec<Action>) -> String {
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut line = "abcdefghijklmnop".to_string();
    let mut iter: usize = 1;
    let repetitions = 1_000_000_000;
    
    while iter < repetitions {
        line = task(line, input);

        match seen.insert(line.clone(), iter) {
            None => iter += 1,
            Some(prev_iter) => {
                let loop_size = iter - prev_iter;
                let to_add = (repetitions - iter) / loop_size;
                if to_add > 0 {
                    iter += to_add * loop_size;
                } else {
                    iter += 1;
                }
            }
        }
    }

    return line;
}

fn parse_stdin() -> Vec<Action> {
    let stdin = io::stdin();

    let mut line = String::new();
    let _ = stdin.lock().read_line(&mut line);

    return line.trim()
        .split(',')
        .into_iter()
        .map(|e| Action::from_str(e))
        .collect();
}

fn main() {
    let input = parse_stdin();
    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}
