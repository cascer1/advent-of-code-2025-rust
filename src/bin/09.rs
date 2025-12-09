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
    let extreme_red_tiles = parse_input(input);
    let edges = parse_edges(input);
    let edge_outer_points = fill_outer_points(&extreme_red_tiles, &edges);

    let mut corners: Vec<(i64, i64)> = Vec::new();

    // We're still only allowed to build rectangles from the original list of corners, so we shouldn't
    // use filled_outer_points here. filled_outer_points is only used to check if all corners
    // of a rectangle are inside the edge.
    for (x, (min, max)) in extreme_red_tiles.iter() {
        corners.push((*x, *max));
        corners.push((*x, *min));
    }

    let mut largest_area = 0;

    let corner_count = corners.len();

    for i in 0..corner_count {
        for j in i+1..corner_count {
            let upper_point = corners.get(i).unwrap();
            let lower_point = corners.get(j).unwrap();

            if rectangle_is_invalid(
                normalize_rectangle((
                    (lower_point.0, lower_point.1),
                    (upper_point.0, upper_point.1),
                )),
                &edges,
                &edge_outer_points,
            ) {
                // No need to calculate this rectangle because it is not entirely inside the edge
                continue;
            }
            let x_range = (lower_point.0 - upper_point.0).abs() + 1;
            let y_range = (upper_point.1 - lower_point.1).abs() + 1;
            let area = x_range * y_range;
            largest_area = largest_area.max(area);
        }
    }

    Some(largest_area as u64)
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

fn parse_edges(input: &str) -> Vec<Edge> {
    let mut result = Vec::with_capacity(input.lines().count());
    let mut lines = input.lines();

    let (x, y) = lines
        .next()
        .unwrap()
        .split_once(',')
        .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
        .unwrap();

    let mut previous = (x, y);
    let last = (x, y);

    for line in lines {
        let next = line
            .split_once(',')
            .map(|(x, y)| (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap()))
            .unwrap();

        result.push(Edge::from_points(previous, next));

        previous = next;
    }

    result.push(Edge::from_points(previous, last));

    result
}

fn fill_outer_points(
    outer_points: &BTreeMap<i64, (i64, i64)>,
    edges: &[Edge],
) -> BTreeMap<i64, (i64, i64)> {
    let max_x = outer_points.keys().max().unwrap();
    let min_x = outer_points.keys().min().unwrap();
    let mut result = outer_points.clone();

    (*min_x..=*max_x)
        .for_each(|x| {
            let mut this_min_y = i64::MAX;
            let mut this_max_y = 0;

            // Using only horizontal edges is valid, because a vertical edge is always
            // between two horizontal edges which will cover its y values.
            edges
                .iter()
                .filter(|edge| !edge.vertical && edge.start <= x && edge.end >= x)
                .for_each(|edge| {
                    this_min_y = this_min_y.min(edge.line);
                    this_max_y = this_max_y.max(edge.line);
                });

            if this_min_y != i64::MAX && this_max_y != 0 {
                result.insert(x, (this_min_y, this_max_y));
                // panic!("Expected to find a more sensible min Y for x = {}", x);
            }
        });

    result
}

fn rectangle_is_invalid(
    rectangle: ((i64, i64), (i64, i64)),
    edges: &[Edge],
    outer_points: &BTreeMap<i64, (i64, i64)>,
) -> bool {
    if any_corner_outside_boundary(&rectangle, outer_points) {
        return true;
    }
    edges.iter().any(|edge| edge.is_inside_rectangle(rectangle))
}

fn any_corner_outside_boundary(
    rectangle: &((i64, i64), (i64, i64)),
    outer_points: &BTreeMap<i64, (i64, i64)>,
) -> bool {
    // `Edge::is_inside_rectangle` can't catch an invalid case if the rectangle is entirely
    // outside the edge. So, if any of the corners of the rectangle is outside the boundary,
    // we can immediately mark it as invalid.
    let ((min_x, min_y), (max_x, max_y)) = rectangle;

    for x in *min_x..=*max_x {
        let (lower_limit, higher_limit) = outer_points.get(&x).unwrap();

        if min_y < lower_limit || max_y > higher_limit {
            return true;
        }
    }
    false
}

fn normalize_rectangle(rectangle: ((i64, i64), (i64, i64))) -> ((i64, i64), (i64, i64)) {
    (
        (
            rectangle.0.0.min(rectangle.1.0), // min X
            rectangle.0.1.min(rectangle.1.1), // min Y
        ),
        (
            rectangle.0.0.max(rectangle.1.0), // max X
            rectangle.0.1.max(rectangle.1.1), // max Y
        ),
    )
}

#[derive(Debug)]
struct Edge {
    start: i64,
    end: i64,
    line: i64,
    vertical: bool,
}

impl Edge {
    fn from_points(a: (i64, i64), b: (i64, i64)) -> Edge {
        if a.0 == b.0 {
            Edge {
                start: a.1.min(b.1),
                end: a.1.max(b.1),
                line: a.0,
                vertical: true,
            }
        } else {
            Edge {
                start: a.0.min(b.0),
                end: a.0.max(b.0),
                line: a.1,
                vertical: false,
            }
        }
    }

    pub fn is_inside_rectangle(&self, rectangle: ((i64, i64), (i64, i64))) -> bool {
        let ((min_x, min_y), (max_x, max_y)) = rectangle;

        if self.vertical {
            if self.line <= min_x || self.line >= max_x {
                // Outside horizontal scope
                return false;
            }

            if self.start >= max_y || self.end <= min_y {
                // Outside rectangle
                return false;
            }

            true
        } else {
            if self.line <= min_y || self.line >= max_y {
                // Outside vertical scope
                return false;
            }

            if self.start >= max_x || self.end <= min_x {
                // Outside rectangle
                return false;
            }

            true
        }
    }
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
