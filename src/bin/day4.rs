use std::str::FromStr;

fn main() {
    let input = include_str!("./day4.txt");
    println!("Part 1: {}", pairs_that_contain_others(input));
    println!("Part 2: {}", pairs_that_overlap(input));
}

fn pairs_that_contain_others(input: &str) -> usize {
    input
        .lines()
        .map(|line| Pair::from_str(line).unwrap())
        .filter(|pair| pair.first.contains(&pair.second) || pair.second.contains(&pair.first))
        .count()
}

fn pairs_that_overlap(input: &str) -> usize {
    input
        .lines()
        .map(|line| Pair::from_str(line).unwrap())
        .filter(|pair| pair.first.overlaps(&pair.second))
        .count()
}

struct Range {
    from: u32,
    to: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.from <= other.from && self.to >= other.to
    }

    fn overlaps(&self, other: &Range) -> bool {
        self.from <= other.to && self.to >= other.from
    }
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_at(s.find('-').unwrap());
        let from = from.parse().unwrap();
        let to = to[1..].parse().unwrap();
        Ok(Range { from, to })
    }
}

struct Pair {
    first: Range,
    second: Range,
}

impl FromStr for Pair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first, second) = s.split_at(s.find(',').ok_or(())?);

        Ok(Pair {
            first: Range::from_str(first)?,
            second: Range::from_str(&second[1..])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        assert_eq!(pairs_that_contain_others(input), 2);
    }
}
