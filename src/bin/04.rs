advent_of_code::solution!(4);

use advent_of_code::CardinalDirection;
use grid::*;
use strum::IntoEnumIterator;

pub fn part_one(input: &str) -> Option<u64> {
    let grid = parse_input(input);
    let mut accessible_count = 0;

    for (position, _) in grid.indexed_iter().filter(|(_, value)| **value) {
        let occupied_spots = get_occupied_spots(&grid, position);

        if occupied_spots < 4 {
            accessible_count += 1;
        }
    }

    Some(accessible_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = parse_input(input);
    let mut removable_count = 0;
    let mut removed_in_iteration = 1;

    while removed_in_iteration > 0 {
        removed_in_iteration = 0;
        let mut positions_to_remove: Vec<(usize, usize)> = Vec::new();

        for (position, _) in grid.indexed_iter().filter(|(_, value)| **value) {
            let occupied_spots = get_occupied_spots(&grid, position);

            if occupied_spots < 4 {
                positions_to_remove.push(position);
                removed_in_iteration += 1;
            }
        }

        removable_count += removed_in_iteration;

        for pos in &positions_to_remove {
            grid[*pos] = false;
        }
    }

    Some(removable_count)
}

fn get_occupied_spots(grid: &Grid<bool>, position: (usize, usize)) -> i32 {
    let mut occupied_spots = 0;

    CardinalDirection::iter()
        .map(|direction| direction.position_at_coords(position.0, position.1))
        .filter(|position| position.is_some())
        .map(|position| position.unwrap())
        .for_each(|check_position| {
            let check_value = grid.get(check_position.x, check_position.y);

            if let Some(&true) = check_value {
                occupied_spots += 1;
            }
        });
    occupied_spots
}

fn parse_input(input: &str) -> Grid<bool> {
    let line_length = input.lines().next().unwrap().len();
    let input_chars = input
        .replace('\n', "")
        .chars()
        .map(|character| character == '@')
        .collect::<Vec<_>>();

    Grid::from_vec(input_chars, line_length)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
