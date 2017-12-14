use std::io;
use std::io::prelude::*;

fn task(input: &String) -> (usize, usize) {
    const ESCAPE_CHAR: char = '!';
    const GARBAGE_BEGIN: char = '<';
    const GARBAGE_END: char = '>';
    const GROUP_BEGIN: char = '{';
    const GROUP_END: char = '}';

    let mut total_score: usize = 0;
    let mut current_score: usize = 0;
    let mut in_garbage = false;
    let mut is_escaped = false;
    let mut garbage_characters: usize = 0;

    for c in input.chars() {
        if is_escaped {
            is_escaped = false;
            continue;
        }

        if c == ESCAPE_CHAR {
            is_escaped = true;
            continue;
        }

        if in_garbage {
            if c == GARBAGE_END {
                in_garbage = false;
            } else {
                garbage_characters += 1;
            }
            continue;
        }

        if c == GARBAGE_BEGIN {
            in_garbage = true;
            continue;
        }

        if c == GROUP_BEGIN {
            current_score += 1;
            continue;
        }

        if c == GROUP_END {
            total_score += current_score;
            current_score -= 1;
            continue;
        }
    }

    return (total_score, garbage_characters);
}

fn parse_stdin() -> String {
    let stdin = io::stdin();

    let mut input = String::new();
    let _ = stdin.lock().read_line(&mut input);
    return input;

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
        assert_eq!(1, task(&"{}".to_string()).0);
        assert_eq!(6, task(&"{{{}}}".to_string()).0);
        assert_eq!(5, task(&"{{},{}}".to_string()).0);
        assert_eq!(16, task(&"{{{},{},{{}}}}".to_string()).0);
        assert_eq!(1, task(&"{<a>,<a>,<a>,<a>}".to_string()).0);
        assert_eq!(9, task(&"{{<ab>},{<ab>},{<ab>},{<ab>}}".to_string()).0);
        assert_eq!(9, task(&"{{<!!>},{<!!>},{<!!>},{<!!>}}".to_string()).0);
        assert_eq!(3, task(&"{{<a!>},{<a!>},{<a!>},{<ab>}}".to_string()).0);
    }

    #[test]
    fn test_second() {
        assert_eq!(0, task(&"<>".to_string()).1);
        assert_eq!(17, task(&"<random characters>".to_string()).1);
        assert_eq!(3, task(&"<<<<>".to_string()).1);
        assert_eq!(2, task(&"<{!>}>".to_string()).1);
        assert_eq!(0, task(&"<!!>".to_string()).1);
        assert_eq!(0, task(&"<!!!>>".to_string()).1);
        assert_eq!(10, task(&"<{o\"i!a,<{i<a>".to_string()).1);
    }
}
