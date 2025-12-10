use std::collections::VecDeque;
use z3::{ast::Int, Optimize, SatResult};

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| solve_line_p1(l)).sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(input.lines().map(|l| solve_line_p2(l)).sum())
}

fn solve_line_p1(line: &str) -> u64 {
    let mut parts = line.split_whitespace().into_iter();

    let desired_pattern = parts.next().expect("missing pattern");
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
    let mut token = parts.next().expect("missing token");
    while token.starts_with('(') {
        buttons.push(
            token
                .trim_matches(['(', ')'])
                .split(',')
                .map(|l| l.parse::<u16>().expect("invalid number"))
                .fold(0u16, |mut acc, ch| {
                    acc |= 1u16 << ch;
                    acc
                }),
        );
        token = parts.next().expect("missing token");
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
            let state = queue.pop_front().expect("queue cannot be empty");
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
   let mut parts = line.split_whitespace().skip(1);

   let mut buttons: Vec<Vec<usize>> = Vec::new();
   let mut token = parts.next().unwrap_or("");
   while token.starts_with('(') {
       let indices: Vec<usize> = token
           .trim_matches(['(', ')'])
           .split(',')
           .filter(|s| !s.is_empty())
           .map(|s| s.parse::<usize>().unwrap())
           .collect();
       buttons.push(indices);
       token = parts.next().unwrap_or("");
   }

   let targets: Vec<i64> = token
       .trim_matches(['{', '}'])
       .split(',')
       .filter(|s| !s.is_empty())
       .map(|s| s.parse::<i64>().unwrap())
       .collect();

   let opt = Optimize::new();

   // Decision variables light_index >= 0, integer
   let light_indices: Vec<Int> = (0..buttons.len())
       .map(|i| Int::new_const(format!("light_{i}")))
       .collect();
   let zero = Int::from_i64(0);
   for light_index in &light_indices {
       opt.assert(&light_index.ge(&zero));
   }

   // Constraints per light/counter
   for light_index in 0..targets.len() {
       let mut terms: Vec<&Int> = Vec::new();
       for (button_index, button) in buttons.iter().enumerate() {
           if button.iter().any(|&i| i == light_index) {
               terms.push(&light_indices[button_index]);
           }
       }
       let left = if terms.is_empty() {
           zero.clone()
       } else {
           Int::add(&terms)
       };
       let rhs = Int::from_i64(targets[light_index]);
       opt.assert(&left.eq(&rhs));
   }

   // Objective: minimize total presses
   let sum_all = if light_indices.is_empty() {
       zero.clone()
   } else {
       let refs: Vec<&Int> = light_indices.iter().collect();
       Int::add(&refs)
   };
   opt.minimize(&sum_all);

   match opt.check(&[]) {
       SatResult::Sat | SatResult::Unknown => {
           // Extract model and compute total presses
           let model = opt.get_model().expect("model expected");
           let mut total: u64 = 0;
           for x in &light_indices {
               let v = model
                   .eval(x, true)
                   .and_then(|n| n.as_u64())
                   .expect("integer value");
               total += v;
           }
           total
       }
       SatResult::Unsat => panic!("No solution for line: {}", line),
   }
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
