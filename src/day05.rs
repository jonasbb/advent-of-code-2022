//! # Day 5: Supply Stacks
//!
//! ## --- Part One ---
//!
//! The expedition can depart as soon as the final supplies have been unloaded from the ships.
//! Supplies are stored in stacks of marked _crates_, but because the needed supplies are buried under many other crates, the crates need to be rearranged.
//!
//! The ship has a _giant cargo crane_ capable of moving crates between stacks.
//! To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps.
//! After the crates are rearranged, the desired crates will be at the top of each stack.
//!
//! The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her _which_ crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.
//!
//! They do, however, have a drawing of the starting stacks of crates _and_ the rearrangement procedure (your puzzle input).
//! For example:
//!
//! ```text
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!
//! move 1 from 2 to 1
//! move 3 from 1 to 3
//! move 2 from 2 to 1
//! move 1 from 1 to 2
//! ```
//!
//! In this example, there are three stacks of crates.
//! Stack 1 contains two crates: crate `Z` is on the bottom, and crate `N` is on top.
//! Stack 2 contains three crates; from bottom to top, they are crates `M`, `C`, and `D`.
//! Finally, stack 3 contains a single crate, `P`.
//!
//! Then, the rearrangement procedure is given.
//! In each step of the procedure, a quantity of crates is moved from one stack to a different stack.
//! In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:
//!
//! ```text
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!  ```
//!
//! In the second step, three crates are moved from stack 1 to stack 3.
//! Crates are moved _one at a time_, so the first crate to be moved (`D`) ends up below the second and third crates:
//!
//! ```text
//!         [Z]
//!         [N]
//!     [C] [D]
//!     [M] [P]
//!  1   2   3
//!  ```
//!
//! Then, both crates are moved from stack 2 to stack 1.
//! Again, because crates are moved _one at a time_, crate `C` ends up below crate `M`:
//!
//! ```text
//!         [Z]
//!         [N]
//! [M]     [D]
//! [C]     [P]
//!  1   2   3
//!  ```
//!
//! Finally, one crate is moved from stack 1 to stack 2:
//!
//! ```text
//!         [Z]
//!         [N]
//!         [D]
//! [C] [M] [P]
//!  1   2   3
//!  ```
//!
//! The Elves just need to know _which crate will end up on top of each stack_; in this example, the top crates are `C` in stack 1, `M` in stack 2, and `Z` in stack 3, so you should combine these together and give the Elves the message _`CMZ`_.
//!
//! _After the rearrangement procedure completes, what crate ends up on top of each stack?_
//!
//! ## --- Part Two ---
//!
//! As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.
//!
//! Some mud was covering the writing on the side of the crane, and you quickly wipe it away.
//! The crane isn't a CrateMover 9000 - it's a _<span title="It's way better than the old CrateMover 1006.">CrateMover 9001</span>_.
//!
//! The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and _the ability to pick up and move multiple crates at once_.
//!
//! Again considering the example above, the crates begin in the same configuration:
//!
//! ```text
//!     [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!  ```
//!
//! Moving a single crate from stack 2 to stack 1 behaves the same as before:
//!
//! ```text
//! [D]
//! [N] [C]
//! [Z] [M] [P]
//!  1   2   3
//!  ```
//!
//! However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates _stay in the same order_, resulting in this new configuration:
//!
//! ```text
//!         [D]
//!         [N]
//!     [C] [Z]
//!     [M] [P]
//!  1   2   3
//!  ```
//!
//! Next, as both crates are moved from stack 2 to stack 1, they _retain their order_ as well:
//!
//! ```text
//!         [D]
//!         [N]
//! [C]     [Z]
//! [M]     [P]
//!  1   2   3
//!  ```
//!
//! Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate `C` that gets moved:
//!
//! ```text
//!         [D]
//!         [N]
//!         [Z]
//! [M] [C] [P]
//!  1   2   3
//! ```
//!
//! In this example, the CrateMover 9001 has put the crates in a totally different order: _`MCD`_.
//!
//! Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies.
//! _After the rearrangement procedure completes, what crate ends up on top of each stack?_

use crate::prelude::*;

#[derive(Clone, Debug)]
struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    #[allow(dead_code)]
    fn display(&self) -> String {
        let mut s = String::new();
        for (idx, stack) in self.stacks.iter().enumerate() {
            s += &format!("{:2}: ", idx + 1);
            for c in stack {
                s += &format!("[{}]", c);
            }
        }
        s
    }
}

#[derive(Inpt, Debug)]
#[inpt(regex = r"move (\d+) from (\d+) to (\d+)")]
struct Move {
    amount: usize,
    // Index, 1-based
    from: usize,
    // Index, 1-based
    to: usize,
}

#[aoc_runner_derive::aoc_generator(day5)]
fn input_generator(input: &str) -> (Stacks, Vec<Move>) {
    let (stack_img, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = Vec::new();
    stack_img.lines().for_each(|line| {
        dbg!(&line);
        let mut chars = line.chars().fuse();
        for idx in 0.. {
            let first_char = chars.next();
            if first_char == Some('[') {
                let b = chars.next().unwrap();
                assert_eq!(Some(']'), chars.next());
                // Optional whitespace
                let _ = chars.next();

                let newsize = std::cmp::max(idx + 1, stacks.len());
                stacks.resize_with(newsize, Vec::new);
                stacks[idx].push(b);
            } else if first_char == Some(' ') {
                // Ignore whitespace
                chars.next();
                chars.next();
                chars.next();
            } else {
                break;
            }
        }
    });
    // We parsed the image top down, but we need to invert the stacks to ensure the first parsed entry is on the top.
    stacks.iter_mut().for_each(|s| s.reverse());

    let moves = moves
        .lines()
        .map(|line| inpt::<Move>(line).unwrap())
        .collect();
    (Stacks { stacks }, moves)
}

#[aoc_runner_derive::aoc(day5, part1)]
fn part1((stacks, moves): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();

    for &Move { amount, from, to } in moves {
        // println!("{}", stacks.display());
        // println!("Move {amount} from {from} to {to}");
        for _ in 0..amount {
            let b = stacks.stacks[from - 1].pop().unwrap();
            stacks.stacks[to - 1].push(b);
        }
    }

    stacks.stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

#[aoc_runner_derive::aoc(day5, part2)]
fn part2((stacks, moves): &(Stacks, Vec<Move>)) -> String {
    let mut stacks = stacks.clone();

    for &Move { amount, from, to } in moves {
        // println!("{}", stacks.display());
        // println!("Move {amount} from {from} to {to}");
        let from_stack_len = stacks.stacks[from - 1].len();
        let buffer = stacks.stacks[from - 1]
            .drain(from_stack_len - amount..)
            .collect::<Vec<_>>();
        stacks.stacks[to - 1].extend(buffer);
    }

    stacks.stacks.iter_mut().map(|s| s.pop().unwrap()).collect()
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

#[test]
fn test_part1() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!("CMZ", part1(&values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2022/day5.txt"));
    assert_eq!("ZRLJGSCTR", part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = input_generator(TEST_INPUT_1);
    assert_eq!("MCD", part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = input_generator(include_str!("../input/2022/day5.txt"));
    assert_eq!("", part2(&values));
    Ok(())
}
