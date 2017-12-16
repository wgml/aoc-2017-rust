use std::io;
use std::io::prelude::*;

struct Generator {
    value: u64,
    mult: u64,
    div: u64,
    filter: u64
}

impl Generator {
    fn new(value: u64, mult: u64) -> Generator {
        return Generator { value, mult, div: 2147483647, filter: 1 };
    }

    fn with_filter(value: u64, mult: u64, filter: u64) -> Generator {
        return Generator { value, mult, div: 2147483647, filter };
    }
}

impl Iterator for Generator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        loop {
            self.value = (self.value * self.mult) % self.div;

            if self.value % self.filter == 0 {
                break;
            }
        }
        return Some(self.value);
    }
}

fn first((a, b): (u64, u64)) -> usize {
    let gen_a = Generator::new(a, 16807);
    let gen_b = Generator::new(b, 48271);
    let zip = gen_a.zip(gen_b);

    return zip.take(40_000_000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count();
}

fn second((a, b): (u64, u64)) -> usize {
    let gen_a = Generator::with_filter(a, 16807, 4);
    let gen_b = Generator::with_filter(b, 48271, 8);
    let zip = gen_a.zip(gen_b);

    return zip.take(5_000_000)
        .filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF)
        .count();
}

fn parse_stdin() -> (u64, u64) {
    fn parse_line(line: &String) -> u64 {
        return line.trim()
            .split(' ')
            .into_iter()
            .last()
            .map(|v| v.parse::<u64>().expect("must be a number"))
            .unwrap();
    }
    let stdin = io::stdin();

    let mut line = String::new();
    let _ = stdin.lock().read_line(&mut line);
    let a = parse_line(&line);
    let _ = stdin.lock().read_line(&mut line);
    let b = parse_line(&line);

    return (a, b);
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
    fn test_generators() {
        let gen_a = Generator::new(65, 16807);
        let gen_b = Generator::new(8921, 48271);
        let mut gens = gen_a.zip(gen_b);

        let expected_a = [1092455, 1181022009, 245556042, 1744312007, 1352636452];
        let expected_b = [430625591, 1233683848, 1431495498, 137874439, 285222916];

        for i in 0..5 {
            assert_eq!(Some((expected_a[i], expected_b[i])), gens.next());
        }
    }

    #[test]
    fn test_first() {
        assert_eq!(588, first((65, 8921)));
    }

    #[test]
    fn test_second() {
        assert_eq!(309, second((65, 8921)));
    }
}
