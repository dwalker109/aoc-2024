use itertools::{repeat_n, Itertools};
use rustc_hash::FxHashMap;

static INPUT: &str = include_str!("../../../input/day07");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    go(input, &[Op::Add, Op::Mult])
}

fn part2(input: &'static str) -> Answer {
    go(input, &[Op::Add, Op::Mult, Op::Concat])
}

fn go(input: &str, ops: &[Op]) -> usize {
    let parsed = parsing::parse(input);
    let mut cp = OpsCartesianProductCache::default();

    parsed
        .iter()
        .filter(|(target, n)| is_truthy(target, n, ops, &mut cp))
        .map(|(n, _)| n)
        .sum()
}

#[derive(Default)]
struct OpsCartesianProductCache(FxHashMap<usize, Vec<Vec<Op>>>);

impl OpsCartesianProductCache {
    fn get(&mut self, len: &usize, ops: &[Op]) -> &[Vec<Op>] {
        self.0.entry(*len).or_insert_with(|| {
            repeat_n(ops.iter().copied(), len - 1)
                .multi_cartesian_product()
                .collect()
        })
    }
}

fn is_truthy(target: &usize, n: &[usize], ops: &[Op], cp: &mut OpsCartesianProductCache) -> bool {
    cp.get(&n.len(), ops).iter().any(|cp| {
        let sum = n
            .iter()
            .rev() // Goes in reverse, which allows for quicker aborting
            .map(|n| Op::Num(*n))
            .interleave(cp.iter().copied());

        let result = sum
            .tuples()
            .try_fold(Op::Num(*target), |acc, (a, b)| b.try_apply(&acc, &a));

        match result {
            None => false,
            Some(x) => x.unwrap() == *n.first().unwrap(),
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
    /// try_apply operates on the sum right to left, so * are divides, + are subtracts, || are splits
    fn try_apply(&self, l: &Op, r: &Op) -> Option<Op> {
        let Op::Num(l) = *l else {
            panic!("must be a number!");
        };
        let Op::Num(r) = *r else {
            panic!("must be a number!");
        };

        match self {
            Op::Add => l.checked_sub(r).map(Op::Num),
            Op::Mult => (l % r == 0).then(|| Op::Num(l / r)),
            Op::Concat => {
                let mut l = l;
                let mut r = r;

                while r != 0 {
                    let last_l_digit = l % 10;
                    let last_r_digit = r % 10;

                    if last_l_digit == last_r_digit {
                        l /= 10;
                        r /= 10;
                    } else {
                        return None;
                    }
                }

                Some(Op::Num(l))
            }
            _ => panic!("cannot apply this op"),
        }
    }

    fn unwrap(&self) -> usize {
        match self {
            Op::Num(n) => *n,
            _ => panic!("must be a number"),
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
        assert_eq!(super::part2(INPUT), 11387);
    }
}
