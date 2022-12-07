use regex::Regex;
use std::str;

struct Action {
    from: usize,
    to: usize,
    quantity: usize,
}

fn initialize_stacks(stacks: &mut Vec<Vec<u8>>, n_stacks : usize) -> () {
    for _ in 0..n_stacks {
        stacks.push(Vec::new())
    }
}

fn parse_stacks(input: &str) -> Option<Vec<Vec<u8>>> {
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    let mut number_stacks : usize = 0;
    for line in input.lines() {
        let bytes = line.as_bytes();
        if bytes[1] == b'1' {
            break;
        }
        if stacks.is_empty() {
            number_stacks = (line.len() + 1) / 4;
            initialize_stacks(&mut stacks, number_stacks);
        }
        for i in 0..number_stacks {
            match bytes[4 * i + 1] {
                b'A'..=b'Z' => stacks[i].push(bytes[4 * i + 1]),
                b' ' => (),
                _ => return None
            }
        }
    }
    for stack in stacks.as_mut_slice() {
        stack.reverse();
    }
    Some(stacks)
}

fn parse_actions(input: &str) -> Option<Vec<Action>> {
    let mut actions : Vec<Action> = Vec::new();
    let re = Regex::new(r"^move (\d*) from (\d*) to (\d*)$").ok()?;
    for line in input.lines() {
        if !line.contains("move") {
            continue;
        }
        let cap = re.captures_iter(line).next()?;
        let quantity : usize = cap[1].parse::<usize>().ok()?;
        let from : usize = cap[2].parse::<usize>().ok()? - 1;
        let to : usize = cap[3].parse::<usize>().ok()? - 1;
        let action = Action{from, to, quantity};
        actions.push(action);
    }
    Some(actions)
}

pub fn part_one(input: &str) -> Option<String> {
    let mut stacks = parse_stacks(input)?;
    let actions = parse_actions(input)?;
    for action in actions {
        for _ in 0..action.quantity {
            let c = stacks[action.from].pop()?;
            stacks[action.to].push(c);
        }
    }

    let mut buf : Vec<u8> = Vec::new();

    for stack in stacks {
        if stack.last().is_some() {
            buf.push(*stack.last().unwrap());
        }
    }
    let s = String::from_utf8(buf).ok()?;
    Some(s)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut stacks = parse_stacks(input)?;
    let actions = parse_actions(input)?;
    for action in actions {
        let mut temp_stack : Vec<u8> = Vec::new();
        for _ in 0..action.quantity {
            let c = stacks[action.from].pop()?;
            temp_stack.push(c);
        }
        temp_stack.reverse();
        for c in temp_stack {
            stacks[action.to].push(c);
        }
    }

    let mut buf : Vec<u8> = Vec::new();

    for stack in stacks {
        if stack.last().is_some() {
            buf.push(*stack.last().unwrap());
        }
    }
    let s = String::from_utf8(buf).ok()?;
    Some(s)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}
