use std::str::FromStr;

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::geometry::Point2D;
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHashSet};
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone)]
pub struct LinenLayout {
    p1: usize,
    p2: usize,
}

impl FromStr for LinenLayout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let patterns: FxHashSet<&str> = left.split(", ").collect();

        // let mut possible = 0;
        // let mut ways = 0;

        let lines: Vec<_> = right.lines().collect();

        // let mut cache = FxHashMap::with_capacity_and_hasher(5000, rustc_hash::FxBuildHasher);

        // for line in lines {
        //     let res = count_possible(line, &patterns, &mut cache);
        //     if res > 0 {
        //         possible += 1;
        //         ways += res;
        //     }
        // }

        // this is surprisingly slower than checking each one independently
        // let Point2D { x: possible, y: ways } = lines
        //     .par_chunks(20)
        //     .map(|chunk| {
        //         let mut possible = 0;
        //         let mut ways = 0;
        //         let mut cache = FxHashMap::with_capacity_and_hasher(5000, rustc_hash::FxBuildHasher);

        //         for line in chunk {
        //             let res = count_possible(line, &patterns, &mut cache);
        //             if res > 0 {
        //                 possible += 1;
        //                 ways += res;
        //             }
        //         }

        //         Point2D::<usize>::new(possible, ways)
        //     })
        //     .sum();

        let Point2D {
            x: possible,
            y: ways,
        } = lines
            .par_iter()
            .map(|line| {
                let mut cache =
                    FxHashMap::with_capacity_and_hasher(5000, rustc_hash::FxBuildHasher);

                let res = count_possible(line, &patterns, &mut cache);
                if res > 0 {
                    Point2D::<usize>::new(1, res)
                } else {
                    Point2D::default()
                }
            })
            .sum();

        Ok(Self {
            p1: possible,
            p2: ways,
        })
    }
}

fn count_possible(
    input: &str,
    patterns: &FxHashSet<&str>,
    cache: &mut FxHashMap<u64, usize>,
) -> usize {
    let key = xxh3_64(input.as_bytes());
    if let Some(v) = cache.get(&key).copied() {
        return v;
    }

    let mut ways = 0;

    if patterns.contains(&input) {
        ways += 1;
    }

    for i in (1..input.len()).rev() {
        let (car, cdr) = input.split_at(i);
        if patterns.contains(&car) {
            let res = count_possible(cdr, patterns, cache);
            ways += res;
        }
    }

    cache.insert(key, ways);

    ways
}

impl Problem for LinenLayout {
    const DAY: usize = 19;
    const TITLE: &'static str = "linen layout";
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
        let solution = LinenLayout::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(213, 1016700771200474));
    }

    #[test]
    fn example() {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        let solution = LinenLayout::solve(input).unwrap();
        assert_eq!(solution, Solution::new(6, 16));
    }
}
