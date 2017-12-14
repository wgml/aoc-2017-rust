use std::io;
use std::io::BufRead;

fn u(index: i32) -> usize {
    assert_eq!(index, (index as usize) as i32, "index out of bounds: {}", index);
    return index as usize;
}

fn compute<Mut>(input: &Vec<i32>, mut mutate: Mut) -> usize where
    Mut: FnMut(&mut Vec<i32>, i32) {
    let mut instructions = input.clone();
    let mut steps: usize = 0;
    let mut index: i32 = 0;

    while index >= 0 && u(index) < instructions.len() {
        let new_index = index + instructions[u(index)];
        mutate(&mut instructions, index);
        index = new_index;
        steps += 1;
    }

    return steps;
}

fn first(input: &Vec<i32>) -> usize {
    return compute(input, |instrs, idx| instrs[u(idx)] += 1);
}

fn second(input: &Vec<i32>) -> usize {
    return compute(input, |instrs, idx| instrs[u(idx)] += if instrs[u(idx)] >= 3 { -1 } else { 1 })
}

fn parse_stdin() -> Vec<i32> {
    let stdin = io::stdin();

    return stdin.lock().lines()
        .map(|line| line.unwrap())
        .map(|line| line.parse::<i32>().expect("required a number"))
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
        let input = vec!(0, 3, 0, 1, -3);
        assert_eq!(5, first(&input));
    }

    #[test]
    fn test_second() {
        let input = vec!(0, 3, 0, 1, -3);
        assert_eq!(10, second(&input));
    }
}
