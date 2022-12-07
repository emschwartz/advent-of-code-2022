pub fn main() {
    let input = include_str!("./day2.txt");

    println!("Part 1: {}", score_part1(input));
    println!("Part 2: {}", score_part2(input));
}

fn score_part1(games: &str) -> u32 {
    games
        .lines()
        .map(|game| {
            let mut line = game.chars();
            let them = Move::try_from(line.next().unwrap()).unwrap();
            let us = Move::try_from(line.nth(1).unwrap()).unwrap();
            Outcome::from_game(us, them) as u32 + us as u32
        })
        .sum()
}

fn score_part2(games: &str) -> u32 {
    games
        .lines()
        .map(|game| {
            let mut line = game.chars();
            let them = Move::try_from(line.next().unwrap()).unwrap();
            let outcome = Outcome::try_from(line.nth(1).unwrap()).unwrap();
            let us = Move::from_their_move_and_outcome(them, outcome);
            outcome as u32 + us as u32
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(i32)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(PartialEq, Clone, Copy)]
#[repr(i32)]
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Outcome {
    fn from_game(us: Move, them: Move) -> Self {
        if us == them {
            Outcome::Draw
        } else if us.beats() == them {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(()),
        }
    }
}

impl Move {
    fn beats(&self) -> Move {
        use Move::*;

        match self {
            Rock => Scissors,
            Paper => Rock,
            Scissors => Paper,
        }
    }

    fn loses_to(&self) -> Move {
        use Move::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }

    fn from_their_move_and_outcome(them: Move, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Lose => them.beats(),
            Outcome::Draw => them,
            Outcome::Win => them.loses_to(),
        }
    }
}

impl TryFrom<char> for Move {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let example = "A Y
B X
C Z";
        let total = score_part1(example);
        assert_eq!(total, 15);
    }

    #[test]
    fn part2() {
        let example = "A Y
B X
C Z";
        let total = score_part2(example);
        assert_eq!(total, 12);
    }
}
