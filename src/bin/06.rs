advent_of_code::solution!(6);
use pad::PadStr;

pub fn part_one(input: &str) -> Option<u64> {
    let (line_length, line_count, numbers, operators) = parse_input(input);
    let mut sum: u64 = 0;

    for problem_number in 0..line_length {
        let mut problem_numbers: Vec<u64> = Vec::with_capacity(3);
        for i in 0..line_count {
            problem_numbers.push(numbers[problem_number as usize + (line_length as usize * i as usize)]);
        }
        let result = solve_problem(&problem_numbers, operators[problem_number as usize]);
        sum += result;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = pad_lines(input);
    let width = lines.clone().first().map(|s| s.len()).unwrap();
    let chars: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let operator_index = lines.len() - 1;

    let mut sum: u64 = 0;

    let mut part_numbers: Vec<u64> = Vec::with_capacity(4);
    let mut part_operator: char = '0';

    for i in 0 .. width {
        let mut parts: Vec<&char> = Vec::with_capacity(5);

        for line in &chars {
            parts.push(line.get(i).unwrap())
        }

        let mut parts_iter = parts.iter();

        if parts.iter().all(|c| **c == ' ') {
            // completed part, time to calculate.
            sum += solve_problem(&part_numbers, part_operator);
            part_numbers.clear();
            part_operator = '0';
        }

        // Take what you need for the number
        let number_string: String = parts_iter.by_ref()
            .take(operator_index)
            .map(|c| **c)
            .collect();

        let trimmed_string = number_string.trim();

        if !trimmed_string.is_empty() {
            part_numbers.push(number_string.trim().parse::<u64>().unwrap());
        }

        // Now get the operator
        let operator = *parts_iter.next().unwrap().to_owned();

        if operator != ' ' {
            part_operator = operator;
        }
    }

    if part_operator != '0' && !part_numbers.is_empty() {
        sum += solve_problem(&part_numbers, part_operator);
    }

    Some(sum)
}

pub fn pad_lines(input: &str) -> Vec<String> {
    // Collect lines as owned Strings
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    // Find maximum byte length
    let max_len = lines.iter().map(|s| s.len()).max().unwrap_or(0);

    lines.iter().map(|s| s.pad_to_width(max_len)).collect()
}

fn solve_problem(numbers: &[u64], operator: char) -> u64 {
    match operator {
        '+' => numbers.iter().sum(),
        '*' => numbers.iter().product(),
        _ => panic!("Unknown operator: {}", operator),
    }
}

fn parse_input(input: &str) -> (u64, u8, Vec<u64>, Vec<char>) {
    let mut lines = input.lines();
    let mut line_count = 0u8;

    let first_line = lines.next().unwrap();
    let first_line_numbers = first_line
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap());
    let line_length = first_line_numbers.clone().count();

    let mut numbers: Vec<u64> = Vec::with_capacity(3 * line_length);
    let mut operators: Vec<char> = Vec::with_capacity(line_length);

    numbers.extend(first_line_numbers);

    for line in lines {
        let line_parts = line.split_whitespace();
        let mut parsing_numbers = true;
        let mut checked_type = false;
        line_count += 1;

        for part in line_parts {
            if !checked_type {
                parsing_numbers = part.parse::<f64>().is_ok();
                checked_type = true;
            }

            if parsing_numbers {
                numbers.push(part.parse::<u64>().unwrap());
            } else {
                operators.push(part.chars().next().unwrap());
            }
        }
    }

    (line_length as u64, line_count, numbers, operators)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
