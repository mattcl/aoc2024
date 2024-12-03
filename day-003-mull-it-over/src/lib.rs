use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    branch,
    bytes::complete::{tag, take_until},
    character::complete,
    combinator,
    multi::{fold_many0, fold_many1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, Clone)]
pub struct MullItOver {
    part1: i64,
    part2: i64,
}

impl FromStr for MullItOver {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // again, easier to solve both during the parsing step
        let (_, part1) = parse_input(s).map_err(|e| e.to_owned())?;
        let (_, part2) = parse_part2_input(s).map_err(|e| e.to_owned())?;

        Ok(Self { part1, part2 })
    }
}

fn parse_input(input: &str) -> IResult<&str, i64> {
    fold_many1(
        parse_maybe_mul,
        || 0_i64,
        |mut acc: i64, item| {
            if let Some(v) = item {
                acc += v.val();
            }
            acc
        },
    )(input)
}

fn parse_part2_input(input: &str) -> IResult<&str, i64> {
    fold_many1(
        parse_maybe_dont_mul,
        || 0_i64,
        |mut acc: i64, item| {
            acc += item;
            acc
        },
    )(input)
}

fn parse_maybe_mul(input: &str) -> IResult<&str, Option<Inst>> {
    preceded(
        take_until("mul"),
        branch::alt((
            combinator::map(parse_mul, Some),
            combinator::map(tag("mul"), |_| None),
        )),
    )(input)
}

fn parse_mul(input: &str) -> IResult<&str, Inst> {
    let (input, (left, right)) = delimited(
        tag("mul("),
        separated_pair(complete::i64, tag(","), complete::i64),
        tag(")"),
    )(input)?;
    Ok((input, Inst { left, right }))
}

fn parse_dont(input: &str) -> IResult<&str, &str> {
    branch::alt((take_until("do()"), combinator::rest))(input)
}

fn parse_maybe_dont_mul(input: &str) -> IResult<&str, i64> {
    let (input, sub_region) = branch::alt((
        terminated(take_until("don't()"), parse_dont),
        combinator::rest,
    ))(input)?;

    let res = parse_input(sub_region);

    if res.is_err() && sub_region.is_empty() {
        res
    } else {
        let v = res.map(|(_, v)| v).unwrap_or_default();

        Ok((input, v))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Inst {
    left: i64,
    right: i64,
}

impl Inst {
    pub fn val(&self) -> i64 {
        self.left * self.right
    }
}

impl Problem for MullItOver {
    const DAY: usize = 3;
    const TITLE: &'static str = "mull it over";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.part1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.part2)
    }
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = MullItOver::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(170068701, 78683433));
    }

    #[test]
    fn example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let solution = MullItOver::solve(input).unwrap();
        assert_eq!(solution, Solution::new(161, 48));
    }
}
