advent_of_code::solution!(6);

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
    None
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
        assert_eq!(result, None);
    }
}
