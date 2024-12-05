use rustc_hash::FxHashSet;

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

    updates
        .iter()
        .filter(|u| !is_correct(u, &rules))
        .map(|update| {
            let mut working = update.clone();

            loop {
                let mut settled = true;

                for p_idx in 0..working.len() {
                    let p = working[p_idx];

                    for x_idx in 0..p_idx {
                        let x = working[x_idx];
                        if rules.contains(&(p, x)) {
                            working.swap(p_idx, x_idx);
                            settled = false;
                        }
                    }

                    for x_idx in p_idx..working.len() {
                        let x = working[x_idx];
                        if rules.contains(&(x, p)) {
                            working.swap(p_idx, x_idx);
                            settled = false;
                        }
                    }
                }

                if settled {
                    return working;
                }
            }
        })
        .map(|u| u[(u.len() - 1) / 2])
        .sum()
}

fn parse(input: &str) -> (FxHashSet<(usize, usize)>, Vec<Vec<usize>>) {
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

fn is_correct(update: &[usize], rules: &FxHashSet<(usize, usize)>) -> bool {
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
        assert_eq!(super::part2(INPUT), 123);
    }
}
