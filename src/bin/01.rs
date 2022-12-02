pub fn part_one(input: &str) -> Option<u32> {
    let chunks : Vec<&str> = input.split("\n\n").collect();
    let calories_per_elf_per_item : Vec<Vec<u32>> = chunks.iter().map(|s| -> Vec<u32> { s.lines().map(|l| l.parse().unwrap()).collect() }).collect();
    let calories_per_elf : Vec<u32> = calories_per_elf_per_item.iter().map(|v| v.iter().sum()).collect();
    Some(calories_per_elf.iter().max().unwrap().to_owned())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
