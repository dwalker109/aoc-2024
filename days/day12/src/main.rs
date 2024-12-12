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

    todo!()
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Xy(isize, isize);

impl Xy {
    fn adj(&self) -> [Self; 4] {
        let Xy(x, y) = self;

        [Xy(x - 1, *y), Xy(*x, y - 1), Xy(x + 1, *y), Xy(*x, y + 1)]
    }

    fn is_adj(&self, other: &Xy) -> bool {
        let Xy(xs, ys) = self;
        let Xy(xo, yo) = other;

        xs.abs_diff(*xo) == 1 && ys.abs_diff(*yo) == 1
    }
}

#[derive(Debug)]
struct Plots {
    by_xy: FxHashMap<Xy, char>,
    by_plant: FxHashMap<char, FxHashSet<Xy>>,
}

impl From<&str> for Plots {
    fn from(value: &str) -> Self {
        let by_xy = value
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(move |(x, c)| (Xy(x as isize, y as isize), c))
            })
            .collect::<FxHashMap<_, _>>();

        let by_plant = by_xy.iter().fold(
            FxHashMap::<char, FxHashSet<Xy>>::default(),
            |mut acc, (xy, c)| {
                acc.entry(*c).or_default().insert(*xy);
                acc
            },
        );

        Self { by_xy, by_plant }
    }
}

impl Plots {
    fn regions(&self) -> Vec<FxHashSet<Xy>> {
        let mut results = Vec::new();

        for mut plots in self.by_plant.values().cloned() {
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
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
