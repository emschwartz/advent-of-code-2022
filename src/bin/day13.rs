use std::cmp::Ordering;

const DIVIDER_PACKETS: [&str; 2] = ["[[2]]", "[[6]]"];

fn main() {
    let input = include_str!("day13.txt");
    println!(
        "Part 1: {:?}",
        packets_in_right_order(input).iter().sum::<usize>()
    );
    println!("Part 2: {}", divider_packets_indices_product(input));
}

fn packets_in_right_order<'a>(input: &'a str) -> Vec<usize> {
    let mut packets_in_order = Vec::new();

    let mut first = None;
    for (index, line) in input.lines().filter(|line| !line.is_empty()).enumerate() {
        if let Some(first) = first.take() {
            if compare_packets(first, line) {
                println!("Right order");
                packets_in_order.push(index / 2 + 1);
            } else {
                println!("Wrong order");
            }
        } else {
            first = Some(line);
            continue;
        }
    }

    packets_in_order
}

fn divider_packets_indices_product(input: &str) -> usize {
    let mut lines: Vec<&str> = input
        .lines()
        .filter(|line| !line.is_empty())
        .chain(DIVIDER_PACKETS.into_iter())
        .collect();
    // We need to use unstable sorting because we're not checking if packets are "equal"
    lines.sort_unstable_by(|a, b| {
        if compare_packets(a, b) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    for line in lines.iter() {
        println!("{}", line);
    }
    let mut lines = lines.into_iter();
    // Find the divider packets (with 1-based indexing)
    let first_divider = 1 + lines.position(|line| line == DIVIDER_PACKETS[0]).unwrap();
    let second_divider =
        1 + first_divider + lines.position(|line| line == DIVIDER_PACKETS[1]).unwrap();
    first_divider * second_divider
}

fn compare_packets(left: &str, right: &str) -> bool {
    dbg!(&left);
    dbg!(&right);
    let mut left = left.chars().peekable();
    let mut right = right.chars().peekable();

    // When comparing a list to a number, we turn the number into a list
    // and put it back "onto the stack". We could implement this behavior
    // by collecting the strs into a VecDeque<char> but then we'd need to
    // copy the whole string ;)
    let mut left_put_back = Vec::with_capacity(3);
    let mut right_put_back = Vec::with_capacity(3);

    loop {
        let a = left_put_back.pop().or_else(|| left.next());
        let b = right_put_back.pop().or_else(|| right.next());
        let a_next = left_put_back.last().or(left.peek()).copied();
        let b_next = right_put_back.last().or(right.peek()).copied();
        println!("{a:?} {b:?}");
        match (a, b) {
            // If both are empty, they're in the right order
            (None, None) => return true,
            // Left list runs out first
            (None, Some(_)) => return true,
            (Some(']'), Some(b)) if b != ']' => return true,
            // Right runs out first
            (Some(_), None) => return false,
            (Some(a), Some(']')) if a != ']' => return false,
            // Handle if both are numbers
            (Some(a), Some(b)) if a.is_numeric() && b.is_numeric() => {
                // Check if either number is a 10
                if a == '1' || b == '1' {
                    match (a_next.unwrap(), b_next.unwrap()) {
                        // Both are 10s
                        ('0', '0') => {
                            left_put_back.pop().or_else(|| left.next());
                            right_put_back.pop().or_else(|| right.next());
                            continue;
                        }
                        // Left should not be bigger
                        ('0', _) => return false,
                        // Left is smaller, right order
                        (_, '0') => return true,
                        _ => {}
                    }
                }

                if a < b {
                    return true;
                } else if a > b {
                    return false;
                }
            }
            // If one is a number but the other is a list,
            // turn the number into a list of one element and compare them
            (Some(a), Some('[')) if a.is_numeric() => {
                left_put_back.push(']');
                if a_next == Some('0') {
                    left_put_back.push(left.next().unwrap());
                }
                left_put_back.push(a);
                left_put_back.push('[');
                right_put_back.push('[');
            }
            (Some('['), Some(b)) if b.is_numeric() => {
                right_put_back.push(']');
                if b_next == Some('0') {
                    right_put_back.push(right.next().unwrap());
                }
                right_put_back.push(b);
                right_put_back.push('[');
                left_put_back.push('[');
            }
            // If both are the same, continue
            (Some(a), Some(b)) if a == b => continue,
            (Some(a), Some(b)) => unimplemented!("{} {}", a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn example_part1() {
        assert_eq!(packets_in_right_order(EXAMPLE), &[1, 2, 4, 6]);
    }

    #[test]
    fn example_part2() {
        assert_eq!(divider_packets_indices_product(EXAMPLE), 140);
    }
}
