pub fn calculate_calories_per_elf(input: &str) -> Option<Vec<u32>> {
    let chunks : Vec<&str> = input.split("\n\n").collect();
    let calories_per_elf_per_item : Vec<Vec<u32>> = chunks.iter().map(|s| -> Vec<u32> { s.lines().map(|l| l.parse().unwrap()).collect() }).collect();
    Some(calories_per_elf_per_item.iter().map(|v| v.iter().sum()).collect())
}
pub fn part_one(input: &str) -> Option<u32> {
    let calories_per_elf : Vec<u32> = calculate_calories_per_elf(input).unwrap();
    Some(calories_per_elf.iter().max().unwrap().to_owned())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories_per_elf : Vec<u32> = calculate_calories_per_elf(input).unwrap();
    calories_per_elf.sort();
    calories_per_elf.reverse();
    Some(calories_per_elf[0..3].iter().sum())
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
        assert_eq!(part_one(&input), Some(69795));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
