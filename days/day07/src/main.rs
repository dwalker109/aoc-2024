use itertools::{repeat_n, Itertools};

static INPUT: &str = include_str!("../../../input/day07");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let input = parsing::parse(input);
    let ops = [Op::Add, Op::Mult];

    let truthy = input
        .iter()
        .filter(|(target, n)| {
            repeat_n(ops.iter().copied(), n.len() - 1)
                .multi_cartesian_product()
                .any(|cp| {
                    let mut sum = n.iter().map(|n| Op::Num(*n)).interleave(cp.iter().copied());

                    let init = sum.next().unwrap();
                    let result = sum
                        .tuples()
                        .fold(init, |acc, (first, second)| first.apply(&acc, &second));

                    *target == result.unwrap()
                })
        })
        .collect_vec();

    truthy.iter().map(|(n, _)| n).sum()
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult,
    Num(usize),
}

impl Op {
    fn apply(&self, l: &Op, r: &Op) -> Op {
        let Op::Num(l) = *l else {
            panic!("must be a number!");
        };
        let Op::Num(r) = *r else {
            panic!("must be a number!");
        };

        match self {
            Op::Add => Op::Num(l + r),
            Op::Mult => Op::Num(l * r),
            _ => panic!("must be add or mult!"),
        }
    }

    fn unwrap(&self) -> usize {
        match self {
            Op::Num(n) => *n,
            _ => panic!("must be a number!"),
        }
    }
}

mod parsing {
    use nom::{
        bytes::complete::tag,
        character::complete::{digit1, space1},
        combinator::map_res,
        multi::separated_list1,
        sequence::tuple,
        IResult,
    };

    pub fn parse(input: &str) -> Vec<(usize, Vec<usize>)> {
        input
            .lines()
            .map(|r| parse_line(r).unwrap())
            .map(|(_, r)| r)
            .collect()
    }

    fn parse_line(input: &str) -> IResult<&str, (usize, Vec<usize>)> {
        let (input, (target, _, nums)) = tuple((
            map_res(digit1, str::parse),
            tag(": "),
            separated_list1(space1, map_res(digit1, str::parse)),
        ))(input)?;

        Ok((input, (target, nums)))
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 3749);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
