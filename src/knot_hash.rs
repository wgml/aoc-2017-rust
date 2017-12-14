use std::iter::FromIterator;

fn hash_round(lengths: &Vec<usize>, indices: &mut Vec<usize>, mut current_position: usize, mut skip_size: usize) -> (usize, usize) {
    for length in lengths {
        let mut to_process: Vec<usize> = Vec::with_capacity(*length);
        for i in 0..*length {
            to_process.push(indices[(current_position + i) % 256]);
        }

        to_process.reverse();
        for i in 0..*length {
            indices[(current_position + i) % 256] = to_process[i];
        }

        current_position = (current_position + skip_size + length) % 256;
        skip_size += 1;
    }

    return (current_position, skip_size);
}

pub fn hash<'a>(lengths: &Vec<usize>, mut indices: &'a mut Vec<usize>, rounds: usize) -> &'a Vec<usize> {
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..rounds {
        let result = hash_round(&lengths, &mut indices, current_position, skip_size);
        current_position = result.0;
        skip_size = result.1;
    }

    return indices;
}

pub fn dense_hash(input: &Vec<usize>) -> String {
    fn xor(input: &[usize]) -> u8 {
        return input.iter().fold(0, |x, v| x ^ v) as u8;
    }
    let mut indices: Vec<usize> = Vec::from_iter(0..256);

    let h = hash(&input, &mut indices, 64);

    let mut result = String::new();
    for chunk in h.chunks(16) {
        result += format!("{:02x}", xor(&chunk)).as_ref();
    }
    return result;
}

pub fn for_str(input: &String) -> String {
    let mut result = input.chars().map(|c| c as usize).collect::<Vec<usize>>();
    result.extend(vec!(17, 31, 73, 47, 23));
    return dense_hash(&result);
}
