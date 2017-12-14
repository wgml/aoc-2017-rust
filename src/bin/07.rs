use std::fmt;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

struct Disc {
    weight: usize,
    parent: String,
    children: Vec<String>
}

impl fmt::Debug for Disc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{weight={}, parent={}, children={:?}}}", self.weight, self.parent, self.children)
    }
}

fn first<'a>(input: &'a HashMap<String, Disc>) -> &'a String {
    let keys = input.keys();
    let mut root = keys.last().expect("must be nonempty");

    loop {
        let node = &input[root];
        if node.parent == "" {
            return root;
        }
        root = &node.parent;
    }
}

fn second(root: &String, input: &HashMap<String, Disc>) -> usize {
    fn weight_of(id: &String, input: &HashMap<String, Disc>) -> usize {
        let node = &input[id];
        let children_weight = node.children.iter().map(|c| weight_of(c, &input)).sum::<usize>();
        return node.weight + children_weight;
    };

    fn unbalanced<'a>(entries: &Vec<(&'a String, usize)>) -> Option<(&'a String, usize, i32)> {
        let mut occurrences :HashMap<usize, Vec<&'a String>> = HashMap::new();
        for entry in entries {
            let &(id, weight) = entry;
            let entry = occurrences.entry(weight).or_insert(Vec::new());
            entry.push(id);
        }

        if occurrences.len() == 1 {
            return None{};
        }

        let invalid_entry_weight = occurrences.iter()
            .into_iter()
            .filter(|c| c.1.len() == 1)
            .map(|c| (c.0, c.1[0]))
            .nth(0)
            .expect("must exist");

        let valid_weight = occurrences.iter()
            .filter(|e| e.1.len() != 1)
            .map(|e| e.0)
            .nth(0)
            .expect("must exist");

        return Some((invalid_entry_weight.1, *valid_weight,
                (*valid_weight as i32) - (*invalid_entry_weight.0 as i32)));
    }

    let mut invalid_entry = root;
    let mut valid_weight = 0;
    let mut offset = 0;

    loop {
        let sums = input[invalid_entry].children.iter()
            .map(|c| (c, weight_of(c, input)))
            .collect::<Vec<(&String, usize)>>();

        match unbalanced(&sums) {
            None => return (input[invalid_entry].weight as i32 + offset) as usize,
            Some((ref ie, vw, o)) => {
                if input[invalid_entry].children.len() == 0 {
                    return valid_weight;
                } else {
                    invalid_entry = ie;
                    valid_weight = vw;
                    offset = o;
                }
            }
        }
    }
}

fn parse_line(line: &String, map: &mut HashMap<String, Disc>) {
    fn empty() -> Disc {
        return Disc{weight: 0, parent: String::new(), children: Vec::new()};
    }

    fn setup_node(id: String, weight: String, children: &Vec<String>, map: &mut HashMap<String, Disc>) {
        if !map.contains_key(&id) {
            map.insert(id.to_string(), empty());
        }

        let disc = map.get_mut(&id).expect("must exist");

        disc.weight = weight.parse::<usize>().expect("weight must be a number");
        disc.children = children.iter().map(|c| c.trim().to_string()).collect();
    }

    let parts: Vec<&str> = line.split("->").collect();
    let name_weight: Vec<&str> = parts[0].split(' ').collect();

    let id = name_weight[0].to_string();
    let weight: String = name_weight[1].chars().skip(1).take_while(|c| c.is_digit(10)).collect();
    let children: Vec<String> = if parts.len() == 2 {
        parts[1].split(", ").map(|e| e.to_string()).collect()
    } else {
        Vec::new()
    };

    setup_node(id.to_string(), weight, &children, map);

    for c in &children {
        let c_id = c.trim().to_string();
        if !map.contains_key(&c_id) {
            map.insert(c_id.to_string(), empty());
        }
        map.get_mut(&c_id).expect("must exist").parent = id.to_string();

    }
}

fn parse_stdin() -> HashMap<String, Disc> {
    let mut map = HashMap::new();

    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        parse_line(&line.unwrap(), &mut map);
    }

    return map;
}

fn main() {
    let input = parse_stdin();
    let root = first(&input);
    println!("first={}", root);
    println!("second={}", second(&root, &input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> HashMap<String, Disc> {
        let lines = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";
        let mut map = HashMap::new();

        for line in lines.split('\n') {
            parse_line(&line.to_string(), &mut map);
        }
        return map;
    }
    #[test]
    fn test_first() {
        assert_eq!("tknk".to_string(), *first(&input()));
    }

    #[test]
    fn test_second() {
        assert_eq!(60, second(&"tknk".to_string(), &input()));
    }
}
