use std::collections::HashSet;

pub fn score_byte(b: u8) -> u32 {
    if b < 91 {
        return (b - b'A' + 27).into();
    } else {
        return (b - b'a' + 1).into();
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut compartments1: Vec<HashSet<u8>> = Vec::new();
    let mut compartments2: Vec<HashSet<u8>> = Vec::new();
    input.lines().enumerate().for_each(|(i, l)| {
        compartments1.push(HashSet::new());
        compartments2.push(HashSet::new());
        let bytes: Vec<u8> = l.bytes().collect();
        bytes[..bytes.len() / 2].iter().for_each(|b| {
            compartments1[i].insert(*b);
        });
        bytes[bytes.len() / 2..].iter().for_each(|b| {
            compartments2[i].insert(*b);
        });
    });
    let mut score: u32 = 0;
    compartments1
        .iter()
        .zip(compartments2.iter())
        .for_each(|(s1, s2)| s1.intersection(s2).for_each(|v| score += score_byte(*v)));
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines : Vec<&str> = input.lines().collect();
    let mut score: u32 = 0;
    for lines in lines.chunks(3) {
        let mut bags: Vec<HashSet<u8>> = Vec::new();
        for line in lines {
            let bag : HashSet<u8> = line.bytes().collect();
            bags.push(bag);
        }
        let mut iter = bags.iter();
        let base = iter.next().unwrap().clone();
        let intersection = iter.fold(base, |s1, s2| s1.intersection(&s2).copied().collect());
        for v in intersection {
            score += score_byte(v);
        }
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}
