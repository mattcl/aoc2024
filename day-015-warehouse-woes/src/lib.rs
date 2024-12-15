use std::{collections::BTreeSet, str::FromStr};

use anyhow::anyhow;
use aoc_plumbing::Problem;
use aoc_std::{
    collections::CharGrid,
    directions::{BoundedCardinalNeighbors, Cardinal},
    geometry::Location,
};

#[derive(Debug, Clone)]
pub struct WarehouseWoes {
    grid: CharGrid,
    wide_grid: CharGrid,
    start: Location,
    movements: Vec<Cardinal>,
}

impl FromStr for WarehouseWoes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .trim()
            .split_once("\n\n")
            .ok_or_else(|| anyhow!("invalid input"))?;

        let mut grid = CharGrid::from_str(left)?;

        let mut start = Location::default();

        'outer: for r in 0..grid.height() {
            for c in 0..grid.width() {
                if grid.locations[r][c] == '@' {
                    start = Location::new(r, c);
                    grid.locations[r][c] = '.';
                    break 'outer;
                }
            }
        }

        let mut wide_grid = CharGrid::default_with_dimensions(grid.width() * 2, grid.height());

        for r in 0..grid.height() {
            for c in 0..grid.width() {
                let wc = c * 2;
                match grid.locations[r][c] {
                    '#' => {
                        wide_grid.locations[r][wc] = '#';
                        wide_grid.locations[r][wc + 1] = '#';
                    }
                    'O' => {
                        wide_grid.locations[r][wc] = '[';
                        wide_grid.locations[r][wc + 1] = ']';
                    }
                    _ => {
                        wide_grid.locations[r][wc] = '.';
                        wide_grid.locations[r][wc + 1] = '.';
                    }
                }
            }
        }

        let movements = right
            .lines()
            .flat_map(|line| line.chars())
            .map(|c| match c {
                '<' => Cardinal::West,
                '>' => Cardinal::East,
                '^' => Cardinal::North,
                'v' => Cardinal::South,
                _ => panic!("what is this {}", c),
            })
            .collect();

        Ok(Self {
            grid,
            wide_grid,
            start,
            movements,
        })
    }
}

impl WarehouseWoes {
    pub fn rearrange(&mut self) -> usize {
        let mut pos = self.start;

        for m in 0..self.movements.len() {
            let dir = self.movements[m];
            if self.maybe_shift_boxes(&pos, dir) {
                pos = pos.cardinal_neighbor(dir).unwrap();
            }
        }

        let mut out = 0;

        for r in 0..self.grid.height() {
            for c in 0..self.grid.width() {
                if self.grid.locations[r][c] == 'O' {
                    out += r * 100 + c;
                }
            }
        }

        out
    }

    pub fn rearrange_wide(&mut self) -> usize {
        let mut pos = self.start;
        pos.col *= 2;

        for m in 0..self.movements.len() {
            let dir = self.movements[m];
            if self.maybe_shift_wide_boxes(&pos, dir) {
                pos = pos.cardinal_neighbor(dir).unwrap();
            }
        }

        let mut out = 0;

        for r in 0..self.wide_grid.height() {
            for c in 0..self.wide_grid.width() {
                if self.wide_grid.locations[r][c] == '[' {
                    out += r * 100 + c;
                }
            }
        }

        out
    }

    fn maybe_shift_boxes(&mut self, loc: &Location, direction: Cardinal) -> bool {
        if let Some((nloc, nch)) = self.grid.cardinal_neighbor(loc, direction) {
            match nch {
                '.' => return true,
                'O' => {
                    if self.maybe_shift_boxes(&nloc, direction) {
                        let bloc = nloc.cardinal_neighbor(direction).unwrap();
                        self.grid.set(&bloc, 'O').unwrap();
                        self.grid.set(&nloc, '.').unwrap();
                        return true;
                    }
                }
                _ => return false,
            }
        }
        false
    }

    fn maybe_shift_wide_boxes(&mut self, loc: &Location, direction: Cardinal) -> bool {
        if let Some((nloc, nch)) = self.wide_grid.cardinal_neighbor(loc, direction) {
            match nch {
                '.' => return true,
                '[' => {
                    match direction {
                        // easy
                        Cardinal::East => {
                            if self
                                .maybe_shift_east(&nloc.cardinal_neighbor(Cardinal::East).unwrap())
                            {
                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row][nloc.col + 1] = '[';
                                self.wide_grid.locations[nloc.row][nloc.col + 2] = ']';
                                return true;
                            }
                        }

                        Cardinal::North => {
                            let right = nloc.cardinal_neighbor(Cardinal::East).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_north(&nloc, &right, &mut seen) {
                                for s in seen {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col + 1] = ']';
                                }

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row - 1][nloc.col] = '[';

                                self.wide_grid.locations[right.row][right.col] = '.';
                                self.wide_grid.locations[right.row - 1][right.col] = ']';
                                return true;
                            }
                        }
                        Cardinal::South => {
                            let right = nloc.cardinal_neighbor(Cardinal::East).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_south(&nloc, &right, &mut seen) {
                                for s in seen.iter().rev() {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col + 1] = ']';
                                }

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row + 1][nloc.col] = '[';

                                self.wide_grid.locations[right.row][right.col] = '.';
                                self.wide_grid.locations[right.row + 1][right.col] = ']';
                                return true;
                            }
                        }
                        // should ot be possible
                        Cardinal::West => unreachable!(),
                    }
                }
                ']' => {
                    match direction {
                        // easy
                        Cardinal::West => {
                            if self
                                .maybe_shift_west(&nloc.cardinal_neighbor(Cardinal::West).unwrap())
                            {
                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row][nloc.col - 1] = ']';
                                self.wide_grid.locations[nloc.row][nloc.col - 2] = '[';
                                return true;
                            }
                        }

                        Cardinal::North => {
                            let left = nloc.cardinal_neighbor(Cardinal::West).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_north(&left, &nloc, &mut seen) {
                                for s in seen {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row - 1][s.col + 1] = ']';
                                }

                                self.wide_grid.locations[left.row][left.col] = '.';
                                self.wide_grid.locations[left.row - 1][left.col] = '[';

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row - 1][nloc.col] = ']';
                                return true;
                            }
                        }
                        Cardinal::South => {
                            let left = nloc.cardinal_neighbor(Cardinal::West).unwrap();
                            let mut seen = BTreeSet::new();
                            if self.maybe_shift_south(&left, &nloc, &mut seen) {
                                for s in seen.iter().rev() {
                                    self.wide_grid.locations[s.row][s.col] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col] = '[';

                                    self.wide_grid.locations[s.row][s.col + 1] = '.';
                                    self.wide_grid.locations[s.row + 1][s.col + 1] = ']';
                                }
                                self.wide_grid.locations[left.row][left.col] = '.';
                                self.wide_grid.locations[left.row + 1][left.col] = '[';

                                self.wide_grid.locations[nloc.row][nloc.col] = '.';
                                self.wide_grid.locations[nloc.row + 1][nloc.col] = ']';
                                return true;
                            }
                        }
                        // should ot be possible
                        Cardinal::East => unreachable!(),
                    }
                }
                _ => return false,
            }
        }
        false
    }

    fn maybe_shift_east(&mut self, loc: &Location) -> bool {
        if let Some((nloc, nch)) = self.wide_grid.cardinal_neighbor(loc, Cardinal::East) {
            match nch {
                '.' => return true,
                '[' => {
                    if self.maybe_shift_east(&nloc.cardinal_neighbor(Cardinal::East).unwrap()) {
                        self.wide_grid.locations[nloc.row][nloc.col] = '.';
                        self.wide_grid.locations[nloc.row][nloc.col + 1] = '[';
                        self.wide_grid.locations[nloc.row][nloc.col + 2] = ']';
                        return true;
                    }
                }
                _ => return false,
            }
        }
        false
    }

    fn maybe_shift_west(&mut self, loc: &Location) -> bool {
        if let Some((nloc, nch)) = self.wide_grid.cardinal_neighbor(loc, Cardinal::West) {
            match nch {
                '.' => return true,
                ']' => {
                    if self.maybe_shift_west(&nloc.cardinal_neighbor(Cardinal::West).unwrap()) {
                        self.wide_grid.locations[nloc.row][nloc.col] = '.';
                        self.wide_grid.locations[nloc.row][nloc.col - 1] = ']';
                        self.wide_grid.locations[nloc.row][nloc.col - 2] = '[';
                        return true;
                    }
                }
                _ => return false,
            }
        }
        false
    }

    fn maybe_shift_north(
        &mut self,
        left: &Location,
        right: &Location,
        seen: &mut BTreeSet<Location>,
    ) -> bool {
        if let (Some((l_loc, l_ch)), Some((r_loc, r_ch))) = (
            self.wide_grid.cardinal_neighbor(left, Cardinal::North),
            self.wide_grid.cardinal_neighbor(right, Cardinal::North),
        ) {
            match (l_ch, r_ch) {
                ('.', '.') => return true,
                ('[', ']') => {
                    seen.insert(l_loc);
                    return self.maybe_shift_north(&l_loc, &r_loc, seen);
                }
                (']', '.') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();
                    seen.insert(left);
                    return self.maybe_shift_north(&left, &l_loc, seen);
                }
                ('.', '[') => {
                    seen.insert(r_loc);
                    return self.maybe_shift_north(
                        &r_loc,
                        &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                        seen,
                    );
                }
                (']', '[') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();
                    seen.insert(left);
                    seen.insert(r_loc);
                    return self.maybe_shift_north(&left, &l_loc, seen)
                        && self.maybe_shift_north(
                            &r_loc,
                            &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                            seen,
                        );
                }
                _ => return false,
            }
        }
        false
    }

    fn maybe_shift_south(
        &mut self,
        left: &Location,
        right: &Location,
        seen: &mut BTreeSet<Location>,
    ) -> bool {
        if let (Some((l_loc, l_ch)), Some((r_loc, r_ch))) = (
            self.wide_grid.cardinal_neighbor(left, Cardinal::South),
            self.wide_grid.cardinal_neighbor(right, Cardinal::South),
        ) {
            match (l_ch, r_ch) {
                ('.', '.') => return true,
                ('[', ']') => {
                    seen.insert(l_loc);
                    return self.maybe_shift_south(&l_loc, &r_loc, seen);
                }
                (']', '.') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();
                    seen.insert(left);
                    return self.maybe_shift_south(&left, &l_loc, seen);
                }
                ('.', '[') => {
                    seen.insert(r_loc);
                    return self.maybe_shift_south(
                        &r_loc,
                        &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                        seen,
                    );
                }
                (']', '[') => {
                    let left = l_loc.cardinal_neighbor(Cardinal::West).unwrap();
                    seen.insert(left);
                    seen.insert(r_loc);
                    return self.maybe_shift_south(&left, &l_loc, seen)
                        && self.maybe_shift_south(
                            &r_loc,
                            &r_loc.cardinal_neighbor(Cardinal::East).unwrap(),
                            seen,
                        );
                }
                _ => return false,
            }
        }
        false
    }
}

impl Problem for WarehouseWoes {
    const DAY: usize = 15;
    const TITLE: &'static str = "warehouse woes";
    const README: &'static str = include_str!("../README.md");

    type ProblemError = anyhow::Error;
    type P1 = usize;
    type P2 = usize;

    fn part_one(&mut self) -> Result<Self::P1, Self::ProblemError> {
        Ok(self.rearrange())
    }

    fn part_two(&mut self) -> Result<Self::P2, Self::ProblemError> {
        Ok(self.rearrange_wide())
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
        let solution = WarehouseWoes::solve(&input).unwrap();
        assert_eq!(solution, Solution::new(1421727, 1463160));
    }

    #[test]
    fn example() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let solution = WarehouseWoes::solve(input).unwrap();
        assert_eq!(solution, Solution::new(10092, 9021));
    }

    // #[test]
    // fn example2() {
    //     let input = "#######
    // #...#.#
    // #.....#
    // #..OO@#
    // #..O..#
    // #.....#
    // #######

    // <vv<<^^<<^^";
    //     let solution = WarehouseWoes::solve(input).unwrap();
    //     assert_eq!(solution, Solution::new(10092, 9021));
    // }
}
