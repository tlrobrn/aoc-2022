# Advent of Code 2021

[Advent of Code](https://adventofcode.com) solutions in [Rust](https://rust-lang.org).

## Structure

Common utilities are defined in [src/lib.rs](src/lib.rs).

Day implementations are defined in [src/bin/](src/bin/).

## Usage

Pass the day binary to `cargo run` and pass the input through `stdin`.
For example, to run `day01` with the input saved in `resources/day01.txt`:

```sh
$ cargo run --bin day01 < resources/day01.txt
```

## Test

```sh
$ cargo test
```

To test a specific day (e.g. `day01`):

```sh
$ cargo test --bin day01
```
