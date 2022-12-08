use std::collections::VecDeque;

fn main() {
    let input = include_str!("day5.txt");
    println!("Part 1: {}", apply_instructions(input, true));
    println!("Part 2: {}", apply_instructions(input, false));
}

/// The front represents the bottom of the stack
type Stack = VecDeque<char>;

fn parse_starting_position(input: &str) -> Vec<Stack> {
    input
        .lines()
        .take_while(|line| !line.trim().starts_with('1'))
        .fold(Vec::new(), |mut stacks, line| {
            let mut chars = line.chars().enumerate().skip(1);
            while let Some((col, crate_id)) = chars.next() {
                if crate_id.is_alphabetic() {
                    let stack_num = (col - 1) / 4;
                    if stack_num >= stacks.len() {
                        stacks.extend((stacks.len()..=stack_num).map(|_| Default::default()));
                    }
                    stacks[stack_num].push_front(crate_id);
                }
            }
            stacks
        })
}

fn parse_instructions(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .lines()
        .skip_while(|line| !line.trim().starts_with('1'))
        .skip(2)
        .map(|line| {
            let mut words = line.split(' ');
            words.next();
            let quantity = words.next().unwrap().parse::<usize>().unwrap();
            words.next();
            let from = words.next().unwrap().parse::<usize>().unwrap();
            words.next();
            let to = words.next().unwrap().parse::<usize>().unwrap();
            (quantity, from - 1, to - 1)
        })
        .collect()
}

fn apply_instructions(input: &str, move_crates_one_at_a_time: bool) -> String {
    let mut stacks = parse_starting_position(input);
    let instructions = parse_instructions(input);

    for (quantity, from, to) in instructions {
        if move_crates_one_at_a_time {
            for _i in 0..quantity {
                if let Some(crate_id) = stacks[from].pop_back() {
                    stacks[to].push_back(crate_id);
                }
            }
        } else {
            let index = stacks[from].len() - quantity;
            let moving_crates = stacks[from].split_off(index);
            stacks[to].extend(moving_crates);
        }
    }

    top_of_each_stack(&stacks)
}

fn top_of_each_stack(stacks: &[Stack]) -> String {
    stacks
        .iter()
        .map(|stack| stack.back().unwrap_or(&' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_starting_position() {
        let input = "[D]
[N] [C]
[Z] [M] [P]
 1   2   3 ";
        let stacks = parse_starting_position(input);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], vec!['Z', 'N', 'D']);
        assert_eq!(stacks[1], vec!['M', 'C']);
        assert_eq!(stacks[2], vec!['P']);

        let input = "        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3";
        let stacks = parse_starting_position(input);
        assert_eq!(stacks.len(), 3);
        assert_eq!(stacks[0], &[]);
        assert_eq!(stacks[1], &['M', 'C']);
        assert_eq!(stacks[2], &['P', 'D', 'N', 'Z'])
    }
}
