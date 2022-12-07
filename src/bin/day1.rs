use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("./day1.txt");
    let mut elves = BinaryHeap::new();
    let mut current_elf = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<u32>().unwrap();
        }
    }
    let most = elves.pop().unwrap();

    println!("Part 1: {}", most);
    println!(
        "Part 2: {}",
        most + elves.pop().unwrap() + elves.pop().unwrap()
    );
}
