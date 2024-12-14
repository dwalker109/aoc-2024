static INPUT: &str = include_str!("../../../input/day13");

type Answer = isize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let machines = parse::parse(input);
    machines.iter().filter_map(Machine::cost_to_win).sum()
}

fn part2(input: &'static str) -> Answer {
    let mut machines = parse::parse(input);
    machines.iter_mut().for_each(|m| {
        m.prize.0+=10000000000000;
        m.prize.1+=10000000000000;
    });
    machines.iter().filter_map(Machine::cost_to_win).sum()
}

#[derive(Debug)]
struct Machine {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
}

impl Machine {
    /// The linear algebra used here was way beyond me; this is adapted from
    /// https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
    fn cost_to_win(&self) -> Option<isize> {
        let Machine {
            button_a,
            button_b,
            prize,
        } = self;

        let presses_a = (prize.0 * button_b.1 - prize.1 * button_b.0)
            / (button_a.0 * button_b.1 - button_a.1 * button_b.0);
        let presses_b = (button_a.0 * prize.1 - button_a.1 * prize.0)
            / (button_a.0 * button_b.1 - button_a.1 * button_b.0);

        (presses_a * button_a.0 + presses_b * button_b.0 == prize.0
            && presses_a * button_a.1 + presses_b * button_b.1 == prize.1)
            .then(|| presses_a * 3 + presses_b)
    }
}

mod parse {
    use nom::{
        bytes::complete::{take, take_until},
        character::complete::{digit1, newline},
        combinator::{map_res, opt},
        sequence::tuple,
        IResult,
    };

    use crate::Machine;

    fn xy(input: &str) -> IResult<&str, (isize, isize)> {
        let (input, (_, _, x, _, _, y, _)) = tuple((
            take_until("X"),
            take(2usize),
            map_res(digit1, str::parse),
            take_until("Y"),
            take(2usize),
            map_res(digit1, str::parse),
            opt(newline),
        ))(input)?;

        Ok((input, (x, y)))
    }

    pub(crate) fn parse(input: &str) -> Vec<Machine> {
        input
            .split("\n\n")
            .map(|input| {
                let (input, button_a) = xy(input).unwrap();
                let (input, button_b) = xy(input).unwrap();
                let (_, prize) = xy(input).unwrap();

                Machine {
                    button_a,
                    button_b,
                    prize,
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 480);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 875318608908);
    }
}
