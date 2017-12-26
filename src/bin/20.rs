use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

type Vec3 = [i64; 3];

#[derive(Clone)]
struct Particle {
    id: usize,
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
}

fn manhattan(vec: &Vec3) -> i64 {
    return vec[0].abs() + vec[1].abs() + vec[2].abs();
}

impl Particle {
    pub fn new(id: usize, position: Vec3, velocity: Vec3, acceleration: Vec3) -> Particle {
        return Particle { id, position, velocity, acceleration };
    }

    pub fn from_str(id: usize, string: &String) -> Particle {
        let parts = string.split(", ");
        let mut vecs = Vec::new();

        for part in parts {
            let vec: Vec<i64> = part[3..part.len() - 1].split(',')
                .into_iter()
                .map(|n| n.parse::<i64>().expect("must be a number"))
                .collect();
            vecs.push([vec[0], vec[1], vec[2]]);
        }
        return Particle::new(id, vecs[0], vecs[1], vecs[2]);
    }

    pub fn tick(&mut self) -> Vec3 {
        for i in 0..3 {
            self.velocity[i] += self.acceleration[i];
            self.position[i] += self.velocity[i];
        }
        return self.position;
    }
}

fn first(input: &Vec<Particle>) -> usize {
    let mut particles = input.clone();

    for _ in 0..1000 {
        for p in particles.iter_mut() {
            p.tick();
        }
    }
    let mut closest = particles[0].id;
    let mut closest_dist = manhattan(&particles[0].position);

    for p in particles.iter().skip(1) {
        let dist = manhattan(&p.position);
        if dist < closest_dist {
            closest = p.id;
            closest_dist = dist;
        }
    }
    return closest;
}

fn second(input: &Vec<Particle>) -> usize {
    let mut particles = input.clone();
    let mut positions: HashMap<Vec3, Vec<usize>> = HashMap::new();

    for _ in 0..1000 {
        positions.clear();
        for particle in particles.iter_mut() {
            let pos = particle.tick();
            if !positions.contains_key(&pos) {
                positions.insert(pos, Vec::new());
            }
            positions.get_mut(&pos).expect("must exist").push(particle.id);
        }
        particles.retain(|particle| positions[&particle.position].len() == 1);
    }

    return particles.len();
}

fn parse_stdin() -> Vec<Particle> {
    let stdin = io::stdin();

    return stdin.lock().lines()
        .into_iter()
        .enumerate()
        .map(|(i, l)| Particle::from_str(i, &l.unwrap()))
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
        let input = vec!(
            Particle::new(0, [4, 0, 0], [0, 0, 0], [-2, 0, 0]),
            Particle::new(1, [3, 0, 0], [2, 0, 0], [-1, 0, 0]),
        );
        assert_eq!(1, first(&input));
    }

    #[test]
    fn test_second() {
        let input = vec!(
            Particle::new(0, [-6, 0, 0], [3, 0, 0], [0, 0, 0]),
            Particle::new(1, [-4, 0, 0], [2, 0, 0], [0, 0, 0]),
            Particle::new(2, [-2, 0, 0], [1, 0, 0], [0, 0, 0]),
            Particle::new(3, [3, 0, 0], [-1, 0, 0], [0, 0, 0]),
        );
        assert_eq!(1, second(&input));
    }
}
