# saguaro

Saguaro is a CDCL SAT solver.

## Build & Run
To run Saguaro:
```
cargo run -- <cnf file>
```

Or, to build a release:
```
cargo build --release
```

## Testing & Benchmarking

This repository includes two small Ruby scripts, `checker.rb` and `benchmark.rb` (which depends on `checker.rb`), to
perform bare-minimum benchmarking and validation of Saguaro's outputs. These scripts assume that certain solvers are
installed (e.g. Z3) but it's easy to update them to use different solvers if necessary.