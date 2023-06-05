# NES Flipull (v1.0) solver for TAS

## Extract a normal mode problem from the game ROM

Give `(rom, stage, rng_state)` to `extract_normal` executable.

Stages are 0-based. The same applies thereafter.

```sh
$ cargo run --example=extract_normal -- Flipull.nes 0 0x1234
```

## Extract a advance mode problem from the game ROM

Give `(rom, stage)` to `extract_advance` executable.

```sh
$ cargo run --example=extract_advance -- Flipull.nes 0
```

## Solve a problem

Give `path_problem` to `solve` executable.

```sh
$ cargo run --example=solve -- problem.txt
$ cargo run --example=solve -- --forbid-five problem.in  # forbid to erase 5 or above blocks at once
$ cargo run --example=solve -- --forbid-just problem.in  # forbid "just clear"
$ cargo run --example=solve -- --last-stage problem.in  # for the last stage
```

## Convert a solution to a NESHawk movie (you can paste it to TAStudio)

```sh
$ cargo run --example=format_solution -- --format=neshawk problem.txt solution.txt
```

## Search top-K RNG seeds for a normal mode stage

Give `(path_ines, stage, k)` to `solve_normal_multi` executable.

You can specify various options. For details, use `--help` option.

```sh
$ cargo run --example=solve_normal_multi -- --max-cost 1000 Flipull.nes 0 10
```
