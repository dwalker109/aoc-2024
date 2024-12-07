use crate::parsing::ParsedInput;
use itertools::{repeat_n, Itertools};
use std::collections::{HashMap, HashSet};
use std::ops::ControlFlow;

static INPUT: &str = include_str!("../../../input/day07");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let input = parsing::parse(input);
    let cp = gen_cp(&input, &[Op::Add, Op::Mult]);

    input
        .iter()
        .filter(|(target, n)| is_truthy(target, n, &cp))
        .map(|(n, _)| n)
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let input = parsing::parse(input);
    let cp = gen_cp(&input, &[Op::Add, Op::Mult, Op::Concat]);

    input
        .iter()
        .filter(|(target, n)| is_truthy(target, n, &cp))
        .map(|(n, _)| n)
        .sum()
}

fn gen_cp(input: &ParsedInput, ops: &[Op]) -> HashMap<usize, Vec<Vec<Op>>> {
    let i_range = input
        .iter()
        .map(|(_, n)| n.len())
        .collect::<HashSet<usize>>();

    i_range
        .iter()
        .map(|i| {
            (
                *i,
                repeat_n(ops.iter().copied(), i - 1)
                    .multi_cartesian_product()
                    .collect(),
            )
        })
        .collect()
}

fn is_truthy(target: &usize, n: &[usize], cp: &HashMap<usize, Vec<Vec<Op>>>) -> bool {
    cp.get(&n.len()).unwrap().iter().any(|cp| {
        let mut sum = n.iter().map(|n| Op::Num(*n)).interleave(cp.iter().copied());

        let init = sum.next().unwrap();
        let result = sum.tuples().try_fold(init, |acc, (first, second)| {
            let next = first.apply(&acc, &second);
            if next.unwrap() > *target {
                ControlFlow::Break(Op::Num(0))
            } else {
                ControlFlow::Continue(next)
            }
        });

        match result {
            ControlFlow::Continue(op) => *target == op.unwrap(),
            ControlFlow::Break(_) => false,
        }
    })
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult,
    Concat,
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
            // See https://www.reddit.com/r/rust/comments/191l3ot
            Op::Concat => Op::Num(l * 10usize.pow(r.ilog10() + 1) + r),
            _ => panic!("must be add or mult or concat!"),
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

    pub type ParsedInput = Vec<(usize, Vec<usize>)>;

    pub fn parse(input: &str) -> ParsedInput {
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
        assert_eq!(super::part2(INPUT), 11387);
    }
}
