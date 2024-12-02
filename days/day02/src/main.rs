#![feature(array_windows)]

static INPUT: &str = include_str!("../../../input/day02");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    parse(input).iter().filter(|r| is_safe(r)).count()
}

fn part2(input: &'static str) -> Answer {
    parse(input)
        .iter()
        .map(|l| {
            (0..l.len())
                .map(|n| {
                    l[..n]
                        .iter()
                        .chain(l[n + 1..].iter())
                        .cloned()
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .filter(|v| v.iter().any(|r| is_safe(&r)))
        .count()
}

fn parse(input: &'static str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(str::parse::<usize>)
                .flatten()
                .collect()
        })
        .collect()
}

fn is_safe(r: &[usize]) -> bool {
    (r.array_windows().all(|&[a, b]| a > b) || r.array_windows().all(|&[a, b]| a < b))
        && r.array_windows()
            .all(|&[a, b]| (1..=3).contains(&a.abs_diff(b)))
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 4);
    }
}
