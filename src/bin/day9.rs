use std::{collections::HashSet, iter::repeat};

type Point = (i16, i16);

fn main() {
    let input = include_str!("day9.txt");
    println!("Part 1: {}", unique_tail_positions(input, 1));
    println!("Part 2: {}", unique_tail_positions(input, 9));
}

fn unique_tail_positions(input: &str, tail_length: usize) -> usize {
    let path = tail_path(input, tail_length);
    path.into_iter().collect::<HashSet<_>>().len()
}

fn tail_path(input: &str, tail_length: usize) -> Vec<Point> {
    let mut directions = parse_head_directions(input);
    // Give the head a head start ;)
    let mut head = directions.next().unwrap();
    let mut tail = vec![(0, 0); tail_length];
    let mut last_tail_segment_path = vec![(0, 0)];
    for step in directions {
        head.0 += step.0;
        head.1 += step.1;

        // In order, move each segment of the tail
        for segment in 0..tail_length {
            let next_segment = if segment == 0 {
                head
            } else {
                tail[segment - 1]
            };
            let tail_segment = &mut tail[segment];

            let new_position = move_tail(*tail_segment, next_segment);

            // Track the path of the last tail segment
            if segment == tail_length - 1 {
                last_tail_segment_path.push(new_position);
            }
            *tail_segment = new_position;
        }
    }
    last_tail_segment_path
}

fn parse_head_directions<'a>(input: &'a str) -> impl Iterator<Item = (i16, i16)> + 'a {
    input.lines().flat_map(|line| {
        let mut chars = line.chars();
        let direction = chars.next().unwrap();
        chars.next();
        let distance = chars.collect::<String>().parse::<usize>().unwrap();

        let step = match direction {
            'R' => (1, 0),
            'L' => (-1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        repeat(step).take(distance)
    })
}

fn move_tail(tail: Point, head: Point) -> Point {
    let x_diff = tail.0.abs_diff(head.0);
    let y_diff = tail.1.abs_diff(head.1);
    let steps_away = x_diff + y_diff;

    let step = if x_diff == 2 && y_diff == 0 {
        // Same row but 2 steps away, move horizontally
        if head.0 > tail.0 {
            (1, 0)
        } else {
            (-1, 0)
        }
    } else if x_diff == 0 && y_diff == 2 {
        // Same column but 2 steps away, move vertically
        if head.1 > tail.1 {
            (0, 1)
        } else {
            (0, -1)
        }
    } else if steps_away > 2 && x_diff > 0 && y_diff > 0 {
        // More than 2 steps away, move diagonally
        let move_x = if head.0 > tail.0 { 1 } else { -1 };
        let move_y = if head.1 > tail.1 { 1 } else { -1 };
        (move_x, move_y)
    } else {
        // Stay put
        (0, 0)
    };
    (tail.0 + step.0, tail.1 + step.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn example_part1() {
        assert_eq!(unique_tail_positions(EXAMPLE, 1), 13);
    }

    #[test]
    fn example_part2() {
        assert_eq!(unique_tail_positions(EXAMPLE, 9), 1);
    }

    #[test]
    fn larger_example_part2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(unique_tail_positions(input, 9), 36);
    }
}
