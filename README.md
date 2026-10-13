# comp-cli

CLI tool to parse problems from [Competitive Companion](https://github.com/jmerle/competitive-companion) and run local C++ tests.

## Requirements

- `cargo`
- `g++` (compiles with `-O2 -std=c++23`)
- Competitive Companion browser extension (uses port `27121`)

## Build

```sh
cargo install --path .
```

## Usage

Listen for problems from the browser extension:

```sh
# keep listening in the current directory
comp-cli listen

# receive a single problem into a specific folder and exit
comp-cli listen ./my_problem --once
```

This creates:

```text
problem_name/
├── main.cpp
└── tests/
    ├── 1.in
    ├── 1.out
    └── ...
```

Compile `main.cpp` and run all tests:

```sh
# run in current directory (default 1s timeout per test)
comp-cli run

# specify folder and custom time limit (in seconds)
comp-cli run ./my_problem -l 2
```
