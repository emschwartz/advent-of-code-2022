use std::collections::VecDeque;
use std::fmt::Write;

fn main() {
    let input = include_str!("day14.txt");
    println!("Part 1: {}", count_sand(input));
}

#[derive(Clone, Copy, PartialEq)]
enum Contents {
    Air,
    Rock,
    Sand,
}

impl Contents {
    fn to_char(&self) -> char {
        match self {
            Contents::Air => '.',
            Contents::Rock => '#',
            Contents::Sand => 'o',
        }
    }
}

struct Map {
    rows: Vec<Vec<Contents>>,
    start_col: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        // Find the maximum coordinates to determine the size of the map
        let coordinates = input
            .lines()
            .flat_map(|line| line.split(&[' ', '>', '-']))
            .filter_map(|coordinates| coordinates.split_once(','))
            .map(|(col, row)| (col.parse::<usize>().unwrap(), row.parse::<usize>().unwrap()));
        let mut min_col = usize::MAX;
        let mut max_col = 0;
        let mut max_row = 0;
        for (col, row) in coordinates {
            min_col = min_col.min(col);
            max_col = max_col.max(col);
            max_row = max_row.max(row);
        }
        let width = max_col - min_col + 1;

        // Populate the map with rocks
        let mut rows = vec![vec![Contents::Air; width]; max_row + 1];

        for line in input.lines().filter(|line| !line.is_empty()) {
            let mut coordinates = line
                .split(&[' ', '-', '>'])
                .filter_map(|coordinates| coordinates.split_once(','))
                .map(|(col, row)| (col.parse::<usize>().unwrap(), row.parse::<usize>().unwrap()));
            let (mut from_col, mut from_row) = coordinates.next().unwrap();
            for (to_col, to_row) in coordinates {
                for col in from_col.min(to_col)..=to_col.max(from_col) {
                    for row in from_row.min(to_row)..=to_row.max(to_row) {
                        rows[row][col - min_col] = Contents::Rock;
                    }
                }
                from_col = to_col;
                from_row = to_row;
            }
        }
        Self {
            start_col: min_col,
            rows,
        }
    }

    fn to_string(&self) -> String {
        let mut output = String::new();
        for row in self.rows.iter() {
            for point in row {
                write!(&mut output, "{}", point.to_char());
            }
            writeln!(&mut output);
        }
        output.pop();
        output
    }

    fn set_point(&mut self, row: usize, col: usize, value: Contents) {
        self.rows[row][col - self.start_col] = value;
    }

    fn get_point(&self, row: usize, col: usize) -> Option<Contents> {
        if row >= self.rows.len()
            || col < self.start_col
            || col - self.start_col >= self.rows[0].len()
        {
            None
        } else {
            Some(self.rows[row][col - self.start_col])
        }
    }
}

fn add_sand(map: &mut Map) -> bool {
    let mut row = 0;
    let mut col = 500;

    loop {
        let below = map.get_point(row + 1, col);
        let diagonal_left = map.get_point(row + 1, col - 1);
        let diagonal_right = map.get_point(row + 1, col + 1);

        if below == Some(Contents::Air) {
            row += 1;
        } else if diagonal_left == Some(Contents::Air) {
            row += 1;
            col -= 1;
        } else if diagonal_right == Some(Contents::Air) {
            row += 1;
            col += 1;
        } else if below == None || diagonal_left == None || diagonal_right == None {
            // Fell off, can't put any more sand
            dbg!(row, col);
            return false;
        } else {
            break;
        }
    }
    map.set_point(row, col, Contents::Sand);
    true
}

fn count_sand(input: &str) -> usize {
    let mut map = Map::new(input);
    let mut count = 0;
    while add_sand(&mut map) {
        count += 1;
    }
    for (index, line) in map.to_string().lines().enumerate() {
        println!("{index:3} {line}");
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn map_parsing() {
        let map = Map::new(EXAMPLE);
        println!("{}", map.to_string());
        assert_eq!(
            map.to_string(),
            "..........
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########."
        );
    }

    #[test]
    fn example_part1() {
        assert_eq!(count_sand(EXAMPLE), 24);
    }
}
