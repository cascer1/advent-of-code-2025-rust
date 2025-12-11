use std::collections::{HashMap, HashSet};

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut graph, inverse_graph, id_list) = parse_input(input);

    let out_id = id_list.iter().position(|r| *r == "out").unwrap();
    let you_id = id_list.iter().position(|r| *r == "you").unwrap();

    let mut can_find_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    can_find_map.insert(
        out_id,
        calculate_can_reach_set(
            &inverse_graph,
            out_id,
        ),
    );

    graph = prune_invalid_nodes(graph, can_find_map.get(&out_id).unwrap());
    let mut visited = vec![false; id_list.len()];

    Some(get_number_paths_with_mandatory_visits(
        you_id,
        out_id,
        &graph,
        &[],
        &mut visited,
        &can_find_map,
    ))
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut graph, inverse_graph, id_list) = parse_input(input);

    let mut can_find_map: HashMap<usize, HashSet<usize>> = HashMap::new();

    let dac_id = id_list.iter().position(|r| *r == "dac").unwrap();
    let fft_id = id_list.iter().position(|r| *r == "fft").unwrap();
    let out_id = id_list.iter().position(|r| *r == "out").unwrap();
    let svr_id = id_list.iter().position(|r| *r == "svr").unwrap();

    can_find_map.insert(
        dac_id,
       calculate_can_reach_set(
            &inverse_graph,
            dac_id,
        ),
    );
    can_find_map.insert(
        fft_id,
        calculate_can_reach_set(
            &inverse_graph,
            fft_id,
        ),
    );
    can_find_map.insert(
        out_id,
        calculate_can_reach_set(
            &inverse_graph,
            out_id,
        ),
    );

    graph = prune_invalid_nodes(graph, can_find_map.get(&out_id).unwrap());

    if can_find_map.get(&fft_id).unwrap().contains(&dac_id) {
        panic!("Forward Fourier Transforms don't work on analog signals!")
    }

    // Logically, an FFT needs to be before a DAC, because FFT's work on digital signals
    // So let's split the problem into three separate steps
    // SVR --> FFT --> DAC --> OUT

    let mut visited = vec![false; id_list.len()];

    let step1 = get_number_paths_with_mandatory_visits(
        svr_id,
        fft_id,
        &graph,
        &[],
        &mut visited,
        &can_find_map,
    );

    let step2 = get_number_paths_with_mandatory_visits(
        fft_id,
        dac_id,
        &graph,
        &[],
        &mut visited,
        &can_find_map,
    );

    let step3 = get_number_paths_with_mandatory_visits(
        dac_id,
        out_id,
        &graph,
        &[],
        &mut visited,
        &can_find_map,
    );

    Some(step1 * step2 * step3)
}

fn prune_invalid_nodes(graph: HashMap<usize, Vec<usize>>, can_visit_out_map: &HashSet<usize>) -> HashMap<usize, Vec<usize>> {
    let mut result = HashMap::new();

    for entry in graph.iter() {
        let (from, to) = entry;
        let mut new_to: Vec<usize> = Vec::new();
        if !can_visit_out_map.contains(from) {
            continue;
        }

        for dest in to {
            if can_visit_out_map.contains(dest) {
                new_to.push(*dest);
            }
        }

        if !new_to.is_empty() {
            result.insert(*from, new_to);
        }
    }

    result
}

fn calculate_can_reach_set(
    inverse_graph: &HashMap<usize, Vec<usize>>,
    start: usize,
) -> HashSet<usize> {
    let mut seen: HashSet<usize> = HashSet::new();
    let mut stack: Vec<usize> = vec![start];

    while let Some(u) = stack.pop() {
        if !seen.insert(u) {
            continue; // already processed
        }
        if let Some(parents) = inverse_graph.get(&u) {
            for &p in parents {
                if !seen.contains(&p) {
                    stack.push(p);
                }
            }
        }
    }

    seen
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>, Vec<&str>) {
    let mut ids = HashSet::new();
    for line in input.lines() {
        let parts = line.split_whitespace();
        for part in parts {
            let inserted = part.trim_end_matches(':');
            ids.insert(inserted);
        }
    }

    let mut id_list: Vec<&str> = ids.into_iter().collect();
    id_list.sort();

    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut inverse_graph: HashMap<usize, Vec<usize>> = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let source = parts.next().unwrap().trim_end_matches(':');
        let source_id = id_list.iter().position(|r| *r == source).unwrap();
        for destination in parts {
            let destination_id = id_list.iter().position(|r| *r == destination).unwrap();
            graph.entry(source_id).or_default().push(destination_id);
            inverse_graph.entry(destination_id).or_default().push(source_id);
        }
    }

    (graph, inverse_graph, id_list)
}

// TODO: optimization improvement: add memoization
fn get_number_paths_with_mandatory_visits(
    source: usize,
    destination: usize,
    destinations: &HashMap<usize, Vec<usize>>,
    mandatory_visits: &[usize],
    visited: &mut Vec<bool>,
    can_reach_map: &HashMap<usize, HashSet<usize>>,
) -> u64 {
    if !can_reach_map.get(&destination).unwrap().contains(&source) {
        return 0;
    }

    let next_steps = destinations.get(&source);

    if next_steps.is_none() {
        return 0;
    }

    if visited[source] {
        return 0;
    }

    for mandatory_visit in mandatory_visits {
        if !visited[*mandatory_visit] {
            // check if the current node can reach all remaining mandatory visits
            if !can_reach_map
                .get(mandatory_visit)
                .unwrap()
                .contains(&source)
            {
                return 0;
            }
        }
    }

    visited[source] = true;

    let mut result = 0;

    for next_step in next_steps.unwrap() {
        if *next_step == destination && mandatory_visits.iter().all(|item| visited[*item]) {
            result += 1;
        } else {
            result += get_number_paths_with_mandatory_visits(
                *next_step,
                destination,
                destinations,
                mandatory_visits,
                visited,
                can_reach_map,
            );
        }
    }

    // intermediate_results[source] = Some(result);
    visited[source] = false;
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_one_real() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(670));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(670));
    }
}
