static INPUT: &str = include_str!("../../../input/day03");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    parsers::chomp(input)
}

fn part2(input: &'static str) -> Answer {
    parsers::chomp_conditional(input)
}

mod parsers {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take, take_until},
        character::complete::{char, digit1},
        combinator::{map_res, rest},
        multi::{many0, many_till},
        sequence::{delimited, preceded, separated_pair},
        IResult,
    };

    fn instr(input: &str) -> IResult<&str, usize> {
        let (input, (_, (l, r))) = many_till(
            take(1usize),
            delimited(
                tag("mul("),
                separated_pair(
                    map_res(digit1, str::parse::<usize>),
                    char(','),
                    map_res(digit1, str::parse::<usize>),
                ),
                char(')'),
            ),
        )(input)?;

        Ok((input, l * r))
    }

    fn do_wrapped(input: &str) -> IResult<&str, &str> {
        let (input, (_, substr)) = many_till(
            take(1usize),
            delimited(tag("do()"), take_until("don't()"), tag("don't()")),
        )(input)?;

        Ok((input, substr))
    }

    fn do_preceded(input: &str) -> IResult<&str, &str> {
        let (input, (_, substr)) = many_till(take(1usize), preceded(tag("do()"), rest))(input)?;

        Ok((input, substr))
    }

    pub fn chomp(input: &str) -> usize {
        let (_, results) = many0(instr)(input).unwrap();

        results.iter().sum()
    }

    pub fn chomp_conditional(input: &str) -> usize {
        let input = &(String::from("do()") + input);
        let (_, filtered_input) = many0(alt((do_wrapped, do_preceded)))(input).unwrap();

        chomp(&filtered_input.join(""))
    }
}

#[cfg(test)]
mod tests {
    static INPUT_1: &str = include_str!("../input_test_1");
    static INPUT_2: &str = include_str!("../input_test_2");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT_1), 161);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT_2), 48);
    }
}
