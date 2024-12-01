# Advent of Code 2024 !!! 

Well, here we are again. Another year, another promise to myself that I'll respect my own
time and bail out once I get tired of it. We'll see how that goes. I'm using Rust again, 
because it's fun, and it's where I'm most productive. Deps are fine. 

Solutions all expect inputs to be found in the `./input` directory, at the project root. Its
contents are stored in a private submodule, to respect the requirement not to publish inputs.

Timings come from a run in GitHub Actions.

## Day Setup

- ensure the `aocday` template and `aocfin` input downloaded are setup 
  (see [aoc-shared](https://github.com/dwalker109/aoc-shared).
- `cd days && cargo generate aocday --name dayXX`
- `cd input && aocfin -O dayXX 2024 XX`

## Day 1: Historian Hysteria

Not hard but I haven't done any AOC since late 2023 so I was a bit rusty. Took
longer that I would expect it to. Went with a simple pair of lists in the end.
Part 2 could be optimised by pre-calculating frequency of occurrences, but since
a naive approach (just count the list every time) take less than a millisecond, 
I just stayed basic. Maybe this will bother me and I'll come back later, who knows.

### Timings

Part 1: 2176849 (81.833µs)
Part 2: 23384288 (371.131µs)
