//! # Day 2: Rock Paper Scissors
//!
//! ## --- Part One ---
//!
//! The Elves begin to set up camp on the beach.
//! To decide whose tent gets to be closest to the snack storage, a giant [Rock Paper Scissors](https://en.wikipedia.org/wiki/Rock_paper_scissors) tournament is already in progress.
//!
//! Rock Paper Scissors is a game between two players.
//! Each game contains many rounds; in each round, the players each simultaneously choose one of Rock, Paper, or Scissors using a hand shape.
//! Then, a winner for that round is selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
//! If both players choose the same shape, the round instead ends in a draw.
//!
//! Appreciative of your help yesterday, one Elf gives you an _encrypted strategy guide_ (your puzzle input) that they say will be sure to help you win.
//! "The first column is what your opponent is going to play: `A` for Rock, `B` for Paper, and `C` for Scissors.
//! The second column--"
//! Suddenly, the Elf is called away to help with someone's tent.
//!
//! The second column, you reason, must be what you should play in response: `X` for Rock, `Y` for Paper, and `Z` for Scissors.
//! Winning every time would be suspicious, so the responses must have been carefully chosen.
//!
//! The winner of the whole tournament is the player with the highest score.
//! Your _total score_ is the sum of your scores for each round.
//! The score for a single round is the score for the _shape you selected_ (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the _outcome of the round_ (0 if you lost, 3 if the round was a draw, and 6 if you won).
//!
//! Since you can't be sure if the Elf is trying to help you or trick you, you should calculate the score you would get if you were to follow the strategy guide.
//!
//! For example, suppose you were given the following strategy guide:
//!
//! ```text
//! A Y
//! B X
//! C Z
//! ```
//!
//! This strategy guide predicts and recommends the following:
//!
//! * In the first round, your opponent will choose Rock (`A`), and you should choose Paper (`Y`).
//!     This ends in a win for you with a score of _8_ (2 because you chose Paper + 6 because you won).
//! * In the second round, your opponent will choose Paper (`B`), and you should choose Rock (`X`).
//!     This ends in a loss for you with a score of _1_ (1 + 0).
//! * The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = _6_.
//!
//! In this example, if you were to follow the strategy guide, you would get a total score of _`15`_ (8 + 1 + 6).
//!
//! _What would your total score be if everything goes exactly according to your strategy guide?_
//!
//! ## --- Part Two ---
//!
//! The Elf finishes helping with the tent and sneaks back over to you.
//! "Anyway, the second column says how the round needs to end: `X` means you need to lose, `Y` means you need to end the round in a draw, and `Z` means you need to win.
//! Good luck!"
//!
//! The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated.
//! The example above now goes like this:
//!
//! * In the first round, your opponent will choose Rock (`A`), and you need the round to end in a draw (`Y`), so you also choose Rock.
//!     This gives you a score of 1 + 3 = _4_.
//! * In the second round, your opponent will choose Paper (`B`), and you choose Rock so you lose (`X`) with a score of 1 + 0 = _1_.
//! * In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = _7_.
//!
//! Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of _`12`_.
//!
//! Following the Elf's instructions for the second column, _what would your total score be if everything goes exactly according to your strategy guide?_

use crate::prelude::*;

#[derive(Inpt, Debug)]
#[inpt(regex = r"(.) (.)")]
struct MovePart1 {
    #[inpt(from_str)]
    opponent: Shape,
    #[inpt(from_str)]
    mine: Shape,
}

impl MovePart1 {
    fn outcome(&self) -> Outcome {
        match (self.opponent, self.mine) {
            (Shape::Rock, Shape::Rock) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Win,
            (Shape::Rock, Shape::Scissors) => Outcome::Lose,
            (Shape::Paper, Shape::Rock) => Outcome::Lose,
            (Shape::Paper, Shape::Paper) => Outcome::Draw,
            (Shape::Paper, Shape::Scissors) => Outcome::Win,
            (Shape::Scissors, Shape::Rock) => Outcome::Win,
            (Shape::Scissors, Shape::Paper) => Outcome::Lose,
            (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        }
    }

    fn score(&self) -> i32 {
        self.mine.score() + self.outcome().score()
    }
}

#[derive(Debug, Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Shape {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(anyhow!("Invalid shape: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Self::Win => 6,
            Self::Lose => 0,
            Self::Draw => 3,
        }
    }
}

impl FromStr for Outcome {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(anyhow!("Invalid outcome: {}", s)),
        }
    }
}

#[derive(Inpt, Debug)]
#[inpt(regex = r"(.) (.)")]
struct MovePart2 {
    #[inpt(from_str)]
    opponent: Shape,
    #[inpt(from_str)]
    outcome: Outcome,
}

impl MovePart2 {
    fn score(&self) -> i32 {
        self.outcome.score() + self.correct_move().score()
    }

    fn correct_move(&self) -> Shape {
        match self.outcome {
            Outcome::Win => match self.opponent {
                Shape::Rock => Shape::Paper,
                Shape::Paper => Shape::Scissors,
                Shape::Scissors => Shape::Rock,
            },
            Outcome::Lose => match self.opponent {
                Shape::Rock => Shape::Scissors,
                Shape::Paper => Shape::Rock,
                Shape::Scissors => Shape::Paper,
            },
            Outcome::Draw => self.opponent,
        }
    }
}

#[aoc_runner_derive::aoc_generator(day2, part1, Complex)]
fn input_generator_complex(input: &str) -> Vec<MovePart1> {
    input.lines().map(|line| inpt(line).unwrap()).collect()
}

#[aoc_runner_derive::aoc_generator(day2, part2, Complex)]
fn input_generator_part2_complex(input: &str) -> Vec<MovePart2> {
    input.lines().map(|line| inpt(line).unwrap()).collect()
}

#[aoc_runner_derive::aoc(day2, part1, Complex)]
fn part1_complex(input: &[MovePart1]) -> i32 {
    input.iter().map(|m| m.score()).sum()
}

#[allow(clippy::identity_op)]
#[aoc_runner_derive::aoc(day2, part1, Simple)]
fn part1_simple(input: &str) -> i32 {
    input.lines().map(|line| {
        match line {
            // First number for played symbol, second for outcome
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,
            _ => panic!("Invalid input: {}", line),
        }
    }).sum()
}

#[aoc_runner_derive::aoc(day2, part2, Complex)]
fn part2_complex(input: &[MovePart2]) -> i32 {
    input
        .iter()
        .map(|m| m.score())
        .sum()
}

#[allow(clippy::identity_op)]
#[aoc_runner_derive::aoc(day2, part2, Simple)]
fn part2_simple(input: &str) -> i32 {
    input.lines().map(|line| {
        match line {
            // First number for played symbol, second for outcome
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => panic!("Invalid input: {}", line),
        }
    }).sum()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"A Y
B X
C Z";

#[test]
fn test_part1_complex() -> Result<()> {
    let values = input_generator_complex(TEST_INPUT_1);
    assert_eq!(15, part1_complex(&values));
    Ok(())
}

#[test]
fn test_part1_simple() -> Result<()> {
    let values = TEST_INPUT_1;
    assert_eq!(15, part1_simple(values));
    Ok(())
}

#[test]
fn test_part1_solution_complex() -> Result<()> {
    let values = input_generator_complex(include_str!("../input/2022/day2.txt").trim());
    assert_eq!(12679, part1_complex(&values));
    Ok(())
}

#[test]
fn test_part1_solution_simple() -> Result<()> {
    let values = include_str!("../input/2022/day2.txt").trim();
    assert_eq!(12679, part1_simple(values));
    Ok(())
}

#[test]
fn test_part2_complex() -> Result<()> {
    let values = input_generator_part2_complex(TEST_INPUT_1);
    assert_eq!(12, part2_complex(&values));
    Ok(())
}

#[test]
fn test_part2_simple() -> Result<()> {
    let values = TEST_INPUT_1;
    assert_eq!(12, part2_simple(values));
    Ok(())
}

#[test]
fn test_part2_solution_complex() -> Result<()> {
    let values = input_generator_part2_complex(include_str!("../input/2022/day2.txt").trim());
    assert_eq!(14470, part2_complex(&values));
    Ok(())
}

#[test]
fn test_part2_solution_simple() -> Result<()> {
    let values = include_str!("../input/2022/day2.txt").trim();
    assert_eq!(14470, part2_simple(values));
    Ok(())
}
