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

## Templates

`main.cpp` is not created empty. When `listen` receives a problem for a
folder that has no `main.cpp` yet, it copies a template there. An existing
`main.cpp` is never overwritten, so re-clicking the extension is safe.

The template is copied verbatim — no placeholders, no substitutions. It is
resolved once, when `comp-cli listen` starts:

1. `~/.config/comp-cli/template.cpp` (respects `$XDG_CONFIG_HOME`)
2. otherwise, a default template embedded in the binary
   (`assets/default_template.cpp`)

To customize it, create the config file:

```sh
mkdir -p ~/.config/comp-cli
cp assets/default_template.cpp ~/.config/comp-cli/template.cpp
```

Notes:

- the template is read once at startup — restart `comp-cli listen` after editing it
- an empty template file produces an empty `main.cpp`; the file is used as-is
- if the template exists but cannot be read (permissions, invalid UTF-8),
  `listen` exits with an error naming the path instead of silently falling back

Compile `main.cpp` and run all tests:

```sh
# run in current directory (default 1s timeout per test)
comp-cli run

# specify folder and custom time limit (in seconds)
comp-cli run ./my_problem -l 2
```

## Known limitations

- `run` waits for the solution to exit before draining its stdout/stderr.
  A solution that writes more than the kernel pipe buffer (~64 KiB) before
  exiting blocks on write and is reported as TLE, even if it would have
  finished immediately. This does not affect infinite loops (TLE either way),
  and sample tests are small enough that it is unlikely in normal use.
  Revisit if manual tests or stress testing are added.

