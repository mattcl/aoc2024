use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use rustc_hash::{FxBuildHasher, FxHashMap};

#[derive(Debug, Clone)]
pub struct HistorianHysteria {
    left: Vec<i32>,
    right: Vec<i32>,
    counts: FxHashMap<i32, i32>,
}

impl FromStr for HistorianHysteria {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = Vec::with_capacity(1000);
        let mut right = Vec::with_capacity(1000);
        let mut counts: FxHashMap<i32, i32> =
            FxHashMap::with_capacity_and_hasher(1000, FxBuildHasher);

        for line in s.trim().lines() {
            let (l, r) = line
                .split_once(" ")
                .ok_or_else(|| anyhow!("invalid input"))?;
            let lv: i32 = l.trim().parse()?;
            left.push(lv);
            let rv: i32 = r.trim().parse()?;
            right.push(rv);
            counts.entry(rv).and_modify(|e| *e += 1).or_insert(1);
        }

        left.sort();
        right.sort();

        Ok(Self {
            left,
            right,
            counts,
        })
    }
}

impl Problem for HistorianHysteria {
    const DAY: usize = 1;
    const TITLE: &'static str = "historian hysteria";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i32;
    type P2 = i32;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self
            .left
            .iter()
            .zip(self.right.iter())
            .map(|(l, r)| (l - r).abs())
            .sum())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self
            .left
            .iter()
            .map(|v| v * self.counts.get(v).copied().unwrap_or_default())
            .sum())
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
        let solution = HistorianHysteria::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1889772, 23228917));
    }

    #[test]
    fn example() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let solution = HistorianHysteria::solve(input).unwrap();
        assert_eq!(solution, Solution::new(11, 31));
    }
}
