use itertools::Itertools;
use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let sorted_permutations = parse_input(input);
    let desired_pairs: usize = if input.lines().count() == 20 {
        10
    } else {
        1000
    };

    let mut pair_count = 0;
    let mut circuits: Vec<HashSet<(u64, u64, u64)>> = Vec::new();

    while pair_count < desired_pairs {
        let pair = sorted_permutations[pair_count];

        let left_index = is_in_circuit(pair.1, &circuits);
        let right_index = is_in_circuit(pair.2, &circuits);

        if left_index.is_some() && right_index.is_none() {
            // add to "one" circuit
            circuits[left_index.unwrap()].insert(pair.2);
        } else if left_index.is_none() && right_index.is_some() {
            // add to "two" circuit
            circuits[right_index.unwrap()].insert(pair.1);
        } else if left_index.is_some() && right_index.is_some() {
            if left_index.unwrap() == right_index.unwrap() {
                pair_count += 1;
                continue;
            }
            // merge circuits
            let right = circuits[right_index.unwrap()].clone();
            circuits[left_index.unwrap()].extend(right);
            circuits.swap_remove(right_index.unwrap());
        } else {
            // new circuit
            let mut new_set = HashSet::new();
            new_set.insert(pair.1);
            new_set.insert(pair.2);
            circuits.push(new_set);
        }

        pair_count += 1;
    }

    circuits.sort_unstable_by(|a, b| a.len().cmp(&b.len()).reverse());

    let result = circuits
        .iter()
        .take(3)
        .map(|set| set.len() as u64)
        .product();

    Some(result)
}

fn parse_input(input: &str) -> Vec<(u64, (u64, u64, u64), (u64, u64, u64))> {
    let mut junction_boxes: Vec<(u64, u64, u64)> = Vec::new();

    for line in input.lines() {
        let parts = line
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<u64>>();
        junction_boxes.push((parts[0], parts[1], parts[2]))
    }

    junction_boxes.sort_unstable();

    let permutations = junction_boxes.iter().combinations(2);

    let mut sorted_permutations: Vec<(u64, (u64, u64, u64), (u64, u64, u64))> =
        Vec::with_capacity(junction_boxes.len() * junction_boxes.len());

    for permutation in permutations {
        let left = permutation[0];
        let right = permutation[1];

        let distance = (((left.0 as i64 - right.0 as i64).pow(2))
            + ((left.1 as i64 - right.1 as i64).pow(2))
            + ((left.2 as i64 - right.2 as i64).pow(2)))
        .isqrt();

        sorted_permutations.push((distance as u64, *left, *right));
    }

    sorted_permutations.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    sorted_permutations
}

fn is_in_circuit(
    junction_box: (u64, u64, u64),
    circuits: &Vec<HashSet<(u64, u64, u64)>>,
) -> Option<usize> {
    circuits
        .iter()
        .position(|v| v.iter().any(|p| p == &junction_box))
}

pub fn part_two(input: &str) -> Option<u64> {
    let sorted_permutations = parse_input(input);
    let line_count = input.lines().count();

    let mut pair_count = 0;
    let mut circuits: Vec<HashSet<(u64, u64, u64)>> = Vec::new();
    let mut pair: (u64, (u64, u64, u64), (u64, u64, u64)) = (0, (0, 0, 0), (0, 0, 0));

    while circuits.is_empty() || circuits[0].len() != line_count {
        pair = sorted_permutations[pair_count];
        pair_count += 1;

        let left_index_option = is_in_circuit(pair.1, &circuits);
        let right_index_option = is_in_circuit(pair.2, &circuits);

        if let Some(left_index) = left_index_option {
            if let Some(right_index) = right_index_option {
                if left_index == right_index {
                    continue;
                }
                let right = circuits[right_index].clone();
                circuits[left_index].extend(right);
                circuits.swap_remove(right_index);
            } else {
                circuits[left_index].insert(pair.2);
            }
        } else if let Some(right_index) = right_index_option {
            circuits[right_index].insert(pair.1);
        } else {
            // new circuit
            let mut new_set = HashSet::new();
            new_set.insert(pair.1);
            new_set.insert(pair.2);
            circuits.push(new_set);
        }
    }

    Some(pair.1.0 * pair.2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
