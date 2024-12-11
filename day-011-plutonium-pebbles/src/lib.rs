use std::str::FromStr;

use aoc_plumbing::Problem;
use rustc_hash::{FxBuildHasher, FxHashMap};

#[derive(Debug, Clone)]
pub struct PlutoniumPebbles {
    p1: usize,
    p2: usize,
}

impl FromStr for PlutoniumPebbles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cur = s.trim().split(' ').map(|v| v.parse::<u64>().map(|a| (a, 1))).collect::<std::result::Result<FxHashMap<u64, usize>, _>>()?;
        let mut next = FxHashMap::with_capacity_and_hasher(4000, FxBuildHasher);
        let mut p1 = 0;
        for i in 0..75 {
            if i == 25 {
                p1 = cur.values().sum();
            }
            for (k, v) in cur.drain() {
                if k == 0 {
                    *next.entry(1).or_default() += v;
                } else if let Some((left, right)) = split_even_digits(k) {
                    *next.entry(left).or_default() += v;
                    *next.entry(right).or_default() += v;
                } else {
                    *next.entry(k * 2024).or_default() += v;
                }
            }

            std::mem::swap(&mut cur, &mut next);
        }

        let p2 = cur.values().sum();


        Ok(Self { p1, p2 })
    }
}

fn split_even_digits(stone: u64) -> Option<(u64, u64)> {
    let digits = stone.checked_ilog10().unwrap_or(0) + 1;

    if digits % 2 == 0 {
        let divisor = 10_u64.pow(digits / 2);
        Some((stone / divisor, stone % divisor))
    } else {
        None
    }
}

impl Problem for PlutoniumPebbles {
    const DAY: usize = 11;
    const TITLE: &'static str = "plutonium pebbles";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
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
        let solution = PlutoniumPebbles::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(189547, 224577979481346));
    }

    #[test]
    fn example() {
        let input = "125 17";
        let solution = PlutoniumPebbles::solve(input).unwrap();
        assert_eq!(solution, Solution::new(55312, 65601038650482));
    }
}
