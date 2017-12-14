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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(3, first(&"1122".to_string()));
        assert_eq!(4, first(&"1111".to_string()));
        assert_eq!(0, first(&"1234".to_string()));
        assert_eq!(9, first(&"91212129".to_string()));
    }

    #[test]
    fn test_second() {
        assert_eq!(6, second(&"1212".to_string()));
        assert_eq!(0, second(&"1221".to_string()));
        assert_eq!(4, second(&"123425".to_string()));
        assert_eq!(12, second(&"123123".to_string()));
        assert_eq!(4, second(&"12131415".to_string()));
    }
}
