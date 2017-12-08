use std::cmp::max;
use std::collections::HashMap;
use std::io;
use std::io::prelude::*;

fn condition(reg_val: i32, cond: &String, value: i32) -> bool {
    return match cond.as_ref() {
        "==" => reg_val == value,
        "!=" => reg_val != value,
        ">=" => reg_val >= value,
        "<=" => reg_val <= value,
        ">" => reg_val > value,
        "<" => reg_val < value,
        _ => panic!("unexpected token ({})", cond)
    };
}

fn task(input: &Vec<(String, String, i32, String, String, i32)>) -> (i32, i32) {
    let mut registers: HashMap<&String, i32> = HashMap::new();
    let mut largest_met = 0;

    for instruction in input {
        let &(ref reg1, ref instr, ref val1,
            ref reg2, ref cond, ref val2) = instruction;

        if condition(*registers.entry(reg2).or_insert(0), cond, *val2) {
            let reg_value = registers.entry(reg1).or_insert(0);
            match instr.as_ref() {
                "inc" => *reg_value += val1,
                "dec" => *reg_value -= val1,
                _ => panic!("unexpected token ({})", instr)
            }

            largest_met = max(largest_met, *reg_value);
        }
    }

    let largest = *registers.values().max().unwrap();
    return (largest, largest_met);
}

fn parse_stdin() -> Vec<(String, String, i32, String, String, i32)> {
    let mut instructions = Vec::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let res: Vec<&str> = l.split_whitespace().collect();
        instructions.push((
            res[0].to_string(),
            res[1].to_string(),
            res[2].parse::<i32>().expect("must be parsable as number"),
            res[4].to_string(),
            res[5].to_string(),
            res[6].parse::<i32>().expect("must be parsable as number")
        ));
    }
    return instructions;
}

fn main() {
    let input = parse_stdin();
    let (first, second) = task(&input);
    println!("first = {}", first);
    println!("second = {}", second);
}
