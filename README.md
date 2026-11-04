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

# receive a single problem, open main.cpp in Zed, and exit
comp-cli listen --once --zed

# receive into a specific folder
comp-cli listen ./my_problem -o
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

## Templates

If `main.cpp` does not exist when a problem is received, `listen` creates it from a template (existing files are never overwritten):

1. `~/.config/comp-cli/template.cpp` (or `$XDG_CONFIG_HOME/comp-cli/template.cpp`)
2. Fallback: embedded `assets/default_template.cpp`

```sh
mkdir -p ~/.config/comp-cli
cp assets/default_template.cpp ~/.config/comp-cli/template.cpp
```
