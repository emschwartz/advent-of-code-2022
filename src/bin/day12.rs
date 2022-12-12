use std::collections::VecDeque;

fn main() {
    let input = include_str!("day12.txt");
    println!("Part 1: {}", shortest_path(input, false));
    println!("Part 2: {}", shortest_path(input, true));
}

type Point = (usize, usize);

struct Map<'a> {
    heights: Vec<&'a [u8]>,
    steps_to_destination: Vec<Vec<Option<usize>>>,
    starting_points: Vec<Point>,
    end: Point,
}

impl<'a> Map<'a> {
    fn new(input: &'a str, start_at_any_a: bool) -> Self {
        let mut heights = Vec::new();
        let mut starting_points = Vec::new();
        let mut end = (0, 0);

        for (row, line) in input.lines().enumerate() {
            if let Some(col) = line.find('E') {
                end = (row, col);
            }
            if start_at_any_a {
                starting_points.extend(line.char_indices().filter_map(|(col, c)| {
                    if c == 'S' || c == 'a' {
                        Some((row, col))
                    } else {
                        None
                    }
                }));
            } else if let Some(col) = line.find('S') {
                starting_points.push((row, col));
            }

            heights.push(line.as_bytes());
        }
        let mut steps_to_destination = vec![vec![None; heights[0].len()]; heights.len()];
        steps_to_destination[end.0][end.1] = Some(0);
        Self {
            heights,
            starting_points,
            end,
            steps_to_destination,
        }
    }

    fn get_height(&self, point: Point) -> u8 {
        if point == self.end {
            b'z' - b'a'
        } else if self.starting_points.contains(&point) {
            0
        } else {
            self.heights[point.0][point.1] - b'a'
        }
    }

    fn set_steps_to_destination(&mut self, point: Point, steps_to_destination: usize) -> bool {
        // Check if the previous point to this path was better
        if let Some(previous) = self.steps_to_destination[point.0][point.1] {
            if previous < steps_to_destination {
                return false;
            }
        }

        self.steps_to_destination[point.0][point.1] = Some(steps_to_destination);
        true
    }

    fn get_steps_to_destination(&self, point: Point) -> Option<usize> {
        self.steps_to_destination[point.0][point.1]
    }

    fn get_adjacent_points_to_explore(&self, point: Point) -> Vec<Point> {
        let mut points = Vec::new();
        if let Some(row) = point.0.checked_sub(1) {
            points.push((row, point.1));
        }
        if point.0 + 1 < self.heights.len() {
            points.push((point.0 + 1, point.1));
        }
        if let Some(col) = point.1.checked_sub(1) {
            points.push((point.0, col));
        }
        if point.1 + 1 < self.heights[0].len() {
            points.push((point.0, point.1 + 1));
        }

        points.retain(|p| {
            self.get_height(*p) + 1 >= self.get_height(point)
                && self.get_steps_to_destination(*p).is_none()
        });
        points
    }
}

fn shortest_path(input: &str, start_at_any_a: bool) -> usize {
    let mut map = Map::new(input, start_at_any_a);

    // Start exploring from the end
    let mut points_to_explore = VecDeque::from([(map.get_adjacent_points_to_explore(map.end), 1)]);

    while let Some((points, steps_to_destination)) = points_to_explore.pop_front() {
        for point in points {
            if map.get_steps_to_destination(point).is_some() {
                continue;
            }
            if map.set_steps_to_destination(point, steps_to_destination) {
                let points = map.get_adjacent_points_to_explore(point);
                if !points.is_empty() {
                    points_to_explore.push_back((points, steps_to_destination + 1));
                }
            }
        }
    }

    // Find the starting point that had the shortest path to the end
    map.starting_points
        .iter()
        .filter_map(|point| map.get_steps_to_destination(*point))
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::shortest_path;

    static EXAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn example_part1() {
        assert_eq!(shortest_path(EXAMPLE, false), 31);
    }

    #[test]
    fn example_part2() {
        assert_eq!(shortest_path(EXAMPLE, true), 29);
    }
}
