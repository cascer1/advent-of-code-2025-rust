use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| solve_line_p1(l)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| solve_line_p2(l)).sum())
}

fn solve_line_p1(line: &str) -> u64 {
    let mut parts = line.split_whitespace().into_iter();

    let desired_pattern = parts.next().unwrap();
    let light_count = desired_pattern.len() - 2;

    let desired_state = desired_pattern
        .trim_matches(['[', ']'])
        .chars()
        .enumerate()
        .fold(0u16, |mut acc, (i, ch)| {
            if ch == '#' {
                acc |= 1u16 << i;
            }
            acc
        });

    let mut buttons: Vec<u16> = Vec::with_capacity(4);
    let mut previous_part = parts.next().unwrap();
    while previous_part.starts_with('(') {
        buttons.push(
            previous_part
                .trim_matches(['(', ')'])
                .split(',')
                .map(|l| l.parse::<u16>().unwrap())
                .fold(0u16, |mut acc, ch| {
                    acc |= 1u16 << ch;
                    acc
                }),
        );
        previous_part = parts.next().unwrap();
    }

    let size = 1usize << light_count;
    let mut visited = vec![0u8; size];
    let mut queue = VecDeque::with_capacity(265);

    visited[0u16 as usize] = 1;

    queue.push_back(0u16);

    let mut presses = 0;
    while !queue.is_empty() {
        presses += 1;

        for _ in 0..queue.len() {
            let state = queue.pop_front().unwrap();
            for &b in buttons.iter() {
                let next = state ^ b;
                let index = next as usize;

                if visited[index] == 0 {
                    // we haven't seen this state before
                    if next == desired_state {
                        return presses;
                    }
                    visited[index] = 1;
                    queue.push_back(next);
                }
            }
        }
    }

    panic!("Unable to find a button combo");
}

fn solve_line_p2(line: &str) -> u64 {
    let mut parts = line.split_whitespace().into_iter().skip(1);

    let mut buttons: Vec<Vec<usize>> = Vec::new();
    let mut previous_part = parts.next().unwrap();
    while previous_part.starts_with('(') {
        buttons.push(
            previous_part
                .trim_matches(['(', ')'])
                .split(",")
                .map(|l| l.parse::<usize>().unwrap())
                .collect(),
        );
        previous_part = parts.next().unwrap();
    }

    let desired_state: Vec<u16> = previous_part
        .trim_matches(['{', '}'])
        .split(",")
        .map(|l| l.parse::<u16>().unwrap())
        .collect();

    let initial_state = vec![0; desired_state.len()];

    let mut queue: VecDeque<(Vec<u16>, u64)> = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((initial_state, 0));

    while let Some((state, presses)) = queue.pop_front() {
        'button: for i in 0..buttons.len() {
            let next = press_button(&state, &buttons[i]);

            if visited.insert(next.clone()) {
                // this is a new state
                if next == desired_state {
                    return presses + 1;
                }

                for (index, value) in next.iter().enumerate() {
                    if value > &desired_state[index] {
                        continue 'button;
                    }
                }

                queue.push_back((next, presses + 1));
            }
        }
    }

    0
}

fn press_button(state: &Vec<u16>, button: &Vec<usize>) -> Vec<u16> {
    let mut new_state = state.clone();

    for &index in button.iter() {
        new_state[*&index] += 1;
    }

    new_state
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
