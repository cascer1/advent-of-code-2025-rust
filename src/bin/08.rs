use std::collections::HashSet;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (sorted_permutations, _) = parse_input(input);
    let line_count = input.lines().count();
    let desired_pairs: usize = if line_count == 20 { 10 } else { 1000 };

    let mut pair_count = 0;

    // keep track of only the junction box index, we don't need the actual coordinates in most cases
    let mut circuits: Vec<HashSet<usize>> = Vec::with_capacity(3);

    while pair_count < desired_pairs {
        let pair = sorted_permutations[pair_count];
        pair_count += 1;
        add_to_circuit(&mut circuits, pair);
    }

    circuits.sort_unstable_by_key(|s| std::cmp::Reverse(s.len()));

    let result = circuits
        .iter()
        .take(3)
        .map(|set| set.len() as u64)
        .product();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (sorted_permutations, junction_boxes) = parse_input(input);
    let line_count = input.lines().count();

    let mut pair_count = 0;
    let mut circuits: Vec<HashSet<usize>> = Vec::with_capacity(3);

    while circuits.is_empty() || circuits[0].len() != line_count {
        let pair = sorted_permutations[pair_count];
        pair_count += 1;
        add_to_circuit(&mut circuits, pair);

        if circuits[0].len() == line_count {
            let left = junction_boxes[pair.1];
            let right = junction_boxes[pair.2];

            return Some((left.0 * right.0) as u64)
        }
    }

    None
}

fn add_to_circuit(circuits: &mut Vec<HashSet<usize>>, pair: (u64, usize, usize)) {
    let left_index_option = is_in_circuit(pair.1, &circuits);
    let right_index_option = is_in_circuit(pair.2, &circuits);

    if let Some(left_index) = left_index_option {
        if let Some(right_index) = right_index_option {
            if left_index != right_index {
                let right = std::mem::take(&mut circuits[right_index]);
                circuits[left_index].extend(right);
                circuits.swap_remove(right_index);
            }
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

fn parse_input(input: &str) -> (Vec<(u64, usize, usize)>, Vec<(i64, i64, i64)>) {
    let mut junction_boxes: Vec<(i64, i64, i64)> = Vec::new();

    for line in input.lines() {
        let mut it = line.split(',');
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        let z: i64 = it.next().unwrap().parse().unwrap();
        junction_boxes.push((x, y, z));
    }

    let mut sorted_permutations: Vec<(u64, usize, usize)> = Vec::new();

    for i in 0..junction_boxes.len() {
        for j in (i + 1)..junction_boxes.len() {
            let left = junction_boxes[i];
            let right = junction_boxes[j];

            let distance = ((left.0 - right.0).pow(2))
                + ((left.1 - right.1).pow(2))
                + ((left.2 - right.2).pow(2));

            sorted_permutations.push((distance as u64, i, j));
        }
    }

    sorted_permutations.sort_unstable_by_key(|x| x.0);
    (sorted_permutations, junction_boxes)
}

fn is_in_circuit(box_index: usize, circuits: &[HashSet<usize>]) -> Option<usize> {
    circuits.iter().position(|v| v.contains(&box_index))
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
