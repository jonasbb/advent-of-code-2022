//! # Day 4: Camp Cleanup
//!
//! ## --- Part One ---
//!
//! Space needs to be cleared before the last supplies can be unloaded from the ships, and so several Elves have been assigned the job of cleaning up sections of the camp.
//! Every section has a unique _ID number_, and each Elf is assigned a range of section IDs.
//!
//! However, as some of the Elves compare their section assignments with each other, they've noticed that many of the assignments _overlap_.
//! To try to quickly find overlaps and reduce duplicated effort, the Elves pair up and make a _big list of the section assignments for each pair_ (your puzzle input).
//!
//! For example, consider the following list of section assignment pairs:
//!
//! ```text
//! 2-4,6-8
//! 2-3,4-5
//! 5-7,7-9
//! 2-8,3-7
//! 6-6,4-6
//! 2-6,4-8
//! ```
//!
//! For the first few pairs, this list means:
//!
//! * Within the first pair of Elves, the first Elf was assigned sections `2-4` (sections `2`, `3`, and `4`), while the second Elf was assigned sections `6-8` (sections `6`, `7`, `8`).
//! * The Elves in the second pair were each assigned two sections.
//! * The Elves in the third pair were each assigned three sections: one got sections `5`, `6`, and `7`, while the other also got `7`, plus `8` and `9`.
//!
//! This example list uses single-digit section IDs to make it easier to draw; your actual list might contain larger numbers.
//! Visually, these pairs of section assignments look like this:
//!
//! ```text
//! .234.....  2-4
//! .....678.  6-8
//!
//! .23......  2-3
//! ...45....  4-5
//!
//! ....567..  5-7
//! ......789  7-9
//!
//! .2345678.  2-8
//! ..34567..  3-7
//!
//! .....6...  6-6
//! ...456...  4-6
//!
//! .23456...  2-6
//! ...45678.  4-8
//! ```
//!
//! Some of the pairs have noticed that one of their assignments _fully contains_ the other.
//! For example, `2-8` fully contains `3-7`, and `6-6` is fully contained by `4-6`.
//! In pairs where one assignment fully contains the other, one Elf in the pair would be exclusively cleaning sections their partner will already be cleaning, so these seem like the most in need of reconsideration.
//! In this example, there are _`2`_ such pairs.
//!
//! _In how many assignment pairs does one range fully contain the other?_
//!
//! ## --- Part Two ---
//!
//! It seems like there is still quite a bit of duplicate work planned.
//! Instead, the Elves would like to know the number of pairs that _overlap at all_.
//!
//! In the above example, the first two pairs (`2-4,6-8` and `2-3,4-5`) don't overlap, while the remaining four pairs (`5-7,7-9`, `2-8,3-7`, `6-6,4-6`, and `2-6,4-8`) do overlap:
//!
//! * `5-7,7-9` overlaps in a single section, `7`.
//! * `2-8,3-7` overlaps all of the sections `3` through `7`.
//! * `6-6,4-6` overlaps in a single section, `6`.
//! * `2-6,4-8` overlaps in sections `4`, `5`, and `6`.
//!
//! So, in this example, the number of overlapping assignment pairs is _`4`_.
//!
//! _In how many assignment pairs do the ranges overlap?_

use crate::prelude::*;

#[derive(Inpt, Debug)]
#[inpt(regex = r"(\d+)-(\d+),(\d+)-(\d+)")]
struct AssignmentPair {
    first_from: usize,
    first_to: usize,
    second_from: usize,
    second_to: usize,
}

// #[aoc_runner_derive::aoc_generator(day4)]
// fn input_generator(input: &str) -> Vec<Vec<u32>> {
//     input
//         .split("\n\n")
//         .map(|segment| {
//             segment
//                 .split('\n')
//                 .map(|line| line.parse().unwrap())
//                 .collect()
//         })
//         .collect()
// }

#[aoc_runner_derive::aoc(day4, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| inpt::<AssignmentPair>(line).unwrap())
        .filter(|ap| {
            let r0 = ap.first_from..=ap.first_to;
            let r1 = ap.second_from..=ap.second_to;
            // Full overlap, i.e., one covers the whole other
            (r0.contains(&ap.second_from) && r0.contains(&ap.second_to))
                || (r1.contains(&ap.first_from) && r1.contains(&ap.first_to))
        })
        .count() as u32
}

#[aoc_runner_derive::aoc(day4, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| inpt::<AssignmentPair>(line).unwrap())
        .filter(|ap| {
            let r0 = ap.first_from..=ap.first_to;
            let r1 = ap.second_from..=ap.second_to;
            // Any overlap
            r0.contains(&ap.second_from)
                || r0.contains(&ap.second_to)
                || r1.contains(&ap.first_from)
                || r1.contains(&ap.first_to)
        })
        .count() as u32
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

#[test]
fn test_part1() -> Result<()> {
    let values = TEST_INPUT_1;
    assert_eq!(2, part1(values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = include_str!("../input/2022/day4.txt").trim();
    assert_eq!(471, part1(values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = TEST_INPUT_1;
    assert_eq!(4, part2(values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = include_str!("../input/2022/day4.txt").trim();
    assert_eq!(888, part2(values));
    Ok(())
}
