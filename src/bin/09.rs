use std::collections::BTreeMap;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let outer_points = parse_input(input);

    let mut upper_points: Vec<(i64, i64)> = Vec::new();
    let mut lower_points: Vec<(i64, i64)> = Vec::new();

    for (x, (min, max)) in outer_points.iter() {
        upper_points.push((*x, *max));
        lower_points.push((*x, *min));
    }

    let mut largest_area = 0;

    for upper_point in upper_points.iter() {
        for lower_point in lower_points.iter() {
            let x_range = (lower_point.0 - upper_point.0).abs() + 1;
            let y_range = (upper_point.1 - lower_point.1).abs() + 1;
            let area = x_range * y_range;
            largest_area = largest_area.max(area);
        }
    }

    Some(largest_area as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

fn parse_input(input: &str) -> BTreeMap<i64, (i64, i64)> {
    let mut result: BTreeMap<i64, (i64, i64)> = BTreeMap::new();

    for line in input.lines() {
        let (x, y) = line
            .split_once(',')
            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
            .unwrap();

        result
            .entry(x)
            .and_modify(|old| {
                if y < old.0 {
                    old.0 = y;
                }
                if y > old.1 {
                    old.1 = y;
                }
            })
            .or_insert((y, y));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
