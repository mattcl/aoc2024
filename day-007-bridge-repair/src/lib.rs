use std::{collections::VecDeque, i64, str::FromStr};

use aoc_plumbing::Problem;
use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1, space1},
    combinator,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::prelude::*;

#[derive(Debug, Clone)]
pub struct BridgeRepair {
    p1: i64,
    p2: i64,
}

impl FromStr for BridgeRepair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, equations) = parse_equations(s).map_err(|e| e.to_owned())?;

        let (p1, p2) = equations
            .into_par_iter()
            .map(|eq| eq.is_valid_combined_dfs())
            .reduce(|| (0, 0), |(p1, p2), (a1, a2)| (p1 + a1, p2 + a2));

        Ok(Self { p1, p2 })

        // let mut p1 = 0;
        // equations.retain(|eq| {
        //     if eq.is_valid() {
        //         p1 += eq.left;
        //         false
        //     } else {
        //         true
        //     }
        // });

        // let p2: i64 = equations
        //     .into_par_iter()
        //     .filter(|eq| eq.is_concat_valid_dfs())
        //     .map(|eq| eq.left)
        //     .sum();

        // Ok(Self { p1, p2: p1 + p2 })
    }
}

impl Problem for BridgeRepair {
    const DAY: usize = 7;
    const TITLE: &'static str = "bridge repair";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.p1)
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.p2)
    }
}

#[derive(Debug, Clone)]
pub struct Equation {
    left: i64,
    right: Vec<i64>,
    widths: Vec<u8>,
}

impl Equation {
    pub fn is_valid(&self) -> bool {
        let mut heads = VecDeque::default();
        let len = self.right.len();

        heads.push_front((len - 1, self.left));

        while let Some((idx, head)) = heads.pop_front() {
            let v = self.right[idx];

            if idx > 0 && v != 0 && head % v == 0 {
                heads.push_back((idx - 1, head / v));
            }

            let head = head - v;

            if idx == 0 && head == 0 {
                return true;
            } else if idx > 0 {
                heads.push_back((idx - 1, head));
            }
        }

        false
    }

    pub fn is_concat_valid(&self) -> bool {
        let mut heads = VecDeque::default();
        let len = self.right.len();

        heads.push_front((1, self.right[0]));

        while let Some((idx, head)) = heads.pop_front() {
            let v = self.right[idx];

            let next_idx = idx + 1;

            let sum = head + v;
            let mul = head * v;
            let con = concat(head, v);
            if next_idx >= len {
                if sum == self.left || mul == self.left || con == self.left {
                    return true;
                }
            } else {
                heads.push_back((next_idx, sum));
                heads.push_back((next_idx, mul));
                heads.push_back((next_idx, con));
            }
        }

        false
    }

    pub fn is_concat_valid_dfs(&self) -> bool {
        self._is_concat_valid_dfs(1, self.right[0])
    }

    fn _is_concat_valid_dfs(&self, idx: usize, head: i64) -> bool {
        if idx == self.right.len() {
            if head == self.left {
                return true;
            }
            return false;
        }

        if head > self.left {
            return false;
        }

        let v = self.right[idx];
        let next_idx = idx + 1;

        self._is_concat_valid_dfs(next_idx, head + v)
            || self._is_concat_valid_dfs(next_idx, head * v)
            || self._is_concat_valid_dfs(next_idx, concat(head, v))
    }

    pub fn is_valid_combined_dfs(&self) -> (i64, i64) {
        let mut p1_valid = false;
        let mut p2_valid = false;
        self._is_valid_combined_dfs(1, self.right[0], false, &mut p1_valid, &mut p2_valid);

        if p1_valid {
            (self.left, self.left)
        } else if p2_valid {
            (0, self.left)
        } else {
            (0, 0)
        }
    }

    fn _is_valid_combined_dfs(
        &self,
        idx: usize,
        head: i64,
        used_concat: bool,
        p1_valid: &mut bool,
        p2_valid: &mut bool,
    ) {
        if idx == self.right.len() {
            if head == self.left {
                *p2_valid = true;
                if !used_concat {
                    *p1_valid = true;
                }
            }
            return;
        }

        if head > self.left {
            return;
        }

        let v = self.right[idx];
        let next_idx = idx + 1;

        self._is_valid_combined_dfs(next_idx, head + v, used_concat, p1_valid, p2_valid);

        if *p1_valid {
            return;
        }

        self._is_valid_combined_dfs(next_idx, head * v, used_concat, p1_valid, p2_valid);

        if *p1_valid {
            return;
        }

        if !*p2_valid {
            let width = self.widths[idx];
            self._is_valid_combined_dfs(
                next_idx,
                (head * 10_i64.pow(width as u32)) + v,
                true,
                p1_valid,
                p2_valid,
            );
        }
    }

    pub fn is_valid_sequential(&self) -> (i64, i64) {
        if self.is_valid() {
            (self.left, self.left)
        } else if self.is_concat_valid() {
            (0, self.left)
        } else {
            (0, 0)
        }
    }

    pub fn is_valid_combined(&self) -> (i64, i64) {
        let mut heads = VecDeque::default();
        let len = self.right.len();
        let mut p2_valid = false;

        heads.push_front((1, self.right[0], false));

        while let Some((idx, head, used_concat)) = heads.pop_front() {
            if p2_valid && used_concat {
                continue;
            }
            let v = self.right[idx];

            let next_idx = idx + 1;

            let sum = head + v;
            let mul = head * v;
            let con = if p2_valid { i64::MAX } else { concat(head, v) };
            if next_idx >= len {
                if sum == self.left || mul == self.left {
                    if used_concat {
                        p2_valid = true;
                    } else {
                        return (self.left, self.left);
                    }
                } else if con == self.left {
                    p2_valid = true;
                }
            } else if p2_valid {
                if !used_concat {
                    heads.push_back((next_idx, sum, used_concat));
                    heads.push_back((next_idx, mul, used_concat));
                }
            } else {
                heads.push_back((next_idx, sum, used_concat));
                heads.push_back((next_idx, mul, used_concat));
                heads.push_back((next_idx, con, true));
            }
        }

        if p2_valid {
            (0, self.left)
        } else {
            (0, 0)
        }
    }

    pub fn is_valid_combined_vec(&self) -> (i64, i64) {
        let mut heads = Vec::default();
        let mut next = Vec::default();
        let len = self.right.len();
        let mut p2_valid = false;

        heads.push((1, self.right[0], false));

        while !heads.is_empty() {
            for (idx, head, used_concat) in heads.drain(0..) {
                if p2_valid && used_concat {
                    continue;
                }
                let v = self.right[idx];

                let next_idx = idx + 1;

                let sum = head + v;
                let mul = head * v;
                let con = if p2_valid { i64::MAX } else { concat(head, v) };
                if next_idx >= len {
                    if sum == self.left || mul == self.left {
                        if used_concat {
                            p2_valid = true;
                        } else {
                            return (self.left, self.left);
                        }
                    } else if con == self.left {
                        p2_valid = true;
                    }
                } else if p2_valid {
                    if !used_concat {
                        next.push((next_idx, sum, used_concat));
                        next.push((next_idx, mul, used_concat));
                    }
                } else {
                    next.push((next_idx, sum, used_concat));
                    next.push((next_idx, mul, used_concat));
                    next.push((next_idx, con, true));
                }
            }

            std::mem::swap(&mut heads, &mut next);
        }

        if p2_valid {
            (0, self.left)
        } else {
            (0, 0)
        }
    }
}

fn concat(left: i64, right: i64) -> i64 {
    let mut digits = 0;
    let mut cur = right;
    while cur > 0 {
        if cur == 10 {
            digits += 2;
            break;
        }
        digits += 1;
        cur /= 10;
    }
    (left * 10_i64.pow(digits.max(1))) + right
}

fn digits(val: i64) -> u8 {
    let mut digits = 0;
    let mut cur = val;
    while cur > 0 {
        if cur == 10 {
            digits += 2;
            break;
        }
        digits += 1;
        cur /= 10;
    }
    digits.max(1) as u8
}

fn parse_equations(input: &str) -> IResult<&str, Vec<Equation>> {
    separated_list1(multispace1, parse_equation)(input)
}

fn parse_equation(input: &str) -> IResult<&str, Equation> {
    combinator::map(
        separated_pair(
            complete::i64,
            tag(": "),
            separated_list1(space1, complete::i64),
        ),
        |(left, right)| {
            let widths = right.iter().map(|v| digits(*v)).collect();
            Equation {
                left,
                right,
                widths,
            }
        },
    )(input)
}

#[cfg(test)]
mod tests {
    use aoc_plumbing::Solution;

    use super::*;

    #[test]
    #[ignore]
    fn full_dataset() {
        let input = std::fs::read_to_string("input.txt").expect("Unable to load input");
        let solution = BridgeRepair::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(2299996598890, 362646859298554));
    }

    #[test]
    fn example() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let solution = BridgeRepair::solve(input).unwrap();
        assert_eq!(solution, Solution::new(3749, 11387));
    }

    #[test]
    fn test_concat() {
        assert_eq!(concat(12, 345), 12345);
        assert_eq!(concat(12, 10), 1210);
        assert_eq!(concat(126, 100), 126100);
        assert_eq!(concat(1, 1), 11);
    }
}
