static INPUT: &str = include_str!("../../../input/day00");

type Answer = &'static str;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    input
}

fn part2(input: &'static str) -> Answer {
    input
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), "Hello, Test World!\n");
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), "Hello, Test World!\n");
    }
}
