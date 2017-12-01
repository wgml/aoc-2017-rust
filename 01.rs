use std::io;

fn first(str: &String) -> u32 {
    let mut sum = 0;
    let len = str.len();
    for (i, c) in str.chars().enumerate() {
        let next = str.chars().nth((i + 1) % (len));

        if c == next.unwrap() {
            sum += c.to_digit(10).unwrap();
        }
    }
    return sum;
}

fn second(str: &String) -> u32 {
    let mut sum = 0;
    let halfway = str.len() / 2;
    let dupl = format!("{}{}", str, str);

    for (i, c) in str.chars().enumerate() {
        let next = dupl.chars().nth(i + halfway);

        if c == next.unwrap() {
            sum += c.to_digit(10).unwrap();
        }
    }
    return sum;
}
fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("no input");

    input.pop(); // \n
    println!("first = {}", first(&input));
    println!("second = {}", second(&input));
}

