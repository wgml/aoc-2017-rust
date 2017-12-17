use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

fn task(input: u32, iters: u32) -> VecDeque<u32> {
    fn rotate(queue: &mut VecDeque<u32>, shift: u32) {
        for _ in 0..shift {
            let e = queue.pop_front().unwrap();
            queue.push_back(e);
        }
    }

    let mut deque = VecDeque::with_capacity((iters + 1) as usize);
    deque.push_back(0);
    for i in 1..(iters + 1) {
        rotate(&mut deque, input);
        deque.push_back(i);
    }

    return deque;
}

fn first(input: u32) -> u32 {
    let deque = task(input, 2017);
    return deque[0];
}

fn second(input: u32) -> u32 {
    let deque = task(input, 50_000_000);

    let index_of_zero = deque.iter().position(|&e| e == 0).unwrap();
    if index_of_zero == deque.len() - 1 {
        return deque[0];
    } else {
        return deque[index_of_zero + 1];
    }
}

fn parse_stdin() -> u32 {
    let stdin = io::stdin();

    let mut line = String::new();
    let _ = stdin.lock().read_line(&mut line);
    return line.parse::<u32>().expect("must be a number");
}

fn main() {
    let input = parse_stdin();
    println!("first = {}", first(input));
    println!("second = {}", second(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        assert_eq!(638, task(3, 2017)[0]);
    }
}
