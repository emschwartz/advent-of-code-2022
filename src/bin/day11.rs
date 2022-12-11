use std::collections::HashMap;
use std::iter::repeat;

fn main() {
    let input = include_str!("day11.txt");
    println!("Part 1: {}", monkey_business(input, 20, true));
    println!("Part 2: {}", monkey_business(input, 10000, false));
}

struct Monkey {
    items: Vec<Worry>,
    items_inspected: usize,
    /// How worry level changes after inspection
    operation: Box<dyn Fn(&mut Worry)>,
    /// Takes in the worry level, determines which monkey to throw to
    test_divisibility: u32,
    test_true: usize,
    test_false: usize,
}

#[derive(Clone, Default)]
struct Worry {
    /// For small numbers (part 1), we can just use the number itself
    number: Option<u32>,
    /// For larger numbers (part 2), we only store the remainders
    /// when the number is divided by the divisor (which is the key)
    remainders: HashMap<u32, u32>,
}

impl Worry {
    fn add(&mut self, rhs: u32) {
        self.number = self.number.and_then(|num| num.checked_add(rhs));
        for (divisor, remainder) in self.remainders.iter_mut() {
            *remainder = (*remainder + rhs) % *divisor;
        }
    }

    fn mul(&mut self, rhs: u32) {
        self.number = self.number.and_then(|num| num.checked_mul(rhs));
        for (divisor, remainder) in self.remainders.iter_mut() {
            *remainder = (*remainder * rhs) % *divisor;
        }
    }

    fn square(&mut self) {
        self.number = self.number.and_then(|num| num.checked_mul(num));
        for (divisor, remainder) in self.remainders.iter_mut() {
            *remainder = remainder.pow(2) % *divisor;
        }
    }

    fn div(&mut self, rhs: u32) {
        if let Some(num) = &mut self.number {
            *num = *num / rhs;
        } else {
            panic!("Number is too large to apply division");
        }
    }

    fn divisible_by(&self, num: u32) -> bool {
        if let Some(number) = self.number {
            number % num == 0
        } else {
            self.remainders[&num] == 0
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut lines = input.lines();
    let mut monkeys = Vec::new();

    // Go through the instructions once to determine all the numbers we'll use
    // for divisibility tests
    let worry = Worry {
        number: Some(0),
        remainders: input
            .lines()
            .filter_map(|line| line.strip_prefix("  Test: divisible by "))
            .map(|num| num.parse().unwrap())
            .zip(repeat(0))
            .collect(),
    };

    while lines.next().is_some() {
        let items = lines
            .next()
            .unwrap()
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<u32>().unwrap())
            .map(|starting_worry| {
                let mut w = worry.clone();
                w.add(starting_worry);
                w
            })
            .collect();
        let operation = lines
            .next()
            .unwrap()
            .strip_prefix("  Operation: new = old ")
            .unwrap();
        let operation: Box<dyn Fn(&mut Worry)> =
            match (operation.chars().next().unwrap(), &operation[2..]) {
                ('*', "old") => Box::new(|old| old.square()),
                ('*', num) => {
                    let num: u32 = num.parse().unwrap();
                    Box::new(move |old| old.mul(num))
                }
                ('+', num) => {
                    let num: u32 = num.parse().unwrap();
                    Box::new(move |old| old.add(num))
                }
                _ => unimplemented!(),
            };
        let test_divisibility: u32 = lines
            .next()
            .unwrap()
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse()
            .unwrap();
        let test_true: usize = lines
            .next()
            .unwrap()
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        let test_false: usize = lines
            .next()
            .unwrap()
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse()
            .unwrap();
        lines.next();
        monkeys.push(Monkey {
            items,
            items_inspected: 0,
            operation,
            test_divisibility,
            test_true,
            test_false,
        })
    }

    monkeys
}

fn monkey_business(input: &str, rounds: usize, relief_after_inspect: bool) -> u64 {
    let mut monkeys = parse_monkeys(input);

    for _i in 0..rounds {
        calculate_round(&mut monkeys, relief_after_inspect);
    }

    monkeys.sort_by_key(|monkey| monkey.items_inspected);
    monkeys
        .iter()
        .map(|m| m.items_inspected as u64)
        .skip(monkeys.len() - 2)
        .product()
}

fn calculate_round(monkeys: &mut Vec<Monkey>, relief_after_inspect: bool) {
    for m in 0..monkeys.len() {
        for mut worry in monkeys[m].items.split_off(0) {
            // Getting worried while they inspect the item
            (monkeys[m].operation)(&mut worry);
            if relief_after_inspect {
                worry.div(3);
            }
            monkeys[m].items_inspected += 1;
            let next_monkey = if worry.divisible_by(monkeys[m].test_divisibility) {
                monkeys[m].test_true
            } else {
                monkeys[m].test_false
            };
            monkeys[next_monkey].items.push(worry);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn example_part1() {
        assert_eq!(monkey_business(EXAMPLE, 20, true), 10605);
    }

    #[test]
    fn example_part2() {
        assert_eq!(monkey_business(EXAMPLE, 20, false), 99 * 103);
        assert_eq!(monkey_business(EXAMPLE, 10000, false), 2713310158);
    }
}
