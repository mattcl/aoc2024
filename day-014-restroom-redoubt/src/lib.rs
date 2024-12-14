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
    pub fn classify(&self, point: &Point2D<i32>) -> Option<Quadrant> {
        let mid_x = N as i32 / 2;
        let mid_y = M as i32 / 2;

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

    pub fn safety_factor(&self, seconds: i32) -> i32 {
        let mut counts = [0; 4];
        for guard in self.guards.iter() {
            let pos = guard.bound_position(seconds, N as i32, M as i32);
            if let Some(q) = self.classify(&pos) {
                counts[q as usize] += 1;
            }
        }

        counts.iter().product()
    }

    pub fn tree(&self) -> i32 {
        let mut x_pos = vec![vec![0; self.guards.len()]; N];
        let mut y_pos = vec![vec![0; self.guards.len()]; M];
        let mut cache = [[0; N]; M];

        for idx in 0..M.max(N) {
            for (g_idx, guard) in self.guards.iter().enumerate() {
                let pos = guard.bound_position(idx as i32, N as i32, M as i32);
                x_pos[idx % N][g_idx] = pos.x;
                y_pos[idx % M][g_idx] = pos.y;
            }
        }

        'outer: for i in 1000..10_000 {
            for (x, y) in x_pos[i % N].iter().zip(y_pos[i % M].iter()) {
                if cache[*y as usize][*x as usize] == i {
                    continue 'outer;
                }

                cache[*y as usize][*x as usize] = i;
            }

            return i as i32;
        }

        i32::MIN
    }
}

impl<const N: usize, const M: usize> Problem for RestroomRedoubtGen<N, M> {
    const DAY: usize = 14;
    const TITLE: &'static str = "restroom redoubt";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = i32;
    type P2 = i32;

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
    origin: Point2D<i32>,
    velocity: Point2D<i32>,
}

impl Guard {
    #[inline]
    pub fn raw_position(&self, seconds: i32) -> Point2D<i32> {
        self.origin + self.velocity * seconds
    }

    pub fn bound_position(&self, seconds: i32, width: i32, height: i32) -> Point2D<i32> {
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
                    preceded(tag("p="), complete::i32),
                    complete::char(','),
                    complete::i32,
                ),
                |(x, y)| Point2D::new(x, y),
            ),
            combinator::map(
                separated_pair(
                    preceded(tag(" v="), complete::i32),
                    complete::char(','),
                    complete::i32,
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
        assert_eq!(solution, Solution::new(12, 1002));
    }
}
