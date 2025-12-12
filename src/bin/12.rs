use regex::Regex;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let (pieces, trees) = parse(input);

    let result = trees
        .iter()
        .map(|tree| tree.can_fit(&pieces))
        .filter(|&r| r == true)
        .count() as u64;

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse(input: &str) -> (Vec<Piece>, Vec<Tree>) {
    let mut pieces = Vec::with_capacity(6);
    let mut trees = Vec::with_capacity(input.lines().count() - 30);

    let mut piece_parts: Vec<&str> = Vec::with_capacity(3);

    let tree_regex = Regex::new("^[0-9]+x[0-9]+:").unwrap();
    let piece_index_regex = Regex::new("^[0-9]+:").unwrap();

    for line in input.lines() {
        if line.is_empty() {
            if piece_parts.len() == 3 {
                pieces.push(parse_piece(&piece_parts))
            }
            piece_parts.clear();
            continue;
        }

        if tree_regex.is_match(line) {
            // we're parsing a tree
            trees.push(parse_tree(line))
        } else {
            if piece_index_regex.is_match(line) {
                // simply piece index, ignore
                continue;
            }
            piece_parts.push(line);
        }
    }

    (pieces, trees)
}

fn parse_piece(piece_parts: &[&str]) -> Piece {
    let mut size: u64 = 0;
    for line in piece_parts {
        size += line.chars().filter(|c| *c == '#').count() as u64;
    }
    Piece { size }
}

fn parse_tree(line: &str) -> Tree {
    let (dimensions, required_strings) = line.split_once(':').unwrap();

    let (width, height) = dimensions
        .split_once('x')
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .unwrap();

    let required_list = required_strings
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    Tree {
        required_pieces: required_list,
        width,
        height,
    }
}

struct Piece {
    size: u64,
}

struct Tree {
    required_pieces: Vec<u64>,
    width: u64,
    height: u64,
}

impl Tree {
    fn area(&self) -> u64 {
        self.width * self.height
    }

    fn can_fit(&self, pieces: &[Piece]) -> bool {
        let mut required_size = 0;
        for (piece_index, piece_count) in self.required_pieces.iter().enumerate() {
            required_size += piece_count * pieces[piece_index].size;
        }
        required_size < self.area()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
