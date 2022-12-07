use std::collections::HashSet;

fn main() {
    let input = include_str!("./day3.txt");
    println!("Part 1: {}", total_priority(input));
    println!("Part 2: {}", total_group_priority(input));
}

fn total_priority(backpacks: &str) -> u32 {
    backpacks
        .lines()
        .map(|backpack| {
            let compartment_size = backpack.len() / 2;
            let (left, right) = backpack.split_at(compartment_size);
            let left: HashSet<char> = left.chars().collect();
            let shared_item = right.chars().find(|c| left.contains(c)).unwrap();
            priority(shared_item)
        })
        .sum()
}

fn total_group_priority(backpacks: &str) -> u32 {
    group_elves(backpacks)
        .into_iter()
        .map(priority)
        .sum::<u32>()
}

fn group_elves(backpacks: &str) -> Vec<char> {
    let mut backpacks = backpacks.lines();

    let mut groups = Vec::new();

    while let Some(first) = backpacks.next() {
        let first = unique_letters(first);
        let second = unique_letters(backpacks.next().unwrap());
        let third = unique_letters(backpacks.next().unwrap());
        let shared_items = first
            .intersection(&second)
            .copied()
            .collect::<HashSet<char>>();
        let badge = shared_items.intersection(&third).next().unwrap();
        groups.push(*badge);
    }

    groups
}

fn unique_letters(backpack: &str) -> HashSet<char> {
    backpack.chars().collect()
}

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        27 + c as u32 - 'A' as u32
    } else {
        1 + c as u32 - 'a' as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn priorities() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
        assert_eq!(priority('Z'), 52);
    }

    #[test]
    fn part1_example() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(total_priority(example), 157);
    }

    #[test]
    fn part2_example() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        assert_eq!(total_group_priority(example), 70);
    }
}
