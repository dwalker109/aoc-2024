static INPUT: &str = include_str!("../../../input/day01");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (mut l, mut r) = parse(input);

    l.sort_unstable();
    r.sort_unstable();

    l.iter().zip(r.iter()).map(|(l, r)| l.abs_diff(*r)).sum()
}

fn part2(input: &'static str) -> Answer {
    let (l, r) = parse(input);

    l.iter()
        .map(|x| x * r.iter().filter(|y| x == *y).count())
        .sum()
}

fn parse(input: &'static str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|l| {
            let (l, r) = l.split_once("   ").unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        })
        .unzip()
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 11)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 31);
    }
}
