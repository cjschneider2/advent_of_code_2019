# advent_of_code_2019
 Rust workspace template for the advent of code 2019

## Workspace template:
This is to show a basic template of how you could set up a workspace project.

The `main.rs` file in `day_1` is already filled out with tests for part-1, 
just add your solution :)

Run `cargo test` or `cargo test --release` to run the test cases, and 
`cargo run --release` to run the main function. I've turned on debug symbols
in the `Cargo.toml` in the workspace root, so all the release binaries will
include debugging symbols by default. This makes debugging with something
like GCC much easier (GCC also understands rust symbols for a few years now).
