use std::collections::HashSet;

pub fn find_unique_idx(input: &str, win_size : usize) -> Option<u32> {
    for (i, window) in input.as_bytes().windows(win_size).enumerate() {
        let win_set : HashSet<u8> = HashSet::from_iter(window.iter().cloned());
        if win_set.len() == win_size {
            let idx : u32 = (i + win_size).try_into().ok()?;
            return Some(idx);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    find_unique_idx(input, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    find_unique_idx(input, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
