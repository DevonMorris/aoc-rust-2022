#[derive(PartialEq)]
enum Entry {
    File { size: usize, name: String },
    Dir { name: String },
}

#[derive(Debug)]
struct Node<T>
where
    T: PartialEq,
{
    idx: usize,
    val: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

#[derive(Debug, Default)]
struct Tree<T>
where
    T: PartialEq,
{
    nodes: Vec<Node<T>>,
}

impl<T> Node<T>
where
    T: PartialEq,
{
    fn new(idx: usize, val: T) -> Self {
        Self {
            idx,
            val,
            parent: None,
            children: vec![],
        }
    }
}

impl<T> Tree<T>
where
    T: PartialEq,
{
    fn new() -> Self {
        Self { nodes: vec![] }
    }
}

// This is some of the worst code I have ever written
fn parse_tree(input: &str) -> Option<Tree<Entry>> {
    let mut tree: Tree<Entry> = Tree::new();
    tree.nodes.push(Node::new(
        0,
        Entry::Dir {
            name: "/".to_string(),
        },
    ));

    let mut current_idx = 0;
    for line in input.lines() {
        if line.contains("$ ls") {
            continue;
        }
        if line.contains("$ cd /") {
            current_idx = 0
        } else if line.contains("$ cd ..") {
            current_idx = tree.nodes[current_idx].parent.unwrap();
        } else if line.contains("$ cd") {
            let target = line.split_at(5).1;
            let idxs: Vec<usize> = tree.nodes[current_idx]
                .children
                .iter()
                .copied()
                .filter_map(|i| match &tree.nodes[i].val {
                    Entry::File { .. } => None,
                    Entry::Dir { name } => {
                        if name == target {
                            Some(i)
                        } else {
                            None
                        }
                    }
                })
                .collect();
            if idxs.is_empty() {
                return None;
            }
            current_idx = idxs[0];
        } else if line.contains("dir ") {
            let val = Entry::Dir {
                name: line.split_at(4).1.to_string(),
            };
            let parent = Some(current_idx);
            let idx = tree.nodes.len();
            let children: Vec<usize> = vec![];
            tree.nodes.push(Node {
                val,
                parent,
                idx,
                children,
            });
            tree.nodes[current_idx].children.push(idx);
        } else {
            let mut splits = line.split(" ");
            let size: usize = splits.next().unwrap().parse().ok().unwrap();
            let name = splits.next().unwrap().to_string();
            let val = Entry::File { name, size };
            let parent = Some(current_idx);
            let idx = tree.nodes.len();
            let children: Vec<usize> = vec![];
            tree.nodes.push(Node {
                val,
                parent,
                idx,
                children,
            });
            tree.nodes[current_idx].children.push(idx);
        }
    }
    Some(tree)
}

fn computer_dir_size_top(tree: &Tree<Entry>, idx: usize) -> Option<usize> {
    match tree.nodes[idx].val {
        Entry::File { .. } => None,
        Entry::Dir { .. } => Some(compute_dir_size(tree, idx)),
    }
}

// This should really be memoized in some way
fn compute_dir_size(tree: &Tree<Entry>, idx: usize) -> usize {
    match tree.nodes[idx].val {
        Entry::File { size, .. } => size,
        Entry::Dir { .. } => tree.nodes[idx]
            .children
            .iter()
            .copied()
            .fold(0, |acc, i| acc + compute_dir_size(tree, i)),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let tree = parse_tree(input).unwrap();
    let sum: usize = tree
        .nodes
        .iter()
        .filter_map(|i| computer_dir_size_top(&tree, i.idx))
        .filter(|s| s <= &100000)
        .sum();
    let sum : u32 = sum.try_into().ok()?;
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let tree = parse_tree(input).unwrap();

    let disk_size = 70000000;
    let space_needed = 30000000;
    let current_dir_size = compute_dir_size(&tree, 0);
    if current_dir_size > disk_size {
        return None;
    }
    let size_remaining = disk_size - current_dir_size;
    if size_remaining > space_needed {
        return None;
    }
    let space_needed = space_needed - size_remaining;
    let min: usize = tree
        .nodes
        .iter()
        .filter_map(|i| computer_dir_size_top(&tree, i.idx))
        .filter(|s| s >= &space_needed)
        .min()?;
    let min : u32 = min.try_into().ok()?;
    Some(min)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
