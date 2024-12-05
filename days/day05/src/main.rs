use std::collections::HashSet;

static INPUT: &str = include_str!("../../../input/day05");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (rules, updates) = parse(input);

    let correct = updates.iter().filter(|u| {
        for (i, p) in u.iter().enumerate() {
            for x in &u[..i] {
                if rules.contains(&(*p, *x)) {
                    return false;
                }
            }
            for x in &u[i..] {
                if rules.contains(&(*x, *p)) {
                    return false;
                }
            }
        }

        true
    });

    correct.map(|u| u[(u.len() - 1) / 2]).sum()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, Vec<Vec<usize>>) {
    let (rules, updates) = input.split_once("\n\n").unwrap();

    let rules = rules
        .lines()
        .map(|l| {
            let (l, r) = l.split_once('|').unwrap();
            (l.parse().unwrap(), r.parse().unwrap())
        })
        .collect();

    let updates = updates
        .lines()
        .map(|l| l.split(',').filter_map(|s| s.parse().ok()).collect())
        .collect();

    (rules, updates)
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 143);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
