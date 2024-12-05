use std::collections::HashSet;

static INPUT: &str = include_str!("../../../input/day05");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (rules, updates) = parse(input);

    updates
        .iter()
        .filter(|u| is_correct(u, &rules))
        .map(|u| u[(u.len() - 1) / 2])
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let (rules, updates) = parse(input);

    let incorrect = updates
        .iter()
        .filter(|u| !is_correct(u, &rules))
        .collect::<Vec<_>>();

    dbg!(&incorrect);

    let mut results = Vec::new();
    for &update in incorrect.iter() {
        let mut curr = update.clone();
        let mut working = update.clone();

        loop {
            let mut stable = true;

            for (p_idx, p) in curr.iter().enumerate() {
                for (x_idx, x) in curr[..p_idx].iter().enumerate() {
                    if rules.contains(&(*p, *x)) {
                        dbg!("Swapping 1", p_idx, x_idx);
                        working.swap(p_idx, x_idx);
                        dbg!(&working);
                        continue;
                        // stable = false;
                    }
                }
                for (x_idx, x) in curr[p_idx..].iter().enumerate() {
                    if rules.contains(&(*x, *p)) {
                        dbg!("Swapping 2", p_idx, x_idx);
                        working.swap(p_idx, x_idx);
                        dbg!(&working);
                        continue;
                        // stable = false;
                    }
                }
            }

            if stable {
                results.push(curr);
                break;
            } else {
                curr = working.clone();
            }
        }
    }

    dbg!(&results);

    todo!()
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

fn is_correct(update: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    for (i, p) in update.iter().enumerate() {
        for x in &update[..i] {
            if rules.contains(&(*p, *x)) {
                return false;
            }
        }
        for x in &update[i..] {
            if rules.contains(&(*x, *p)) {
                return false;
            }
        }
    }

    true
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
