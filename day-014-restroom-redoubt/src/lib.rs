use std::str::FromStr;

use aoc_plumbing::Problem;
use aoc_std::geometry::Point2D;
use nom::{
    bytes::complete::tag,
    character::complete,
    combinator,
    multi::separated_list1,
    sequence::{self, preceded, separated_pair},
    IResult,
};

#[derive(Debug, Clone)]
pub struct RestroomRedoubtGen<const N: usize, const M: usize> {
    guards: Vec<Guard>,
}

impl<const N: usize, const M: usize> FromStr for RestroomRedoubtGen<N, M> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, guards) = parse_guards(s).map_err(|e| e.to_owned())?;
        Ok(Self { guards })
    }
}

impl<const N: usize, const M: usize> RestroomRedoubtGen<N, M> {
    pub fn classify(&self, point: &Point2D<i64>) -> Option<Quadrant> {
        let mid_x = N as i64 / 2;
        let mid_y = M as i64 / 2;

        if point.x == mid_x || point.y == mid_y {
            return None;
        }

        let upper = point.y < mid_y;
        let left = point.x < mid_x;

        Some(match (upper, left) {
            (true, true) => Quadrant::UL,
            (true, false) => Quadrant::UR,
            (false, true) => Quadrant::LL,
            (false, false) => Quadrant::LR,
        })
    }

    pub fn safety_factor(&self, seconds: i64) -> i64 {
        let mut counts = [0; 4];
        for guard in self.guards.iter() {
            let pos = guard.bound_position(seconds, N as i64, M as i64);
            if let Some(q) = self.classify(&pos) {
                counts[q as usize] += 1;
            }
        }

        counts.iter().product()
    }

    pub fn tree(&self) -> i64 {
        let mut cache = [0_u128; M];

        'outer: for i in 3000..10_000 {
            for v in cache.iter_mut() {
                *v = 0;
            }

            for guard in self.guards.iter() {
                let pos = guard.bound_position(i, N as i64, M as i64);
                let mask: u128 = 1 << pos.x as usize;
                if cache[pos.y as usize] & mask != 0 {
                    continue 'outer;
                }
                cache[pos.y as usize] |= mask;
            }

            return i;
        }

        i64::MIN
    }
}

impl<const N: usize, const M: usize> Problem for RestroomRedoubtGen<N, M> {
    const DAY: usize = 14;
    const TITLE: &'static str = "restroom redoubt";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i64;
    type P2 = i64;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.safety_factor(100))
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.tree())
    }
}

pub type RestroomRedoubt = RestroomRedoubtGen<101, 103>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Quadrant {
    UL = 0,
    UR,
    LL,
    LR,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Guard {
    origin: Point2D<i64>,
    velocity: Point2D<i64>,
}

impl Guard {
    #[inline]
    pub fn raw_position(&self, seconds: i64) -> Point2D<i64> {
        self.origin + self.velocity * seconds
    }

    pub fn bound_position(&self, seconds: i64, width: i64, height: i64) -> Point2D<i64> {
        let raw = self.raw_position(seconds);
        Point2D::new(raw.x.rem_euclid(width), raw.y.rem_euclid(height))
    }
}

fn parse_guards(input: &str) -> IResult<&str, Vec<Guard>> {
    separated_list1(complete::newline, parse_guard)(input)
}

fn parse_guard(input: &str) -> IResult<&str, Guard> {
    combinator::map(
        sequence::tuple((
            combinator::map(
                separated_pair(
                    preceded(tag("p="), complete::i64),
                    complete::char(','),
                    complete::i64,
                ),
                |(x, y)| Point2D::new(x, y),
            ),
            combinator::map(
                separated_pair(
                    preceded(tag(" v="), complete::i64),
                    complete::char(','),
                    complete::i64,
                ),
                |(x, y)| Point2D::new(x, y),
            ),
        )),
        |(origin, velocity)| Guard { origin, velocity },
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
        let solution = RestroomRedoubt::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(219512160, 6398));
    }

    #[test]
    fn example() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let solution = RestroomRedoubtGen::<11, 7>::solve(input).unwrap();
        assert_eq!(solution, Solution::new(12, 3000));
    }
}
