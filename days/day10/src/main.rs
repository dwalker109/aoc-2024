use rustc_hash::{FxHashMap, FxHashSet};

static INPUT: &str = include_str!("../../../input/day10");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let map = Map::from(input);

    map.0
        .iter()
        .filter_map(|(xy, n)| (*n == 0).then_some(xy))
        .fold(0, |mut acc, xy| {
            let mut reached = FxHashSet::default();
            xy.score(None, &map, &mut reached);
            acc += reached.len();
            acc
        })
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

struct Map(FxHashMap<Xy, usize>);

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        Self(
            input
                .lines()
                .enumerate()
                .flat_map(|(y, l)| {
                    l.chars().enumerate().map(move |(x, c)| {
                        (Xy(x as isize, y as isize), c.to_digit(10).unwrap() as usize)
                    })
                })
                .collect(),
        )
    }
}

#[derive(Debug, Default)]
struct Cache(FxHashMap<Xy, Option<usize>>);

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Xy(isize, isize);

impl Xy {
    fn adj(&self) -> [Xy; 4] {
        let Xy(x, y) = self;
        [Xy(x - 1, *y), Xy(*x, y - 1), Xy(x + 1, *y), Xy(*x, y + 1)]
    }

    fn score(&self, prev: Option<&Xy>, map: &Map, reached: &mut FxHashSet<Xy>) -> usize {
        let Some(self_height) = map.0.get(self) else {
            return 0;
        };

        let mut scores = 0;

        match prev {
            Some(prev) => {
                let Some(prev_height) = map.0.get(prev) else {
                    return 0;
                };

                if self_height > prev_height && self_height.abs_diff(*prev_height) == 1 {
                    if *self_height == 9 {
                        reached.insert(*self);
                    }

                    for xy in self.adj() {
                        scores += xy.score(Some(self), map, reached);
                    }
                }
            }
            None => {
                for xy in self.adj() {
                    scores += xy.score(Some(self), map, reached);
                }
            }
        }

        scores
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 36);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
