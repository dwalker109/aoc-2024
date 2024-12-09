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

    let map_len = map.len();
    let highest_data_end_idx = map.iter().filter(|el| el.is_some()).count();
    for idx in 0..map_len {
        if idx < highest_data_end_idx && map[idx].is_none() {
            let (data_end_idx, ..) = map
                .iter()
                .rev()
                .enumerate()
                .find(|(_, el)| el.is_some())
                .unwrap();

            map.swap(idx, map_len - 1 - data_end_idx);
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
        .filter(|s| matches!(s, BlockGroup::File { .. }))
        .copied()
        .collect_vec();

    let mut target = fs.clone();

    while let Some(src_blk) = source.pop() {
        if let Some((free_idx, free_size)) = target
            .iter()
            .enumerate()
            .find(|(_, t)| matches!(t, BlockGroup::Free(free_size) if *free_size >= src_blk.size()))
            .map(|(i, b)| (i, b.size()))
        {
            let src_idx_in_target = target.len()
                - 1
                - target
                    .iter()
                    .rev()
                    .position(|t| matches!(t, &f @ BlockGroup::File { .. } if f == src_blk))
                    .unwrap();

            if src_idx_in_target < free_idx {
                continue;
            }

            target[src_idx_in_target] = BlockGroup::Free(src_blk.size());

            let excess_size = free_size - src_blk.size();
            if excess_size == 0 {
                target[free_idx] = src_blk;
            } else {
                target.splice(
                    free_idx..free_idx + 1,
                    [src_blk, BlockGroup::Free(excess_size)],
                );
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
