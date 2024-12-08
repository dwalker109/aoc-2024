use rustc_hash::{FxHashMap, FxHashSet};
use std::ops::{Add, Range, Sub};

static INPUT: &str = include_str!("../../../input/day08");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let map = Map::from(input);
    let mut antinodes = FxHashSet::default();

    for (_, ant) in map.antennae {
        for (a_idx, a) in ant.iter().enumerate() {
            for (b_idx, b) in ant.iter().enumerate().filter(|&(b_idx, ..)| a_idx != b_idx) {
                let diff = *a - *b;
                let loc = *a + diff;
                if map.width.contains(&loc.0) && map.height.contains(&loc.1) {
                    antinodes.insert(loc);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

struct Map {
    width: Range<isize>,
    height: Range<isize>,
    antennae: FxHashMap<char, Vec<Xy>>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let width = 0..input.lines().next().unwrap().len() as isize;
        let height = 0..input.lines().count() as isize;

        let mut antennae = FxHashMap::<char, Vec<Xy>>::default();
        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
                antennae
                    .entry(c)
                    .or_default()
                    .push(Xy(x as isize, y as isize));
            }
        }

        Self {
            width,
            height,
            antennae,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Xy(isize, isize);

impl Add for Xy {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Xy(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Xy {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Xy(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 14);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
