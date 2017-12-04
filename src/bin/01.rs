use std::io;

fn first(input: &String) -> u32 {
    let mut sum = 0;
    let len = input.len();
    for (i, c) in input.chars().enumerate() {
        let next = input.chars().nth((i + 1) % (len));

        if c == next.unwrap() {
            sum += c.to_digit(10).unwrap();
        }
    }
    return sum;
}

fn second(input: &String) -> u32 {
    let mut sum = 0;
    let halfway = input.len() / 2;
    let dupl = format!("{}{}", input, input);

    for (i, c) in input.chars().enumerate() {
        let next = dupl.chars().nth(i + halfway);

        if c == next.unwrap() {
            sum += c.to_digit(10).unwrap();
        }
    }
    return sum;
}

pub fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("no input");

    input.pop(); // \n
    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}
