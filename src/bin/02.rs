advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    input.split(",").for_each(|range| {
        let edges = find_range_edges(range);
        let start = edges.first().expect("The vector should have at least one entry").to_owned();
        let end = edges.last().expect("The vector should have at least one entry").to_owned();

        for number in start..=end {
            let number_string = number.to_string();
            if is_repeating(number_string.as_bytes(), false) {
                sum += number;
            }
        }
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;

    input.split(",").for_each(|range| {
        let edges = find_range_edges(range);
        let start = edges.first().expect("The vector should have at least one entry").to_owned();
        let end = edges.last().expect("The vector should have at least one entry").to_owned();

        for number in start..=end {
            let number_string = number.to_string();
            if is_repeating(number_string.as_bytes(), true) {
                sum += number;
            }
        }
    });

    Some(sum)
}

fn find_range_edges(range: &str) -> Vec<u64> {
    range
        .split("-")
        .map(|x| x.trim().parse::<u64>().expect("Invalid number"))
        .collect::<Vec<_>>()
}

/// Calculate the KMP failure function
fn calculate_failure_function(input: &[u8]) -> Vec<usize> {
    let mut failure_function: Vec<usize> = vec![0; input.len()];

    let mut j: usize = 0;
    for i in 1..input.len() {
        while j > 0 && input[i] != input[j] {
            j = failure_function[j - 1];
        }
        if input[i] == input[j] {
            j += 1;
        }
        failure_function[i] = j;
    }

    failure_function
}

fn is_repeating(input: &[u8], multiple: bool) -> bool {
    if !multiple {
        let (part1, part2) = input.split_at(input.len() / 2);
        if part1 == part2 {
            return true;
        }
    } else {
        let failure_function = calculate_failure_function(input);

        let total_length = input.len();
        let last_failure = failure_function.last().unwrap();
        let pattern_length = total_length - last_failure;

        if last_failure > &0 && total_length.is_multiple_of(pattern_length) {
            return true;
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
