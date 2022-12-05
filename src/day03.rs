//! # Day 2: Rucksack Reorganization
//!
//! ## --- Part One ---
//!
//! One Elf has the important job of loading all of the [rucksacks](https://en.wikipedia.org/wiki/Rucksack) with supplies for the jungle journey.
//! Unfortunately, that Elf didn't quite follow the packing instructions, and so a few items now need to be rearranged.
//!
//! Each rucksack has two large _compartments_.
//! All items of a given type are meant to go into exactly one of the two compartments.
//! The Elf that did the packing failed to follow this rule for exactly one item type per rucksack.
//!
//! The Elves have made a list of all of the items currently in each rucksack (your puzzle input), but they need your help finding the errors.
//! Every item type is identified by a single lowercase or uppercase letter (that is, `a` and `A` refer to different types of items).
//!
//! The list of items for each rucksack is given as characters all on a single line.
//! A given rucksack always has the same number of items in each of its two compartments, so the first half of the characters represent items in the first compartment, while the second half of the characters represent items in the second compartment.
//!
//! For example, suppose you have the following list of contents from six rucksacks:
//!
//! ```text
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!
//! * The first rucksack contains the items `vJrwpWtwJgWrhcsFMMfFFhFp`, which means its first compartment contains the items `vJrwpWtwJgWr`, while the second compartment contains the items `hcsFMMfFFhFp`.
//!     The only item type that appears in both compartments is lowercase `_p_`.
//! * The second rucksack's compartments contain `jqHRNqRjqzjGDLGL` and `rsFMfFZSrLrFZsSL`.
//!     The only item type that appears in both compartments is uppercase `_L_`.
//! * The third rucksack's compartments contain `PmmdzqPrV` and `vPwwTWBwg`; the only common item type is uppercase `_P_`.
//! * The fourth rucksack's compartments only share item type `_v_`.
//! * The fifth rucksack's compartments only share item type `_t_`.
//! * The sixth rucksack's compartments only share item type `_s_`.
//!
//! To help prioritize item rearrangement, every item type can be converted to a _priority_:
//!
//! * Lowercase item types `a` through `z` have priorities 1 through 26.
//! * Uppercase item types `A` through `Z` have priorities 27 through 52.
//!
//! In the above example, the priority of the item type that appears in both compartments of each rucksack is 16 (`p`), 38 (`L`), 42 (`P`), 22 (`v`), 20 (`t`), and 19 (`s`); the sum of these is _`157`_.
//!
//! Find the item type that appears in both compartments of each rucksack.
//! _What is the sum of the priorities of those item types?_
//!
//! ## --- Part Two ---
//!
//! As you finish identifying the misplaced items, the Elves come to you with another issue.
//!
//! For safety, the Elves are divided into groups of three.
//! Every Elf carries a badge that identifies their group.
//! For efficiency, within each group of three Elves, the badge is the _only item type carried by all three Elves_.
//! That is, if a group's badge is item type `B`, then all three Elves will have item type `B` somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.
//!
//! The problem is that someone forgot to put this year's updated authenticity sticker on the badges.
//! All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.
//!
//! Additionally, nobody wrote down which item type corresponds to each group's badges.
//! The only way to tell which item type is the right one is by finding the one item type that is _common between all three Elves_ in each group.
//!
//! Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type.
//! So, in the above example, the first group's rucksacks are the first three lines:
//!
//! ```text
//! vJrwpWtwJgWrhcsFMMfFFhFp
//! jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
//! PmmdzqPrVvPwwTWBwg
//! ```
//!
//! And the second group's rucksacks are the next three lines:
//!
//! ```text
//! wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
//! ttgJtRGJQctTZtZT
//! CrZsJsPPZsGzwwsLwLmpwMDw
//! ```
//!
//! In the first group, the only item type that appears in all three rucksacks is lowercase `r`; this must be their badges.
//! In the second group, their badge item type must be `Z`.
//!
//! Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (`r`) for the first group and 52 (`Z`) for the second group.
//! The sum of these is _`70`_.
//!
//! Find the item type that corresponds to the badges of each three-Elf group.
//! _What is the sum of the priorities of those item types?_

use crate::prelude::*;

#[aoc_runner_derive::aoc(day3, part1)]
fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mid = line.len() / 2;
            for c in line[..mid].chars() {
                if line[mid..].contains(c) {
                    return c;
                }
            }
            unreachable!("no match found");
        })
        .map(priority)
        .sum()
}

#[aoc_runner_derive::aoc(day3, part2)]
fn part2(input: &str) -> u32 {
    input
        .lines()
        .tuples()
        .map(|(l0, l1, l2)| {
            for c in l0.chars() {
                if l1.contains(c) && l2.contains(c) {
                    return c;
                }
            }
            unreachable!("no match found");
        })
        .map(priority)
        .sum()
}

fn priority(c: char) -> u32 {
    //convert to priority
    match c {
        'a'..='z' => c as u32 - 'a' as u32 + 1,
        'A'..='Z' => c as u32 - 'A' as u32 + 27,
        _ => unreachable!("invalid char"),
    }
}

#[cfg(test)]
static TEST_INPUT_1: &str = r"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

#[test]
fn test_part1() -> Result<()> {
    let values = TEST_INPUT_1;
    assert_eq!(157, part1(values));
    Ok(())
}

#[test]
fn test_part1_solution() -> Result<()> {
    let values = include_str!("../input/2022/day3.txt").trim();
    assert_eq!(8039, part1(&values));
    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let values = TEST_INPUT_1;
    assert_eq!(45000, part2(&values));
    Ok(())
}

#[test]
fn test_part2_solution() -> Result<()> {
    let values = include_str!("../input/2022/day3.txt").trim();
    assert_eq!(2510, part2(&values));
    Ok(())
}
