use std::str::FromStr;

use aoc_plumbing::Problem;
use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};
use rayon::prelude::*;

const MOD_MASK: i64 = (1 << 24) - 1;

// max set of diffs of -9,0,0,0, (base-19 I999)
const SEQ_MAX: usize = 126891;
const SEQ_SIZE: usize = SEQ_MAX + 1;
const DESIRED_CHUNKS: usize = 4;

const DESIRED_AGG_CHUNKS: usize = 1_000;
const AGG_CHUNK_SIZE: usize = SEQ_SIZE / DESIRED_AGG_CHUNKS;

const SLOT_1: i64 = 19 * 19 * 19;
const SLOT_2: i64 = 19 * 19;
const SLOT_3: i64 = 19;

#[derive(Debug, Clone)]
pub struct MonkeyMarket {
    p1: i64,
    p2: u16,
}

pub struct Chunk {
    values: Vec<i64>,
    totals: Vec<u16>,
}

impl Chunk {
    pub fn new(vals: &[i64]) -> Self {
        Self {
            values: vals.to_vec(),
            totals: Vec::default(),
        }
    }
}

impl FromStr for MonkeyMarket {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, initial_numbers) = parse_numbers(s).map_err(|e| e.to_owned())?;

        let chunk_size = (initial_numbers.len() / DESIRED_CHUNKS)
            + if initial_numbers.len() % DESIRED_CHUNKS == 0 {
                0
            } else {
                1
            };

        let mut chunks = initial_numbers
            .chunks(chunk_size)
            .map(Chunk::new)
            .collect::<Vec<_>>();

        let p1 = chunks
            .par_iter_mut()
            .map(|chunk| {
                let mut num_total = 0;

                let mut seen = vec![usize::MAX; SEQ_SIZE];
                chunk.totals = vec![0; SEQ_SIZE];

                for i in 0..chunk.values.len() {
                    let mut cur = chunk.values[i];
                    // let mut key: usize = 0;
                    let mut prev = cur % 10;

                    // not doing this in the loop saves us a little bit of time
                    // because we don't have to check an additional condition in
                    // the loop
                    cur = next_number(cur);
                    let cur_digit = cur % 10;
                    let mut delta1 = cur_digit - prev + 9;
                    prev = cur_digit;

                    cur = next_number(cur);
                    let cur_digit = cur % 10;
                    let mut delta2 = cur_digit - prev + 9;
                    prev = cur_digit;

                    cur = next_number(cur);
                    let cur_digit = cur % 10;
                    let mut delta3 = cur_digit - prev + 9;
                    prev = cur_digit;

                    for _ in 0..1997 {
                        cur = next_number(cur);
                        let cur_digit = cur % 10;
                        let delta = cur_digit - prev + 9;
                        prev = cur_digit;
                        // key = ((key << 5) & SEQ_MASK) | (delta + 9);
                        let key =
                            ((SLOT_1 * delta1) + (SLOT_2 * delta2) + (SLOT_3 * delta3) + delta)
                                as usize;

                        delta1 = delta2;
                        delta2 = delta3;
                        delta3 = delta;

                        // let adjusted_key = (key - SEQ_MIN) as usize;

                        if seen[key] != i {
                            seen[key] = i;
                            chunk.totals[key] += cur_digit as u16;
                        }
                    }
                    num_total += cur;
                }
                num_total
            })
            .sum();

        let best_aggregator = (0..(SEQ_SIZE / AGG_CHUNK_SIZE)).collect::<Vec<_>>();

        let p2 = best_aggregator
            .into_par_iter()
            .map(|base_idx| {
                let mut max = 0;
                let base = base_idx * AGG_CHUNK_SIZE;
                for i in base..((base + AGG_CHUNK_SIZE).min(SEQ_MAX)) {
                    let mut cur_tot = 0;
                    for chunk in chunks.iter() {
                        cur_tot += chunk.totals[i];
                    }
                    max = max.max(cur_tot);
                }
                max
            })
            .max()
            .unwrap();

        Ok(Self { p1, p2 })
    }
}

#[inline]
fn next_number(input: i64) -> i64 {
    let mut a = (input ^ (input << 6)) & MOD_MASK;
    a = a ^ (a >> 5);
    a ^ ((a << 11) & MOD_MASK)
}

fn parse_numbers(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(newline, complete::i64)(input)
}

impl Problem for MonkeyMarket {
    const DAY: usize = 22;
    const TITLE: &'static str = "monkey market";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = u16;

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
        let solution = MonkeyMarket::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(17965282217, 2152));
    }

    // #[test]
    // fn example1() {
    //     let input = "1
    // 10
    // 100
    // 2024";
    //     let solution = MonkeyMarket::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(37327623, 24));
    // }

    // #[test]
    // fn example2() {
    //     let input = "1
    // 2
    // 3
    // 2024";
    //     let solution = MonkeyMarket::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(37990510, 23));
    // }

    #[test]
    fn verify_next() {
        assert_eq!(100000000 & MOD_MASK, 16113920);
        assert_eq!(next_number(123), 15887950);

        let mut a = 1;

        for _ in 0..2000 {
            a = next_number(a);
            // println!("{:024b}", a);
        }

        // assert!(false);

        assert_eq!(a, 8685429);
    }
}
