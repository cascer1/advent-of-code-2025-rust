advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (fresh_ingredients, available_ids) = parse_input(input);
    let mut fresh_available: Vec<u64> = Vec::new();

    for id in available_ids {
        if id_is_fresh(&fresh_ingredients, id) {
            fresh_available.push(id);
        }
    }

    Some(fresh_available.len() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (fresh_ingredients, _) = parse_input(input);

    let mut sum = 0;

    for (start, end) in fresh_ingredients {
        sum += end - start + 1;
    }

    Some(sum)
}

fn id_is_fresh(fresh_ingredients: &Vec<(u64, u64)>, id: u64) -> bool {
    for (start, end) in fresh_ingredients {
        if id >= *start && id <= *end {
            return true;
        }
    }

    false
}

fn normalize_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return ranges;
    }

    ranges.sort_unstable_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(&right.1)));

    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    let mut current = ranges[0];

    for (start, end) in ranges.into_iter().skip(1) {
        // Merge if overlapping or directly adjacent
        if start <= current.1.saturating_add(1) {
            if end > current.1 {
                current.1 = end;
            }
        } else {
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);
    merged
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut fresh_ids: Vec<(u64, u64)> = Vec::new();
    let mut available_ids: Vec<u64> = Vec::new();

    let mut parsing_available = false;

    for line in input.lines() {
        if parsing_available {
            let number = line.parse().unwrap();
            available_ids.push(number);
        } else {
            if line.is_empty() {
                parsing_available = true;
                continue;
            }

            // split line on "-", then convert left and right to u64
            let (left_s, right_s) = line.split_once('-').expect("expected a single dash");
            let from: u64 = left_s.parse().expect("left not a number");
            let to: u64 = right_s.parse().expect("right not a number");

            fresh_ids.push((from, to));
        }
    }

    (normalize_ranges(fresh_ids), available_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
