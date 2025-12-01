advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut position: i64 = 50;
    let mut times_at_zero = 0;
    let lines = parse(input);

    lines.into_iter().for_each(|(dir, steps)| {
        let (new_position, _at_zero) = do_move(position, steps, dir == 'R');
        position = new_position;
        if position == 0 {
            times_at_zero += 1;
        }
    });

    Some(times_at_zero)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = parse(input);

    let mut times_at_zero: u64 = 0;
    let mut position: i64 = 50;

    lines.into_iter().for_each(|(dir, steps)| {
        let (new_position, at_zero) = do_move(position, steps, dir == 'R');
        position = new_position;
        times_at_zero += at_zero;
    });

    Some(times_at_zero)
}

fn do_move(position: i64, steps: i64, is_positive: bool) -> (i64, u64) {
    let mut new_position;
    let mut full_revolutions = 0;

    if is_positive {
        new_position = position + steps;

        while new_position >= 100 {
            new_position -= 100;
            full_revolutions += 1;
        }
    } else {
        new_position = (position - steps) % 100;
        if new_position < 0 { new_position += 100; }

        full_revolutions = if position == 0 {
            steps / 100
        } else if steps < position {
            0
        } else  {
            1 + (steps - position) / 100
        };
    }

    (new_position, full_revolutions as u64)
}

fn parse(input: &str) -> Vec<(char, i64)> {
    let lines: Vec<(char, i64)> = input
        .lines()
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect();
    lines
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
        assert_eq!(result, Some(26));
    }
}
