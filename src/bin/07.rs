use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    // Let's go line by line instead of first parsing
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let width = first_line.len();

    let mut last_beam_locations: HashSet<usize> = HashSet::new();

    // the first line is special, it only contains a start position
    let start_pos = first_line.find('S').unwrap();
    last_beam_locations.insert(start_pos);
    let mut split_count = 0u64;

    for line in lines {
        let splitter_positions = line
            .char_indices()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        if splitter_positions.is_empty() {
            continue;
        }

        let mut new_beam_locations = HashSet::new();

        let mut line_split_count = 0u64;

        for x in last_beam_locations.iter() {
            if splitter_positions.contains(x) {
                line_split_count += 1;

                if x > &0 {
                    new_beam_locations.insert(*x - 1);
                }

                if x + 1 < width {
                    new_beam_locations.insert(*x + 1);
                }
            } else {
                new_beam_locations.insert(*x);
            }
        }

        split_count += line_split_count;
        last_beam_locations = new_beam_locations;
    }

    Some(split_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    // Let's go line by line instead of first parsing
    let mut lines = input.lines();

    let first_line = lines.next().unwrap();
    let width = first_line.len();

    // key = x, value = number of worlds
    let mut last_beam_locations: HashMap<usize, usize> = HashMap::new();

    // the first line is special, it only contains a start position
    let start_pos = first_line.find('S').unwrap();
    last_beam_locations.insert(start_pos, 1);

    for line in lines {
        let splitter_positions = line
            .char_indices()
            .filter(|(_, c)| *c == '^')
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();

        if splitter_positions.is_empty() {
            continue;
        }

        let mut new_beam_locations: HashMap<usize, usize> = HashMap::new();

        for (&x, &count) in &last_beam_locations {
            if splitter_positions.contains(&x) {
                if x > 0 {
                    *new_beam_locations.entry(x - 1).or_insert(0) += count;
                }

                if x + 1 < width {
                    *new_beam_locations.entry(x + 1).or_insert(0) += count;
                }
            } else {
                *new_beam_locations.entry(x).or_insert(0) += count;
            }
        }

        last_beam_locations = new_beam_locations;
    }

    let world_count = last_beam_locations.values().map(|x| *x as u64).sum::<u64>();
    Some(world_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
