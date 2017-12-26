use std::io;
use std::io::prelude::*;

type Matrix = Vec<Vec<char>>;

fn task(input: &Matrix) -> (String, usize) {
    fn at(matrix: &Matrix, x: usize, y: usize, dir: &(isize, isize)) -> char {
        if x == 0 && dir.0 == -1 {
            return ' ';
        }
        if y == 0 && dir.1 == -1 {
            return ' ';
        }
        let x = ((x as isize) + dir.0) as usize;
        let y = ((y as isize) + dir.1) as usize;

        if matrix.len() <= y || matrix[y].len() <= x {
            return ' ';
        }
        return matrix[y][x];
    }

    fn find_start(input: &Matrix) -> (usize, usize) {
        for i in 0..input[0].len() {
            if input[0][i] != ' ' {
                return (i, 0);
            }
        }
        panic!("No valid start point found!");
    }

    let (mut x, mut y) = find_start(input);
    let mut dir: (isize, isize) = (0, 1);
    let mut letters = String::new();
    let mut steps = 0;

    loop {
        let c = input[y][x];
        if c == '+' {
            let dirs = vec!((dir.1, dir.0), (-dir.1, -dir.0));
            dir = *dirs.iter()
                .filter(|d| at(input, x, y, d) != ' ')
                .nth(0)
                .expect("must be one valid direction");
        } else if c == ' ' {
            break;
        } else if c != '|' && c != '-' {
            letters.push(c);
        }
        let new_x = (x as isize) + dir.0;
        let new_y = (y as isize) + dir.1;

        if new_x < 0 || new_y < 0 || input.len() < y || input[y].len() < x {
            break;
        }

        x = new_x as usize;
        y = new_y as usize;
        steps += 1;
    }

    return (letters, steps);
}

fn parse_stdin() -> Matrix {
    let stdin = io::stdin();

    return stdin.lock().lines()
        .into_iter()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())

        .collect();
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
    fn test_task() {
        let input = "     |
     |  +--+
     A  |  C
 F---|----E|--+
     |  |  |  D
     +B-+  +--+ ".split('\n')
            .into_iter()
            .map(|s| s.chars().collect::<Vec<char>>())
            .collect();
        assert_eq!(("ABCDEF".to_string(), 38), task(&input));
    }
}
