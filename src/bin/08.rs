#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

struct SpiralIterator {
    i: usize,
    j: usize,
    height: usize,
    width: usize,
    depth: usize,
    direction: Direction,
    num_visited: usize,
}

impl SpiralIterator {
    fn new(height: usize, width: usize) -> Self {
        Self {
            i: 0,
            j: 0,
            depth: 0,
            direction: Direction::Right,
            height,
            width,
            num_visited: 0,
        }
    }
}

impl Iterator for SpiralIterator {
    type Item = (usize, usize, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_visited == self.height * self.width {
            return None;
        }
        let prev_i = self.i;
        let prev_j = self.j;
        let prev_dir = self.direction;
        match self.direction {
            Direction::Right => {
                if self.j == (self.width - self.depth - 1) {
                    self.direction = Direction::Down;
                    self.i += 1;
                } else {
                    self.j += 1;
                }
            }
            Direction::Down => {
                if self.i == (self.height - self.depth - 1) {
                    self.direction = Direction::Left;
                    self.j -= 1;
                } else {
                    self.i += 1;
                }
            }
            Direction::Left => {
                if self.j == self.depth {
                    self.direction = Direction::Up;
                    self.i -= 1;
                } else {
                    self.j -= 1;
                }
            }
            Direction::Up => {
                if self.i == self.depth + 1 {
                    self.direction = Direction::Right;
                    self.depth += 1;
                    self.j += 1;
                } else {
                    self.i -= 1;
                }
            }
        };
        self.num_visited += 1;
        Some((prev_i, prev_j, prev_dir))
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<u8>>,
}

// I ended up not using all of these iterators but they were fun to learn about
struct GridSpiralIterator<'a> {
    iterator: SpiralIterator,
    grid: &'a Grid,
}

struct GridMutSpiralIterator<'a> {
    iterator: SpiralIterator,
    grid: &'a mut Grid,
}

impl<'a> GridSpiralIterator<'a> {
    fn new(grid: &'a Grid) -> Self {
        let iterator = SpiralIterator::new(grid.height(), grid.width());
        Self { iterator, grid }
    }
}

impl<'a> GridMutSpiralIterator<'a> {
    fn new(grid: &'a mut Grid) -> Self {
        let iterator = SpiralIterator::new(grid.height(), grid.width());
        Self { iterator, grid }
    }
}

impl<'a> Iterator for GridSpiralIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, _) = self.iterator.next()?;
        println!("{}, {}", i, j);
        Some(self.grid.data[i][j])
    }
}

impl<'a> Iterator for GridMutSpiralIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let (i, j, _) = self.iterator.next()?;
        Some(self.grid.data[i][j])
    }
}

impl Grid {
    fn new() -> Self {
        Grid { data: Vec::new() }
    }

    fn height(self: &Self) -> usize {
        self.data.len()
    }

    fn width(self: &Self) -> usize {
        self.data.first().unwrap().len()
    }

    fn spiral_iter(self: &Self) -> GridSpiralIterator {
        GridSpiralIterator::new(self)
    }

    fn spiral_mut_iter(self: &mut Self) -> GridMutSpiralIterator {
        GridMutSpiralIterator::new(self)
    }
}

fn parse_input(input: &str) -> Option<Grid> {
    let mut grid = Grid::new();
    for line in input.lines() {
        grid.data.push(Vec::new());
        for c in line.chars() {
            grid.data.last_mut()?.push(c.to_digit(10)?.try_into().ok()?);
        }
    }
    Some(grid)
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input)?;
    let mut visibilities: Vec<Vec<Option<u8>>> = Vec::new();
    for i in 0..grid.height() {
        visibilities.push(Vec::new());
        for j in 0..grid.width() {
            visibilities.last_mut()?.push(None);
        }
    }

    for i in 0..grid.height() {
        for j in 0..grid.width() {
            let val = grid.data[i][j];
            let mut case1 = true;
            let mut case2 = true;
            let mut case3 = true;
            let mut case4 = true;
            for k in 0..i {
                case1 = case1 && (grid.data[i][j] > grid.data[k][j]);
            }
            for k in i + 1..grid.height() {
                case2 = case2 && (grid.data[i][j] > grid.data[k][j]);
            }
            for k in 0..j {
                case3 = case3 && (grid.data[i][j] > grid.data[i][k]);
            }
            for k in j + 1..grid.width() {
                case4 = case4 && (grid.data[i][j] > grid.data[i][k]);
            }
            let visible = case1 || case2 || case3 || case4;
            if visible {
                visibilities[i][j] = Some(grid.data[i][j]);
            }
        }
    }
    Some(
        visibilities
            .iter()
            .flatten()
            .filter(|v| v.is_some())
            .count()
            .try_into()
            .ok()?,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input)?;

    let mut max_visibility: usize = 0;
    for i in 0..grid.height() {
        for j in 0..grid.width() {
            let tree_height = grid.data[i][j];
            let mut visibility1 = 0;
            if i != 0 {
                for k in (0..i).rev() {
                    visibility1 += 1;
                    if tree_height <= grid.data[k][j] {
                        break;
                    }
                }
            }

            let mut visibility2 = 0;
            for k in i + 1..grid.height() {
                visibility2 += 1;
                if tree_height <= grid.data[k][j] {
                    break;
                }
            }

            let mut visibility3 = 0;
            if j != 0 {
                for k in (0..j).rev() {
                    visibility3 += 1;
                    if tree_height <= grid.data[i][k] {
                        break;
                    }
                }
            }

            let mut visibility4 = 0;
            for k in j + 1..grid.width() {
                visibility4 += 1;
                if tree_height <= grid.data[i][k] {
                    break;
                }
            }

            let visibility = visibility1 * visibility2 * visibility3 * visibility4;
            if visibility > max_visibility {
                max_visibility = visibility;
            }
        }
    }
    Some(max_visibility.try_into().ok()?)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

