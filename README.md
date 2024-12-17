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

## Day 6: Guard Gallivant

I *really* made a meal of this. I feel like there is more code here than strictly
sensible, but I wrote the structs and their methods up front, and part 1 basically
worked immediately. Part 2 just got me in a muddle though. My code was a mess,
I kept getting the wrong answers, and my debugging ended up being scattershot.
I came back to it later and rewrote part 2, and got it. It's brute force, and slower
than I'd like (nearly half a second), but I'm leaving it as is (for now at least).

### Timings

```
Part 1: 5199 (404.163µs)
Part 2: 1915 (542.346611ms)
```

## Day 7: Bridge Repair

Thoroughly enjoyed this. Once I figured out how to make my various permutations
(actually *cartesian product*), I needed to act on them. I initially just used an
eval library to make it easy but operator precedence killed that idea, so I switched
to folding over my own enum containing either operators or a literal number. This
led to a *glorious* part 2 where I made a tiny change (literally add another op)
and solved it immediately. Serendipitous indeed. Performance was not great though,
so once I was advised that doing the sums right to left would lead to much more
efficient culling of sums which wouldn't work, I implemented that and speeded
things up 6x, to something I'm happier with. Caching the ops also helped a little.

### Timings

```
Part 1: 538191549061 (2.884808ms)
Part 2: 34612812972206 (113.005146ms)
```

## Day 8: Resonant Collinearity

Another *excellent* day. I found it quite straightforward, and part 2 was another
natural extension of part 1, which was satisfying. Implementing Add and Sub traits
for my Xy coordinates, and iterating over each group of antennae in two dimensions
felt elegant, and is very fast indeed. Allowing myself to wake up and ponder the
problem a little bit works wonders for me. Probably the quickest day so far,
bizarrely.

### Timings

```
Part 1: 332 (33.903µs)
Part 2: 1174 (94.125µs)
```

## Day 9: Disk Fragmenter

I don't love my solution here - it feels a bit messy, hard to read, and isn't 
that quick (c. 80ms for part 2 after some ugly optimisation) - but I'll go with 
it. Mutably traversing lists from both directions can be a bit gory in rust, but
what I have here isn't horrific so there's that, at least. Not my best day, not
my worst.

### Timings

```
Part 1: 6415184586041 (2.177603ms)
Part 2: 6436819084274 (78.568809ms)
```

## Day 10: Hoof It

I figured out most of this before starting to code it; but I assumed I'd need
to use a cache and so worked myself into a knot. They I just kept getting the
wrong answers to the test data and had convinced myself I'd got some weird bug.
It turns out I'd just solved part 2 first because the part 1 instructions were
a bit obtuse; on re-reading them later I realised the mistake I'd made, and
according to the the subreddit, this is common. Got it working in 5 mins after 
that, and part 2 was a cakewalk. In fact, the same solve function produces
both answers since all that really changes is a lack of deduplication in part 2.

### Timings

```
Part 1: 538 (382.454µs)
Part 2: 1110 (315.249µs)
```

## Day 11: Plutonian Pebbles

This was easy, because I remember the lanternfish problem. It's basically the same.
The problem statement was (intentionally) misleading because it stated the pebbles
were ordered, but in reality, that's irellevant because the rules are independent 
and we only care about quantities. Fun, since I knew the trick. Also pretty fast.

### Timings

```
Part 1: 538 (386.786µs)
Part 2: 1110 (351.85µs)
```

## Day 12: Garden Groups

This one definitely required some thought. Grouping things into regions is easy,
and figuring out a perimeter was too (how many of the adjacent X, Y co-ordinates
are contained within this region, subtracted from 4). Calculating the number of
sides took a bit more gymnastics and ultimtely the amount of interation I'm 
doing here means this isn't mega fast, but at < 10 ms for both parts it's OK.

### Timings

```
Part 1: 1415378 (2.790766ms)
Part 2: 862714 (5.741602ms)
```

## 👺 Day 13: Claw Contraption

A maths days. And not one where I figured out what the maths was and felt like it
tackled it; I just didn't get it. I knew how to brute force part 1 but I didn't
even bother trying, and a look at the subreddit clued me in that "linear algebra"
was what I needed, but after watching 2 hours of videos I got some of the basics
but didn't have a clue what to do to solve this. So, I read the tutorial at
https://www.reddit.com/r/adventofcode/comments/1hd7irq/2024_day_13_an_explanation_of_the_mathematics/
and implemented my own variant. I can't take any credit for this one, so I didn't
even do it until the following day. Worst one of the year and I hope we don't
get many more of these.

### Timings

```
Part 1: 38714 (138.498µs)
Part 2: 74015623345775 (164.236µs)
```

## Day 14: Restroom Redoubt

This wasn't especially though. Part 1 was a simple add with mod to facilitate wrapping
aroung a grid. It *looked* like this was going to be a problem which punished
you for running a simple simulation (instead of skipping ahead), but in fact part 2
required solving something a bit ambiguous - finding a picture in the output.
There was no test case for this and it was mainly detective work. In the end, I
stumbled across the hueristic (all robots will be on a unique square when the tree
is being displayed) and optimised that to finish things off.

### Timings

```
Part 1: 219512160 (76.684µs)
Part 2: 6398 (15.430592ms)
```
