use std::{collections::hash_map::Entry, str::FromStr};

use aoc_plumbing::Problem;
use aoc_std::geometry::Point2D;
use itertools::Itertools;
use num::integer::gcd;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct ResonantCollinearity {
    antennas: FxHashMap<u8, FxHashSet<Point2D<i8>>>,
    size: i8,
}

impl FromStr for ResonantCollinearity {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas: FxHashMap<u8, FxHashSet<Point2D<i8>>> = FxHashMap::default();

        let mut size = 0;
        for (r, line) in s.trim().lines().enumerate() {
            size = line.len() as i8;
            for (c, ch) in line.chars().enumerate() {
                if ch != '.' {
                    match antennas.entry(ch as u8) {
                        Entry::Occupied(mut occupied_entry) => {
                            occupied_entry.get_mut().insert((c as i8, r as i8).into());
                        }
                        Entry::Vacant(vacant_entry) => {
                            let s = vacant_entry.insert(FxHashSet::default());
                            s.insert((c as i8, r as i8).into());
                        }
                    }
                }
            }
        }

        Ok(Self { antennas, size })
    }
}

impl ResonantCollinearity {
    pub fn compute_antinodes(&self) -> usize {
        let mut antinodes = FxHashSet::default();

        for antennas in self.antennas.values() {
            self.compute_antinodes_for(antennas, &mut antinodes);
        }

        antinodes.len()
    }

    fn compute_antinodes_for(
        &self,
        antennas: &FxHashSet<Point2D<i8>>,
        antinodes: &mut FxHashSet<Point2D<i8>>,
    ) {
        for (a, b) in antennas.iter().tuple_combinations() {
            let left = a.min(b);
            let right = a.max(b);
            let slope = Point2D::new(right.x - left.x, right.y - left.y);

            let candidate1 = left - slope;
            let candidate2 = right + slope;

            if candidate1.x >= 0
                && candidate1.x < self.size
                && candidate1.y >= 0
                && candidate1.y < self.size
            {
                antinodes.insert(candidate1);
            }

            if candidate2.x >= 0
                && candidate2.x < self.size
                && candidate2.y >= 0
                && candidate2.y < self.size
            {
                antinodes.insert(candidate2);
            }
        }
    }

    pub fn compute_line_antinodes(&self) -> usize {
        let mut antinodes = FxHashSet::default();

        for antennas in self.antennas.values() {
            self.compute_line_antinodes_for(antennas, &mut antinodes);
        }

        antinodes.len()
    }

    fn compute_line_antinodes_for(
        &self,
        antennas: &FxHashSet<Point2D<i8>>,
        antinodes: &mut FxHashSet<Point2D<i8>>,
    ) {
        for (a, b) in antennas.iter().tuple_combinations() {
            let left = a.min(b);
            let right = a.max(b);
            let mut slope = Point2D::new(right.x - left.x, right.y - left.y);

            loop {
                let d = gcd(slope.x, slope.y);

                if d == 1 {
                    break;
                }

                slope.x /= d;
                slope.y /= d;
            }

            let mut candidate1 = left - slope;
            let mut candidate2 = left + slope;

            antinodes.insert(*left);

            loop {
                if candidate1.x >= 0
                    && candidate1.x < self.size
                    && candidate1.y >= 0
                    && candidate1.y < self.size
                {
                    antinodes.insert(candidate1);
                } else {
                    break;
                }

                candidate1 -= slope;
            }

            loop {
                if candidate2.x >= 0
                    && candidate2.x < self.size
                    && candidate2.y >= 0
                    && candidate2.y < self.size
                {
                    antinodes.insert(candidate2);
                } else {
                    break;
                }
                candidate2 += slope;
            }
        }
    }
}

impl Problem for ResonantCollinearity {
    const DAY: usize = 8;
    const TITLE: &'static str = "resonant collinearity";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.compute_antinodes())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.compute_line_antinodes())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct AntennaGrid {
    frequency: u8,
    rows: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Antenna {
    point: Point2D<i64>,
    frequency: u8,
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = ResonantCollinearity::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(413, 1417));
    }

    #[test]
    fn example() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let solution = ResonantCollinearity::solve(input).unwrap();
        assert_eq!(solution, Solution::new(14, 34));
    }
}
