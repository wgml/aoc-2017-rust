use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

type Map = HashMap<i64, HashMap<i64, State>>;
const DIRECTIONS: [(i64, i64); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn get<'a>(map: &'a mut Map, x: &i64, y: &i64) -> &'a State {
    if !map.contains_key(y) {
        map.insert(*y, HashMap::new());
    }
    if !map[y].contains_key(&x) {
        map.get_mut(y).expect("must be present").insert(*x, State::Clean);
    }
    return map[&y].get(&x).expect("must be present");
}

fn set(map: &mut Map, x: &i64, y: &i64, v: State) {
    map.get_mut(y).expect("must be present").insert(*x, v);
}

fn first(input: &Map) -> usize {
    let mut map = input.clone();

    let mut y = (map.len() / 2) as i64;
    let mut x = (map[&y].len() / 2) as i64;
    let mut dir = 0;

    let mut infections = 0;
    for _ in 0..10000 {
        let new_state = match *get(&mut map, &x, &y) {
            State::Clean => {
                dir = (dir - 1) % 4;
                infections += 1;
                State::Infected
            }
            State::Infected => {
                dir = (dir + 1) % 4;
                State::Clean
            }
            _ => panic!("unreachable")
        };

        set(&mut map, &x, &y, new_state);
        x += DIRECTIONS[dir].0;
        y += DIRECTIONS[dir].1;
    }
    return infections;
}

fn second(input: &Map) -> usize {
    let mut map = input.clone();

    let mut y = (map.len() / 2) as i64;
    let mut x = (map[&y].len() / 2) as i64;
    let mut dir = 0;

    let mut infections = 0;
    for _ in 0..10000000 {
        let new_state = match *get(&mut map, &x, &y) {
            State::Clean => {
                dir = (dir - 1) % 4;
                State::Weakened
            }
            State::Weakened => {
                infections += 1;
                State::Infected
            }
            State::Infected => {
                dir = (dir + 1) % 4;
                State::Flagged
            }
            State::Flagged => {
                dir = (dir + 2) % 4;
                State::Clean
            }
        };

        set(&mut map, &x, &y, new_state);
        x += DIRECTIONS[dir].0;
        y += DIRECTIONS[dir].1;
    }
    return infections;
}

fn parse_stdin() -> Map {
    fn parse_line(line: &String) -> HashMap<i64, State> {
        return line.chars()
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i as i64, if c == '#' { State::Infected } else { State::Clean }))
            .collect();
    }

    let stdin = io::stdin();

    return stdin.lock().lines()
        .into_iter()
        .enumerate()
        .map(|(i, l)| (i as i64, parse_line(&l.unwrap())))
        .collect();
}

fn main() {
    let input = parse_stdin();
    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}
