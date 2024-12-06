use crate::pos::{Move, Xy};
use rustc_hash::FxHashSet;
use std::ops::Range;

static INPUT: &str = include_str!("../../../input/day06");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let map = Map::from(input);
    let mut guard = guard::SingleGuard::from(input);

    guard.r#move(&map)
}

fn part2(input: &'static str) -> Answer {
    // let map = Map::from(input);
    // let mut guard = guard::ParaGuard::from(input);
    //
    // guard.r#move(&map);
    //
    todo!();
}

struct Map {
    hor: Range<isize>,
    ver: Range<isize>,
    obs: FxHashSet<Xy>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let hor = 0..input.lines().next().unwrap().chars().count() as isize;
        let ver = 0..input.lines().count() as isize;
        let obs = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| (c == '#').then_some(Xy::from((x, y))))
            })
            .collect();

        Self { hor, ver, obs }
    }
}

impl Map {
    pub fn oob(&self, r#move: &Move) -> bool {
        !self.hor.contains(&r#move.0 .0) || !self.ver.contains(&r#move.0 .1)
    }

    pub fn is_blocked(&self, r#move: &Move) -> bool {
        self.obs.contains(&r#move.0)
    }
}

mod pos {
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Dir {
        U,
        R,
        D,
        L,
    }

    impl From<char> for Dir {
        fn from(input: char) -> Self {
            match input {
                '^' => Self::U,
                '>' => Self::R,
                'v' => Self::D,
                '<' => Self::L,
                _ => panic!(),
            }
        }
    }
    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Xy(pub isize, pub isize);

    impl From<(usize, usize)> for Xy {
        fn from((x, y): (usize, usize)) -> Self {
            Self(x as isize, y as isize)
        }
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Move(pub Xy, pub Dir);

    impl Move {
        pub fn next(&self) -> Self {
            let Move((Xy(x, y)), dir) = self;

            match dir {
                Dir::U => Self(Xy(*x, y - 1), *dir),
                Dir::R => Self(Xy(x + 1, *y), *dir),
                Dir::D => Self(Xy(*x, y + 1), *dir),
                Dir::L => Self(Xy(x - 1, *y), *dir),
            }
        }

        pub fn right(&self) -> Self {
            let Move(pos, dir) = self;

            match dir {
                Dir::U => Self(*pos, Dir::R),
                Dir::R => Self(*pos, Dir::D),
                Dir::D => Self(*pos, Dir::L),
                Dir::L => Self(*pos, Dir::U),
            }
        }
    }
}

mod guard {
    use super::*;
    use crate::pos::{Dir, Move, Xy};

    fn get_start(input: &str) -> Move {
        for (y, l) in input.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '^' | '>' | 'v' | '<' => {
                        return Move(Xy::from((x, y)), Dir::from(c));
                    }
                    _ => continue,
                }
            }
        }

        unreachable!();
    }

    pub struct SingleGuard {
        curr: Move,
        hist: FxHashSet<Xy>,
    }

    impl From<&str> for SingleGuard {
        fn from(input: &str) -> Self {
            Self {
                curr: get_start(input),
                hist: FxHashSet::default(),
            }
        }
    }

    impl SingleGuard {
        pub fn r#move(&mut self, map: &Map) -> usize {
            while !map.oob(&self.curr) {
                let curr = self.curr;
                let next = curr.next();

                if map.is_blocked(&next) {
                    self.curr = curr.right();
                } else {
                    self.hist.insert(curr.0);
                    self.curr = next;
                };
            }

            self.hist.len()
        }
    }

    // pub struct ParaGuard {
    //     curr: (Xy, Dir),
    //     hist: FxHashSet<(Xy, Dir)>,
    //     visited: FxHashSet<Xy>,
    // }
    //
    // impl From<&str> for ParaGuard {
    //     fn from(input: &str) -> Self {
    //         Self {
    //             curr: get_start(input),
    //             hist: FxHashSet::default(),
    //             visited: FxHashSet::default(),
    //         }
    //     }
    // }
    //
    // impl ParaGuard {
    //     pub fn r#move(&mut self, map: &Map) {
    //         while !map.oob(&self.curr.0) {}
    //     }
    // }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 41);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 6);
    }
}
