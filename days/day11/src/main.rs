use rustc_hash::FxHashMap;

static INPUT: &str = include_str!("../../../input/day11");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    solve::<25>(input)
}

fn part2(input: &'static str) -> Answer {
    solve::<75>(input)
}

fn parse(input: &str) -> Vec<Stone> {
    input
        .lines()
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(Stone::from)
        .collect()
}

fn solve<const N: u8>(input: &str) -> usize {
    let stones = parse(input);
    let mut stone_counts: FxHashMap<Stone, usize> = stones.iter().map(|s| (*s, 1usize)).collect();

    for _ in 0..N {
        let mut working: FxHashMap<Stone, usize> = FxHashMap::default();

        for (s, n) in stone_counts.iter() {
            for next in s.blink().into_iter() {
                let e = working.entry(next).or_default();
                *e += *n;
            }
        }

        stone_counts = working;
    }

    stone_counts.values().sum()
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
struct Stone(usize);

impl From<&str> for Stone {
    fn from(value: &str) -> Self {
        Self(value.parse().unwrap())
    }
}

impl Stone {
    fn blink(&self) -> Vec<Stone> {
        if self.0 == 0 {
            return vec![Stone(1)];
        }

        let len = self.0.checked_ilog10().unwrap_or(0) + 1;
        if len % 2 == 0 {
            let mut l = self.0;
            let mut r = 0;

            for i in 0..(len / 2) {
                r += (l % 10) * 10usize.pow(i);
                l /= 10;
            }

            return vec![Stone(l), Stone(r)];
        }

        return vec![Stone(self.0 * 2024)];
    }
}

#[cfg(test)]
mod tests {
    use crate::Stone;

    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 55312);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 65601038650482);
    }

    #[test]
    fn stones() {
        assert_eq!(Stone(0).blink(), vec![Stone(1)]);
        assert_eq!(Stone(1029).blink(), vec![Stone(10), Stone(29)]);
        assert_eq!(Stone(1000).blink(), vec![Stone(10), Stone(0)]);
        assert_eq!(Stone(1).blink(), vec![Stone(2024)]);
        assert_eq!(Stone(2).blink(), vec![Stone(4048)]);
    }
}
