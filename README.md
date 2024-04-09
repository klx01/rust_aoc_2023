Just me trying to get familiar with rust by solving [advent of code](https://adventofcode.com/2023) challenges.

# Usage
    cargo run --bin day01
and so on

# Workaround over an issue
For some reason, binaries that are built in release mode are getting instantly killed when i run them. It does not happen in debug mode. So i've checked which options are used in release via

    cargo build -vv -r

and narrowed it down to -C strip=debuginfo . And i've double-checked using the bare rustc, a "hello world" compiled with this option produces a binary that gets killed, and without this option it works fine.

So i've added this to the Cargo.toml 

    [profile.release]
    strip = "none"

And it helped.

As for why this happens, i've only managed to dig this using the log util

    kernel: (AppleSystemPolicy) ASP: Security policy would not allow process: <pid>, <path_to_the_binary_that_ive_tried_to_run>

# Todo
 * day 21 part 2 - i know what to do, but it's cumbersome
 * day 24 part 2 - requires some arcane math, need to check how it works
 * day 25 - need to study those graph algos
