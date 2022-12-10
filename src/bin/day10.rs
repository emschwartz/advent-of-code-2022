use std::fmt::Write;
use std::iter::once;

fn main() {
    let input = include_str!("./day10.txt");
    let strengths: Vec<_> = signal_strengths(input).collect();
    println!(
        "Part 1: {}",
        strengths[19]
            + strengths[59]
            + strengths[99]
            + strengths[139]
            + strengths[179]
            + strengths[219]
    );

    println!("Part 2: \n{}", to_crt_string(input, 40));
}

fn parse_instructions<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    input.lines().flat_map(|line| {
        let add = if let Some(num) = line.strip_prefix("addx ") {
            Some(num.parse().expect("expected int"))
        } else if line.starts_with("noop") {
            None
        } else {
            unimplemented!()
        };
        once(0).chain(add.into_iter())
    })
}

fn get_cycles<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    let mut x = 1;
    once(1).chain(parse_instructions(input).map(move |add| {
        let after = x + add;
        x = after;
        after
    }))
}

fn signal_strengths<'a>(input: &'a str) -> impl Iterator<Item = i32> + 'a {
    get_cycles(input)
        .enumerate()
        .map(|(cycle, x)| (cycle as i32 + 1) * x)
}

fn apply_crt<'a>(input: &'a str, width: usize) -> impl Iterator<Item = char> + 'a {
    get_cycles(input)
        .enumerate()
        .map(move |(cycle, x_value)| {
            let current_pixel = (cycle % width) as i32;
            current_pixel.abs_diff(x_value) < 2
        })
        .map(|b| if b { '#' } else { '.' })
}

fn to_crt_string(input: &str, width: usize) -> String {
    let mut output = String::new();

    for (index, pixel) in apply_crt(input, width).enumerate() {
        if index > 0 && index % 40 == 0 {
            writeln!(&mut output).unwrap();
        }
        write!(&mut output, "{}", pixel).unwrap();
    }

    if output.ends_with('\n') {
        output.pop();
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn small_example() {
        let input = "noop
addx 3
addx -5";

        let cycles: Vec<i32> = get_cycles(input).collect();
        assert_eq!(cycles, [1, 1, 1, 4, 4, -1]);
    }

    #[test]
    fn cycle_values() {
        let cycles: Vec<_> = get_cycles(EXAMPLE).collect();
        assert_eq!(cycles[19], 21);
        assert_eq!(cycles[59], 19);
        assert_eq!(cycles[99], 18);
        assert_eq!(cycles[139], 21);
        assert_eq!(cycles[179], 16);
        assert_eq!(cycles[219], 18);
    }

    #[test]
    fn example_part1() {
        let strengths: Vec<_> = signal_strengths(EXAMPLE).collect();
        assert_eq!(strengths[19], 420);
        assert_eq!(strengths[59], 1140);
        assert_eq!(strengths[99], 1800);
        assert_eq!(strengths[139], 2940);
        assert_eq!(strengths[179], 2880);
        assert_eq!(strengths[219], 3960);
    }

    #[test]
    fn example_part2() {
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

        println!("{}", to_crt_string(EXAMPLE, 40));

        assert_eq!(to_crt_string(EXAMPLE, 40), expected);
    }
}
