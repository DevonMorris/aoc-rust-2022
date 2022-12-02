use std::cmp::Ordering;

#[derive(PartialEq)]
pub enum RpsSelection {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RpsSelection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Ordering::*;
        match (self, other) {
            (RpsSelection::Rock, RpsSelection::Rock) => Some(Equal),
            (RpsSelection::Rock, RpsSelection::Paper) => Some(Less),
            (RpsSelection::Rock, RpsSelection::Scissors) => Some(Greater),
            (RpsSelection::Paper, RpsSelection::Rock) => Some(Greater),
            (RpsSelection::Paper, RpsSelection::Paper) => Some(Equal),
            (RpsSelection::Paper, RpsSelection::Scissors) => Some(Less),
            (RpsSelection::Scissors, RpsSelection::Rock) => Some(Less),
            (RpsSelection::Scissors, RpsSelection::Paper) => Some(Greater),
            (RpsSelection::Scissors, RpsSelection::Scissors) => Some(Equal),
        }
    }
}

pub struct Game {
    opponent_selection: RpsSelection,
    my_selection: RpsSelection,
}

impl Game {
    pub fn new(input: &str) -> Option<Self> {
        let choices_char: Vec<&str> = input.split(" ").collect();
        let opponent_selection = match choices_char[0] {
            "A" => RpsSelection::Rock,
            "B" => RpsSelection::Paper,
            "C" => RpsSelection::Scissors,
            _ => return None,
        };
        let my_selection = match choices_char[1] {
            "X" => RpsSelection::Rock,
            "Y" => RpsSelection::Paper,
            "Z" => RpsSelection::Scissors,
            _ => return None,
        };
        Some(Game {
            opponent_selection,
            my_selection,
        })
    }
    pub fn new2(input: &str) -> Option<Self> {
        let choices_char: Vec<&str> = input.split(" ").collect();
        let opponent_selection = match choices_char[0] {
            "A" => RpsSelection::Rock,
            "B" => RpsSelection::Paper,
            "C" => RpsSelection::Scissors,
            _ => return None,
        };
        let my_selection = match (&opponent_selection, choices_char[1]) {
            (RpsSelection::Rock, "X") => RpsSelection::Scissors,
            (RpsSelection::Paper, "X") => RpsSelection::Rock,
            (RpsSelection::Scissors, "X") => RpsSelection::Paper,
            (RpsSelection::Rock, "Y") => RpsSelection::Rock,
            (RpsSelection::Paper, "Y") => RpsSelection::Paper,
            (RpsSelection::Scissors, "Y") => RpsSelection::Scissors,
            (RpsSelection::Rock, "Z") => RpsSelection::Paper,
            (RpsSelection::Paper, "Z") => RpsSelection::Scissors,
            (RpsSelection::Scissors, "Z") => RpsSelection::Rock,
            (_, _) => return None,
        };
        Some(Game {
            opponent_selection,
            my_selection,
        })
    }
}

pub fn score_game(game: &Game) -> u32 {
    use Ordering::*;
    match game
        .my_selection
        .partial_cmp(&game.opponent_selection)
        .unwrap()
    {
        Equal => 3,
        Greater => 6,
        Less => 0,
    }
}

pub fn score_choice(game: &Game) -> u32 {
    match game.my_selection {
        RpsSelection::Rock => 1,
        RpsSelection::Paper => 2,
        RpsSelection::Scissors => 3,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| Game::new(l).unwrap())
            .map(|g| score_game(&g) + score_choice(&g))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| Game::new2(l).unwrap())
            .map(|g| score_game(&g) + score_choice(&g))
            .sum(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
