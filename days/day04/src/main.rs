use rustc_hash::FxHashMap;

static INPUT: &str = include_str!("../../../input/day04");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let board = Board::from(input);
    let word = &['X', 'M', 'A', 'S'];

    board.0.keys().fold(0, |acc, xy| {
        acc + [
            Dir::UL,
            Dir::U,
            Dir::UR,
            Dir::L,
            Dir::R,
            Dir::DL,
            Dir::D,
            Dir::DR,
        ]
        .iter()
        .fold(0, |acc, dir| acc + board.go(xy, word, dir) as usize)
    })
}

fn part2(input: &'static str) -> Answer {
    let board = Board::from(input);
    let mas = &['M', 'A', 'S'];
    let sam = &['S', 'A', 'M'];

    board.0.keys().fold(0, |acc, xy| {
        acc + ((board.go(xy, mas, &Dir::DR) || board.go(xy, sam, &Dir::DR))
            && (board.go(&xy.off_2(), mas, &Dir::DL) || board.go(&xy.off_2(), sam, &Dir::DL)))
            as usize
    })
}

#[derive(Eq, PartialEq, Hash)]
struct Xy(isize, isize);

impl Xy {
    fn off_2(&self) -> Xy {
        let Xy(x, y) = *self;

        Xy(x + 2, y)
    }
}

enum Dir {
    UL,
    U,
    UR,
    L,
    R,
    DL,
    D,
    DR,
}

impl Dir {
    fn next(&self, curr: &Xy) -> Xy {
        let Xy(x, y) = *curr;
        match self {
            Dir::UL => Xy(x - 1, y - 1),
            Dir::U => Xy(x, y - 1),
            Dir::UR => Xy(x + 1, y - 1),
            Dir::L => Xy(x - 1, y),
            Dir::R => Xy(x + 1, y),
            Dir::DL => Xy(x - 1, y + 1),
            Dir::D => Xy(x, y + 1),
            Dir::DR => Xy(x + 1, y + 1),
        }
    }
}

struct Board(FxHashMap<Xy, char>);

impl Board {
    fn go(&self, curr: &Xy, letters: &[char], dir: &Dir) -> bool {
        let Some((target_letter, rest)) = letters.split_first() else {
            // No letters remain to find, increment
            return true;
        };

        let Some(curr_letter) = self.0.get(curr) else {
            // Current Xy is out of bound, not a match
            return false;
        };

        if curr_letter == target_letter {
            // Letters match, descend
            self.go(&dir.next(curr), rest, dir)
        } else {
            // Letters don't match
            false
        }
    }
}

impl From<&str> for Board {
    fn from(input: &str) -> Board {
        let grid = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(move |(x, char)| (Xy(x as isize, y as isize), char))
            })
            .collect();

        Self(grid)
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 9);
    }
}
