use std::isize;

use itertools::{Itertools, MinMaxResult};
use rustc_hash::{FxHashMap, FxHashSet};

static INPUT: &str = include_str!("../../../input/day12");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let regions = Plots::from(input).regions();

    regions
        .iter()
        .map(|r| {
            let area = r.len();
            let mut perimeter = 0;

            for p in r {
                perimeter += 4 - p.adj().iter().filter(|op| r.contains(op)).count();
            }

            area * perimeter
        })
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let regions = Plots::from(input).regions();

    regions
        .iter()
        .map(|r| {
            let area = r.len();

            let (low_x, high_x) = match r.iter().minmax_by_key(|el| el.0) {
                MinMaxResult::NoElements => panic!(),
                MinMaxResult::OneElement(xy) => (xy.0, xy.0),
                MinMaxResult::MinMax(a, b) => (a.0, b.0),
            };

            let (low_y, high_y) = match r.iter().minmax_by_key(|el| el.1) {
                MinMaxResult::NoElements => panic!(),
                MinMaxResult::OneElement(xy) => (xy.1, xy.1),
                MinMaxResult::MinMax(a, b) => (a.1, b.1),
            };

            let mut sides = 0;

            for dir in [Cardinal::N, Cardinal::S] {
                for y in low_y..=high_y {
                    let mut dir_sides = 0;

                    for (Xy(a, _), Xy(b, _)) in r
                        .iter()
                        .filter(|el| el.1 == y)
                        .filter(|el| el.is_exposed(&dir, r))
                        .sorted()
                        .chain([Xy(isize::MAX, isize::MAX)].iter())
                        .tuple_windows()
                    {
                        if a.abs_diff(*b) > 1 {
                            dir_sides += 1;
                        }
                    }

                    sides += dir_sides;
                }
            }

            for dir in [Cardinal::E, Cardinal::W] {
                for x in low_x..=high_x {
                    let mut dir_sides = 0;

                    for (Xy(_, a), Xy(_, b)) in r
                        .iter()
                        .filter(|el| el.0 == x)
                        .filter(|el| el.is_exposed(&dir, r))
                        .sorted()
                        .chain([Xy(isize::MAX, isize::MAX)].iter())
                        .tuple_windows()
                    {
                        if a.abs_diff(*b) > 1 {
                            dir_sides += 1;
                        }
                    }

                    sides += dir_sides;
                }
            }

            area * sides
        })
        .sum()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Xy(isize, isize);

impl Xy {
    fn adj(&self) -> [Self; 4] {
        let Xy(x, y) = self;

        [Xy(x - 1, *y), Xy(*x, y - 1), Xy(x + 1, *y), Xy(*x, y + 1)]
    }

    fn is_exposed(&self, dir: &Cardinal, r: &FxHashSet<Xy>) -> bool {
        let other = match dir {
            Cardinal::N => Xy(self.0, self.1 - 1),
            Cardinal::E => Xy(self.0 + 1, self.1),
            Cardinal::S => Xy(self.0, self.1 + 1),
            Cardinal::W => Xy(self.0 - 1, self.1),
        };

        let found = r.contains(&other);

        !found
    }
}

enum Cardinal {
    N,
    E,
    S,
    W,
}

#[derive(Debug)]
struct Plots(FxHashMap<char, FxHashSet<Xy>>);

impl From<&str> for Plots {
    fn from(value: &str) -> Self {
        let by_plant = value
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| (Xy(x as isize, y as isize), c))
            })
            .fold(
                FxHashMap::<char, FxHashSet<Xy>>::default(),
                |mut acc, (xy, c)| {
                    acc.entry(c).or_default().insert(xy);
                    acc
                },
            );

        Self(by_plant)
    }
}

impl Plots {
    fn regions(&self) -> Vec<FxHashSet<Xy>> {
        let mut results = Vec::new();

        for mut plots in self.0.values().cloned() {
            while !plots.is_empty() {
                let mut region = FxHashSet::default();

                let first = *plots.iter().next().unwrap();
                plots.remove(&first);
                let mut stack = vec![first];

                while let Some(p) = stack.pop() {
                    region.insert(p);
                    for adj in p.adj() {
                        if plots.remove(&adj) {
                            stack.push(adj);
                        }
                    }
                }

                results.push(region);
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1930);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 1206);
    }
}
