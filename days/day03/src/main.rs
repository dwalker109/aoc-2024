static INPUT: &str = include_str!("../../../input/day03");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let (_, result) = parsers::chomp_line("mul(2,2)").unwrap();

    result
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

mod parsers {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, digit1},
        combinator::{map_res, opt},
        multi::many0,
        sequence::{delimited, separated_pair},
        IResult,
    };

    fn instr(input: &str) -> IResult<&str, usize> {
        dbg!(input);
        let (input, (l, r)) = delimited(
            tag("mul("),
            separated_pair(
                map_res(digit1, str::parse::<usize>),
                char(','),
                map_res(digit1, str::parse::<usize>),
            ),
            char(')'),
        )(input)?;

        Ok((input, l * r))
    }

    pub fn chomp_line(input: &str) -> IResult<&str, usize> {
        let (input, results) = many0(opt(instr))(input)?;

        Ok((input, results.iter().flatten().sum()))
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 161);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }
}
