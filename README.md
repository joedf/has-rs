# has-rs

A simple rust implementation of [`has`](https://github.com/kdabir/has) by [kdabir](https://github.com/kdabir).
> `has` checks presence of various command line tools on the PATH and reports their installed version.

This was created as part of [my practice and learning](https://github.com/joedf/LearningRust) of the Rust programming language. I started this back in August 2022, but I got to releasing this now (October 2024).

## build & run

You can simply navigate to the `has-rs` folder and run the following command to build it:
> cargo build

... or to run it and check if a system has `git`, `gcc` and `node` installed:
> cargo run git gcc node

