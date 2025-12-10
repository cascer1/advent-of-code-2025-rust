advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let (presence, width, height) = parse_input(input);
    let mut accessible_count = 0;

    // Calculate neighbor count per position
    for y in 0..height {
        for x in 0..width {
            let coordinates = y * width + x;
            if presence[coordinates] == 0 {
                continue;
            }
            let mut neighbor_count: u8 = 0;
            neighbors(x, y, width, height, |nx, ny| {
                neighbor_count += presence[ny * width + nx];
            });
            if neighbor_count < 4 {
                accessible_count += 1;
            }
        }
    }

    Some(accessible_count)
}


fn part_two(input: &str) -> Option<u64> {
    let (mut presence, width, height) = parse_input(input);

    // Calculate neighbor count per position
    let mut neighbor_counts = vec![0u8; width * height];
    let mut removal_queue: Vec<(usize, usize)> = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            let coordinates = y * width + x;
            if presence[coordinates] == 0 {
                continue;
            }
            let mut neighbor_count: u8 = 0;
            neighbors(x, y, width, height, |nx, ny| {
                neighbor_count += presence[ny * width + nx];
            });
            neighbor_counts[coordinates] = neighbor_count;
            if neighbor_count < 4 {
                removal_queue.push((x, y));
            }
        }
    }

    let mut head = 0;
    let mut removed_count: u64 = 0;

    while head < removal_queue.len() {
        let (x, y) = removal_queue[head];
        head += 1;
        let coordinates = y * width + x;
        if presence[coordinates] == 0 {
            continue;
        }

        presence[coordinates] = 0; // remove
        removed_count += 1;

        // Update neighbor counts for rolls adjacent to the one we just removed
        neighbors(x, y, width, height, |nx, ny| {
            let neighbor_coords = ny * width + nx;
            if presence[neighbor_coords] == 1 {
                if neighbor_counts[neighbor_coords] > 0 {
                    neighbor_counts[neighbor_coords] -= 1;
                }
                if neighbor_counts[neighbor_coords] < 4 {
                    // If the neighbor now has fewer than 4 neighbors, we can remove it as well
                    removal_queue.push((nx, ny));
                }
            }
        });
    }

    Some(removed_count)
}

const NEIGHBOR_OFFSETS: [(isize, isize); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1, 0),           (1, 0),
    (-1, 1),  (0, 1),  (1, 1),
];

#[inline]
#[allow(clippy::many_single_char_names)]
fn neighbors(x: usize, y: usize, w: usize, h: usize, mut f: impl FnMut(usize, usize)) {
    let xi = x as isize;
    let yi = y as isize;
    for (dx, dy) in NEIGHBOR_OFFSETS {
        let nx = xi + dx;
        let ny = yi + dy;
        if 0 <= nx && nx < w as isize && 0 <= ny && ny < h as isize {
            f(nx as usize, ny as usize);
        }
    }
}

fn parse_input(input: &str) -> (Vec<u8>, usize, usize) {
    let s = input.trim_end_matches(['\n', '\r']); // probably not necessary
    let mut lines = s.lines();
    let first = lines.next().expect("first line cannot be missing");
    let width = first.len();
    let height = 1 + lines.clone().count();

    let mut v = Vec::with_capacity(width * height);// We know the exact size, so we can preallocate

    v.extend(first.bytes().map(|b| (b == b'@') as u8));
    for line in lines {
        debug_assert_eq!(line.len(), width);
        v.extend(line.bytes().map(|b| (b == b'@') as u8));
    }
    (v, width, height)
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
