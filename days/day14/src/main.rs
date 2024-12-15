use std::ops::Range;

static INPUT: &str = include_str!("../../../input/day14");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1::<101, 103>(INPUT), || part2(INPUT))
}

fn part1<const W: i32, const H: i32>(input: &'static str) -> Answer {
    let mut robots = parse(input);
    robots.iter_mut().for_each(|r| r.r#move(100, (W, H)));

    quads::<W, H>()
        .iter()
        .map(|(w, h)| {
            robots
                .iter()
                .filter(|r| w.contains(&r.pos.0) && h.contains(&r.pos.1))
                .count()
        })
        .product()
}

const fn quads<const W: i32, const H: i32>() -> [(Range<i32>, Range<i32>); 4] {
    [
        (0..(W / 2), 0..(H / 2)),
        ((W / 2) + 1..W, 0..(H / 2)),
        (0..(W / 2), (H / 2) + 1..H),
        ((W / 2) + 1..W, (H / 2) + 1..H),
    ]
}

fn part2(input: &'static str) -> Answer {
    todo!();
}

#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

impl Robot {
    fn r#move(&mut self, steps: i32, room: (i32, i32)) {
        self.pos.0 = (self.pos.0 + self.vel.0 * steps).wrapping_rem_euclid(room.0);
        self.pos.1 = (self.pos.1 + self.vel.1 * steps).wrapping_rem_euclid(room.1);
    }
}

fn parse(input: &str) -> Vec<Robot> {
    use nom::{
        bytes::complete::{tag, take_till, take_until},
        character::{is_newline, is_space},
        combinator::map_res,
        sequence::{preceded, separated_pair},
        IResult,
    };

    fn r#do(input: &str) -> Robot {
        fn coords(input: &str) -> IResult<&str, (i32, i32)> {
            separated_pair(
                map_res(take_until(","), str::parse),
                tag(","),
                map_res(
                    take_till(|c| is_space(c as u8) || is_newline(c as u8)),
                    str::parse,
                ),
            )(input)
        }

        let (input, pos) = (preceded(tag("p="), coords))(input).unwrap();
        let (_, vel) = (preceded(tag(" v="), coords))(input).unwrap();

        Robot { pos, vel }
    }

    input.lines().map(r#do).collect()
}

#[cfg(test)]
mod tests {
    use crate::Robot;

    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1::<11, 7>(INPUT), 12);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), super::Answer::default());
    }

    #[test]
    fn moving() {
        let mut r = Robot {
            pos: (2, 4),
            vel: (2, -3),
        };
        r.r#move(5, (11, 7));
        assert_eq!(r.pos, (1, 3));
    }
}
