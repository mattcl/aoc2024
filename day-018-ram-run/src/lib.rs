use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::{
    geometry::Location,
    pathing::dijkstra::{dijkstra, DijkstraResult},
};

#[derive(Debug, Clone)]
pub struct RamRunGen<const N: usize> {
    p1: i64,
    p2: (u8, u8),
}

impl<const N: usize> FromStr for RamRunGen<N> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let take = if N == 7 { 12 } else { 1024 };

        let initial: u128 = 1 << (N + 1) | 1;

        // make this larger than we need and put a "wall" around the grid
        let mut grid = vec![initial; N + 2];
        grid[0] = u128::MAX;
        grid[N + 1] = u128::MAX;

        let mut iter = s.trim().lines().filter_map(|line| {
            line.split_once(',')
                .map(|(l, r)| (l.parse::<u8>().unwrap(), r.parse::<u8>().unwrap()))
        });

        for (c, r) in iter.by_ref().take(take) {
            grid[r as usize + 1] |= 1 << (c + 1);
        }

        let start = Location::new(1, 1);
        let end = Location::new(N, N);

        let res = dijkstra(
            &start,
            &mut |loc| {
                loc.cardinal_neighbors()
                    .filter(|(_, nloc)| {
                        let mask = 1 << (nloc.col);
                        grid[nloc.row] & mask == 0
                    })
                    .map(|(_dir, nloc)| (nloc, 1))
            },
            &mut |n| n == &end,
        );

        let p1 = res.cost()?;

        let remaining: Vec<(u8, u8)> = iter.collect();

        // okay, let's binary search through the remaining configurations until
        // we find the one we want
        let mut left = 0;
        let mut right = remaining.len();
        let mut cur_grid = grid.clone();

        while left < right {
            let cur_idx = (left + right) / 2;

            for (c, r) in &remaining[left..=cur_idx] {
                let byte_mask = 1 << (c + 1);
                cur_grid[*r as usize + 1] |= byte_mask;
            }

            let res = dijkstra(
                &start,
                &mut |loc| {
                    loc.cardinal_neighbors()
                        .filter(|(_, nloc)| {
                            let mask = 1 << (nloc.col);
                            cur_grid[nloc.row] & mask == 0
                        })
                        .map(|(_dir, nloc)| (nloc, 1))
                },
                &mut |n| n == &end,
            );
            match res {
                DijkstraResult::Success { .. } => {
                    left = cur_idx + 1;
                }
                DijkstraResult::NoPath { .. } => {
                    right = cur_idx;

                    // reset the grid to up to the left bound
                    cur_grid = grid.clone();
                    for (c, r) in &remaining[..=left] {
                        let byte_mask = 1 << (c + 1);
                        cur_grid[*r as usize + 1] |= byte_mask;
                    }
                }
            }
        }

        Ok(Self {
            p1,
            p2: remaining[left],
        })
    }
}

impl<const N: usize> Problem for RamRunGen<N> {
    const DAY: usize = 18;
    const TITLE: &'static str = "ram run";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = String;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(format!("{},{}", self.p2.0, self.p2.1))
    }
}

pub type RamRun = RamRunGen<71>;

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = RamRun::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(292, "58,44".into()));
    }

    #[test]
    fn example() {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        let solution = RamRunGen::<7>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(22, "6,1".into()));
    }
}
