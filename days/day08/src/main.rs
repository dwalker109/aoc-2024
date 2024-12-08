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

    explore(&map.antennae, |a, b| {
        let diff = *a - *b;
        let loc = *a + diff;
        if map.width.contains(&loc.0) && map.height.contains(&loc.1) {
            antinodes.insert(loc);
        }
    });

    antinodes.len()
}

fn part2(input: &'static str) -> Answer {
    let map = Map::from(input);
    let mut antinodes = FxHashSet::default();

    explore(&map.antennae, |a, b| {
        let diff = *a - *b;
        let mut loc = *a;
        while map.width.contains(&loc.0) && map.height.contains(&loc.1) {
            antinodes.insert(loc);
            loc = loc + diff;
        }
    });

    antinodes.len()
}

fn explore(antennae: &[Vec<Xy>], mut callback: impl FnMut(&Xy, &Xy)) {
    for antenna in antennae {
        for (a_idx, a) in antenna.iter().enumerate() {
            for (_, b) in antenna
                .iter()
                .enumerate()
                .filter(|&(b_idx, ..)| a_idx != b_idx)
            {
                callback(a, b);
            }
        }
    }
}

struct Map {
    width: Range<isize>,
    height: Range<isize>,
    antennae: Vec<Vec<Xy>>,
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
            antennae: antennae.into_values().collect(),
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
        assert_eq!(super::part2(INPUT), 34);
    }
}
