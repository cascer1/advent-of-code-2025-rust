advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    let mut pieces = [0usize; 6];
    let mut piece_size = 0;
    let mut trees_that_can_fit: usize = 0;
    let mut piece_index = 0;

    for line in input.lines() {
        if line.is_empty() {
            pieces[piece_index] = piece_size;
            piece_size = 0;
            piece_index += 1;
            continue;
        }

        if line.contains('x') {
            // we're parsing a tree
            let (dimensions, required_strings) = line.split_once(':').unwrap();

            let area_parts  = dimensions
                .split_once('x')
                .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                .unwrap();

            let area = area_parts.0 * area_parts.1;

            let required_size: usize = required_strings
                .split_ascii_whitespace() // slightly faster than .split_whitespace()
                .zip(pieces.iter())
                .map(|(count, &size)| count.parse::<usize>().unwrap() * size)
                .sum();

            if required_size <= area {
                trees_that_can_fit += 1;
            }
        } else {
            // technically we're including the piece label too, but that doesn't have #'s, so it's fine.
            piece_size += line.as_bytes().iter().filter(|&&c| c == b'#').count();
        }
    }

    Some(trees_that_can_fit as u64)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
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
