# AoC Rust Template

This repository serves as a template for solving [Advent of Code](https://adventofcode.com) puzzles using Rust.

## Overview

The purpose of this repository is to provide a structured starting point for tackling Advent of Code puzzles. It is designed to support solving puzzles
for a single year. If you plan to participate in multiple years, you should create a separate repository or crate for each year.

### Structure

Each day's solution is organized as a module under `src/dayX`, with a library file (`src/dayX/mod.rs`) containing two functions:  

- `part1()`: Solves part 1 of the puzzle.  
- `part2()`: Solves part 2 of the puzzle.  

Both functions return the solution as a string.

The `main` program allows you to download puzzle inputs and submit solutions directly from the command line.

---

## Usage

1. Clone this repository or use it as a template on GitHub to create your own repository for a specific year.  
2. Implement the solutions for the desired day and part in the corresponding module.

**Note:** Since this template is built for a single year, if you are solving puzzles from multiple years,
you must create a separate repository or Rust crate for each year to avoid conflicts.

### Running Solutions

Once you've implemented a solution for year `Y`, day `D`, and part `P` (e.g., `dayD::partP`), you can run it with the following command:

```bash
cargo run -- -y Y -d D P
```

For example, if you want to run the code for day 18 of year 2022 and for part 2, you would use:

```bash
cargo run -- -y 2022 -d 18 2
```

- **Default Behavior**: If no year (`Y`), day (`D`), or part (`P`) is specified:
  - The year defaults to the current year.
  - The day defaults to the most recently unlocked day.
  - The part defaults to part 1.

For example:

- To solve the current day's part 1:  

  ```bash
  cargo run
  ```

- To solve the current day's part 2:  

  ```bash
  cargo run -- 2
  ```

### Submitting Solutions

To submit your solution, add the `-p` flag to the command (requires setup; see [Automatic Submission](#automatic-submission)):

```bash
cargo run -- -p -y Y -d D P
```

### Running All Solutions

To execute all implemented solutions, use the `--all` flag:

```bash
cargo run -- --all
```

### Benchmarking Solutions

Benchmark all solutions using:

```bash
cargo bench
```

(Note: This feature is still a work-in-progress.)

---

## Automatic Submission

To enable automatic solution submission, you need to configure your session cookie. Follow the instructions provided in the [aoc-cli README](https://github.com/scarvalhojr/aoc-cli#session-cookie) to retrieve your session cookie and set it up.

---

## Dependencies

This project relies on [aoc-cli](https://github.com/scarvalhojr/aoc-cli) for interacting with the Advent of Code servers (e.g., downloading inputs, submitting answers).
