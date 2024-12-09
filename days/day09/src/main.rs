use itertools::{repeat_n, Itertools};

static INPUT: &str = include_str!("../../../input/day09");

type Answer = usize;

fn main() {
    aoc_shared::runner::solve(|| part1(INPUT), || part2(INPUT))
}

fn part1(input: &'static str) -> Answer {
    let fs = parse(input);

    let mut map = fs.fold(Vec::new(), |mut acc, curr| {
        let (val, size) = match curr {
            BlockGroup::Free(size) => (None, size as usize),
            BlockGroup::File { size, id } => (Some(id), size as usize),
        };
        acc.splice(acc.len().., repeat_n(val, size));
        acc
    });

    let mut data = map
        .iter()
        .enumerate()
        .filter_map(|(i, el)| el.is_some().then_some((i, *el)))
        .collect_vec();

    let map_len = map.len();
    let highest_data_end_idx = map.iter().filter(|el| el.is_some()).count();
    for idx in 0..map_len {
        if idx < highest_data_end_idx && map[idx].is_none() {
            let (data_end_idx, ..) = data.pop().unwrap();
            map.swap(idx, data_end_idx);
        }
    }

    map.iter()
        .enumerate()
        .map(|(i, n)| i * n.unwrap_or_default())
        .sum()
}

fn part2(input: &'static str) -> Answer {
    let fs = parse(input).collect::<Vec<_>>();

    let mut source = fs
        .iter()
        .enumerate()
        .filter_map(|(i, b)| matches!(b, BlockGroup::File { .. }).then_some((i, *b)))
        .collect_vec();

    let mut target = fs;

    while let Some((src_blk_idx, src_blk)) = source.pop() {
        if let Some((free_idx, free_size)) = target
            .iter()
            .enumerate()
            .find(|(ti, t)| matches!(t, BlockGroup::Free(free_size) if *ti < src_blk_idx && *free_size >= src_blk.size()))
            .map(|(i, b)| (i, b.size()))
        {
            if src_blk_idx < free_idx {
                continue;
            }

            target[src_blk_idx] = BlockGroup::Free(src_blk.size());

            let excess_size = free_size - src_blk.size();
            if excess_size == 0 {
                target[free_idx] = src_blk;
            } else {
                target.splice(
                    free_idx..free_idx + 1,
                    [src_blk, BlockGroup::Free(excess_size)],
                );
                for x in source.iter_mut().filter(|(i, _)| *i > free_idx) {
                    x.0+=1;
                }
            }
        }
    }

    target
        .iter()
        .scan(0usize, |pos, curr| match curr {
            BlockGroup::Free(size) => {
                *pos += *size as usize;
                Some(0)
            }
            BlockGroup::File { size, id } => {
                let mut chk = 0;

                for i in *pos..*pos + *size as usize {
                    chk += i * id;
                }

                *pos += *size as usize;

                Some(chk)
            }
        })
        .sum()
}

fn parse(input: &str) -> impl Iterator<Item = BlockGroup> + '_ {
    let files = input.chars().enumerate().filter_map(|(i, c)| {
        (i % 2 == 0).then_some(BlockGroup::File {
            size: c.to_digit(10)?,
            id: i / 2,
        })
    });

    let free = input
        .chars()
        .enumerate()
        .filter_map(|(i, c)| (i % 2 != 0).then_some(BlockGroup::Free(c.to_digit(10)?)));

    files.interleave(free)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BlockGroup {
    Free(u32),
    File { size: u32, id: usize },
}

impl BlockGroup {
    fn size(&self) -> u32 {
        match self {
            BlockGroup::Free(size) => *size,
            BlockGroup::File { size, .. } => *size,
        }
    }
}

#[cfg(test)]
mod tests {
    static INPUT: &str = include_str!("../input_test");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 1928);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 2858);
    }
}
