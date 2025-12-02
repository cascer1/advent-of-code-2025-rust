use std::collections::HashSet;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut invalid_options: HashSet<u64> = HashSet::new();

    input.split(",").for_each(|range| {
        let edges = range
            .split("-")
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let start = edges.first().unwrap().to_owned();
        let end = edges.last().unwrap().to_owned();

        for number in start..=end {
            let number_string = number.to_string();
            if is_repeating(&number_string, false) {
                invalid_options.insert(number);
            }
        }
    });

    Some(invalid_options.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut invalid_options: HashSet<u64> = HashSet::new();

    input.split(",").for_each(|range| {
        let edges = range
            .split("-")
            .map(|x| x.trim().parse::<u64>().unwrap())
            .collect::<Vec<_>>();

        let start = edges.first().unwrap().to_owned();
        let end = edges.last().unwrap().to_owned();

        for number in start..=end {
            let number_string = number.to_string();
            if is_repeating(&number_string, true) {
                invalid_options.insert(number);
            }
        }
    });

    Some(invalid_options.iter().sum())
}

fn is_repeating(input: &str, multiple: bool) -> bool {
    if !multiple {
        let (part1, part2) = input.split_at(input.len() / 2);
        if part1 == part2 {
            return true;
        }
    } else {
        for part_length in 1..input.len() {
            if input.chars().count() % part_length == 0 {
                let subs = input
                    .as_bytes()
                    .chunks(part_length)
                    .map(str::from_utf8)
                    .collect::<Result<Vec<&str>, _>>()
                    .unwrap();

                if subs.iter().all(|part| *part == subs[0]) {
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
