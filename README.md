# Advent of Code 2024 !!! 

Well, here we are again. Another year, another promise to myself that I'll respect my own
time and bail out once I get tired of it. We'll see how that goes. I'm using Rust again, 
because it's fun, and it's where I'm most productive. Deps are fine. 

Solutions all expect inputs to be found in the `./input` directory, at the project root. Its
contents are stored in a private submodule, to respect the requirement not to publish inputs.

Timings come from a run in GitHub Actions.

## Day Setup

- ensure the `aocday` template and `aocfin` input downloaded are setup 
  (see [aoc-shared](https://github.com/dwalker109/aoc-shared)).
- `cd days && cargo generate aocday --name dayXX`
- `cd input && aocfin -O dayXX 2024 XX`

## Day 1: Historian Hysteria

Not hard but I haven't done any AOC since late 2023 so I was a bit rusty. Took
longer that I would expect it to. Went with a simple pair of lists in the end.
Part 2 could be optimised by pre-calculating frequency of occurrences, but since
a naive approach (just count the list every time) take less than a millisecond, 
I just stayed basic. Maybe this will bother me and I'll come back later, who knows.

### Timings

```
Part 1: 2176849 (81.833µs)
Part 2: 23384288 (371.131µs)
```

## Day 2: Red-Nosed Reports

Fairly simple really, and part one didn't take long. Part two confused me a
surprising amount, but ultimately it just came down to pre-computing a 
number of different variants to test against. Most of my issues here were
related to me trying to avoid allocations and getting tied in knots of nested
iterator adapters.

### Timings

```
Part 1: 639 (228.246µs)
Part 2: 674 (441.274µs)
```

## Day 3: Mull It Over

I decided I didn't want to use a regex for this, and I didn't want to hand parse
either. So I decided to use [nom](https://docs.rs/nom/latest/nom/c) instead. It is
a great tool, which I have used for some low level message parsing before. But
I haven't used it for over a year, and I forgot some of the basics, and I really tied
myself in knots. That said, I am reasonably happy with where I ended up and it's
pretty fast.

### Timings

```
Part 1: 188192787 (270.224µs)
Part 2: 113965544 (381.812µs)  
```

## Day 4: Day 4: Ceres Search

Quite enjoyed this, though I initially misunderstood the instructions and failed
to deal with a key requirement - straight lines only. The examples kinda *looked*
like crooked matches was OK, but once I learned (thanks, subreddit) the truth,
I wasn't far from a resolution. Part 2 was great; I was able to repurpose my part 1
pretty well, which is always satisfying.

### Timings

```
Part 1: 2534 (2.92382ms)
Part 2: 1866 (1.686545ms)
```

## Day 5: Print Queue

Another one I liked. Took me a little while to figure out what was needed but once
I reasoned it out, implementing it worked first time, near enough. Part 2 involved
traversing and modifying a vector while traversing so that makes borrowing rules
a challenge, but switching to using manual index access is simple enough.

### Timings

```
Part 1: 6267 (309.209µs)
Part 2: 5184 (1.212138ms)
```