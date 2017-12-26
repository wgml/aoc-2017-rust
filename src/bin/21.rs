use std::io;
use std::io::prelude::*;

type Image = Vec<Vec<bool>>;
type Rule = (Image, Image);

fn rotate_rule(rule: &Image) -> Image {
    let len = rule.len();
    let mut result = Image::with_capacity(len);
    for y in 0..len {
        result.push(Vec::with_capacity(len));
        for _ in 0..len {
            result[y].push(false);
        }
    }

    for row in 0..len {
        for col in 0..len {
            result[row][len - col - 1] = rule[col][row];
        }
    }

    return result;
}

fn flip_rule_horizontally(rule: &Image) -> Image {
    return rule.into_iter().rev().map(|row| row.into_iter().map(|c| *c).collect()).collect();
}

fn flip_rule_vertically(rule: &Image) -> Image {
    return rule.into_iter().map(|row| row.into_iter().rev().map(|c| *c).collect()).collect();
}

fn rule_matches(rule: &Image, image: &Image, x: usize, y: usize) -> bool {
    fn matches(rule: &Image, image: &Image, x: usize, y: usize) -> bool {
        let len = rule.len();

        for row in 0..len {
            for col in 0..len {
                if rule[row][col] != image[y + row][x + col] {
                    return false;
                }
            }
        }
        return true;
    }

    let mut rule = rule.clone();
    for _ in 0..4 {
        rule = rotate_rule(&rule);

        if matches(&rule, image, x, y) {
            return true;
        }

        if matches(&flip_rule_vertically(&rule), image, x, y) {
            return true;
        }

        if matches(&flip_rule_horizontally(&rule), image, x, y) {
            return true;
        }
    }

    return false;
}

fn apply_rules(image: &Image, rules: &Vec<Rule>) -> Image {
    let size = image.len();
    let chunk_size = if size % 2 == 0 { 2 } else { 3 };
    let chunks = size / chunk_size;

    let mut result = Vec::with_capacity(chunks);
    for _ in 0..chunks {
        result.push(Vec::with_capacity(chunks));
    }

    for y in 0..chunks {
        let row = y * chunk_size;
        for x in 0..chunks {
            let col = x * chunk_size;
            let mut any_rule_matches = false;

            for rule in rules {
                if rule.0.len() != chunk_size {
                    continue;
                }
                if rule_matches(&rule.0, image, col, row) {
                    result[y].push(&rule.1);
                    any_rule_matches = true;
                    break;
                }
            }
            assert!(any_rule_matches);
        }
    }

    let mut new_image = Vec::new();
    let new_chunk_size = result[0][0].len();
    let new_image_size = new_chunk_size * chunks;

    let mut curr_y = 0;
    for row_rules in result {
        for _ in 0..new_chunk_size {
            new_image.push(Vec::with_capacity(new_image_size));
        }

        for rule in row_rules {
            for y in 0..new_chunk_size {
                for x in 0..new_chunk_size {
                    let new_val = rule[y][x];
                    new_image[curr_y + y].push(new_val);
                }
            }
        }
        curr_y += new_chunk_size;
    }
    return new_image;
}

fn sim(rules: &Vec<Rule>, iterations: usize) -> usize {
    let mut image = vec!(vec!(false, true, false), vec!(false, false, true), vec!(true, true, true));

    for _ in 0..iterations {
        image = apply_rules(&image, &rules);
    }
    return image.into_iter()
        .map(|row| row.into_iter()
            .filter(|pixel| *pixel)
            .count())
        .sum();
}

fn first(input: &Vec<Rule>) -> usize {
    return sim(input, 5);
}

fn second(input: &Vec<Rule>) -> usize {
    return sim(input, 18);
}

fn parse_rule(input: &String) -> Rule {
    fn parse_matrix(matrix: &String) -> Image {
        return matrix.split('/')
            .into_iter()
            .map(|line|
                line.chars()
                    .map(|c| c == '#')
                    .collect())
            .collect();
    }

    let parts: Vec<String> = input.split(" => ").map(|e| e.to_string()).collect();
    let input = parse_matrix(&parts[0]);
    let output = parse_matrix(&parts[1]);
    return (input, output);
}

fn parse_stdin() -> Vec<Rule> {
    let stdin = io::stdin();

    return stdin.lock().lines()
        .into_iter()
        .map(|l| parse_rule(&l.unwrap()))
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
    fn test_flip() {
        let input = vec!(
            vec!(true, false, false),
            vec!(true, false, true),
            vec!(false, true, true)
        );
        let expected_h = vec!(
            vec!(false, true, true),
            vec!(true, false, true),
            vec!(true, false, false)
        );
        let expected_v = vec!(
            vec!(false, false, true),
            vec!(true, false, true),
            vec!(true, true, false)
        );
        assert_eq!(expected_h, flip_rule_horizontally(&input));
        assert_eq!(expected_v, flip_rule_vertically(&input));
    }

    #[test]
    fn test_rotate() {
        let input = vec!(
            vec!(true, false, false),
            vec!(true, false, true),
            vec!(false, true, true)
        );
        let expected = vec!(
            vec!(false, true, true),
            vec!(true, false, false),
            vec!(true, true, false)
        );
        assert_eq!(expected, rotate_rule(&input));
    }

    #[test]
    fn test_first() {
        let input = vec!(
            parse_rule(&"../.# => ##./#../...".to_string()),
            parse_rule(&".#./..#/### => #..#/..../..../#..#".to_string()),
        );
        assert_eq!(12, sim(&input, 2));
    }
}
