use std::io;

fn first(input: i32) -> i32 {
    let in_f = input as f64;
    let n = (in_f.sqrt() / 2.).floor() as i32;
    let offset = (input - (2 * n - 1).pow(2)) % (2 * n);
    return n + (offset - n).abs();
}

fn do_second(input: i32, dim: usize, array: &mut Vec<Vec<i32>>) -> i32 {
    fn value_of(array: &Vec<Vec<i32>>, x: usize, y: usize) -> i32 {
        return array[x - 1][y - 1] + array[x][y - 1] + array[x + 1][y - 1] + array[x - 1][y] + array[x + 1][y] + array[x - 1][y + 1] + array[x][y + 1] + array[x + 1][y + 1];
    }

    let mut x = dim / 2;
    let mut y = x;
    let mut k = 1;

    array[x][y] = 1;
    loop {
        for _ in 0..k {
            y += 1;
            array[x][y] = value_of(&array, x, y);
            if array[x][y] > input {
                return array[x][y];
            }
        }
        for _ in 0..k {
            x -= 1;
            array[x][y] = value_of(&array, x, y);
            if array[x][y] > input {
              return array[x][y];
            }
        }
        k += 1;
        for _ in 0..k {
            y -= 1;
            array[x][y] = value_of(&array, x, y);
            if array[x][y] > input {
                return array[x][y];
            }
        }
        for _ in 0..k {
            x += 1;
            array[x][y] = value_of(&array, x, y);
            if array[x][y] > input {
                return array[x][y];
            }
        }
        k += 1;
    }
}

fn second(input: i32) -> i32 {
    let dim: usize = 64;
    let mut array = Vec::with_capacity(dim);

    for _ in 0..dim {
        let mut line = Vec::with_capacity(dim);
        for _ in 0..dim {
            line.push(0);
        }
        array.push(line);
    }

    return do_second(input, dim, &mut array);
}

fn parse_stdin() -> i32 {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("input needed");

    let trimmed = s.trim();
    return trimmed.parse::<i32>().unwrap();
}

fn main() {
    let input: i32 = parse_stdin();

    println!("input is = {}", input);
    println!("first = {}", first(input));
    println!("second = {}", second(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() {
        assert_eq!(1, first(6));
        assert_eq!(2, first(15));
        assert_eq!(9, first(100));
    }

    #[test]
    fn test_second() {
        assert_eq!(10, second(6));
        assert_eq!(122, second(100));
        assert_eq!(13486, second(10000));
    }
}
