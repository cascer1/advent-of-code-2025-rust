advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let result = find_largest_number(line, 2);
        sum += result;
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;

    for line in input.lines() {
        let result = find_largest_number(line, 12);
        sum += result;
    }

    Some(sum)
}

fn find_largest_number(input: &str, num_digits: usize) -> u64 {
    let mut found_digits: Vec<char> = Vec::with_capacity(num_digits);
    let mut last_index: usize = 0;
    let mut found_count = 0;
    let input_length = input.chars().count() - 1;

    while found_count < num_digits {
        let max_index = input_length - (num_digits - found_count - 1);

        let mut largest_number = '0';
        let mut largest_index = 0;

        for index in last_index..=max_index {
            let digit = input.chars().nth(index).unwrap();
            if digit > largest_number {
                largest_number = digit;
                largest_index = index;
            }
        }

        found_digits.insert(found_count, largest_number);
        last_index = largest_index + 1;
        found_count += 1;
    }

    let mut result: u64 = 0;

    for (index, digit) in found_digits.iter().enumerate() {
        let power: u64 = (num_digits - index - 1) as u64;
        let this_digit = digit.to_digit(10).unwrap() as u64 * 10_u64.pow(power.try_into().unwrap());

        result += this_digit;
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
