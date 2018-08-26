*Note: The original codebase was taken from the [`cargo open`](TODO) project, and was adapted to
work for listing source files instead.*

# `cargo src`

TODO build status

[![Build Status](https://travis-ci.org/AndrewRadev/cargo-src.svg?branch=master)](https://travis-ci.org/AndrewRadev/cargo-src)

A third-party cargo extension that lists local source locations of dependencies.

## Installing

You can install the tool from crates.io by running:

``` bash
$ cargo install cargo-src
```

This will install the executable `cargo-src` in your cargo `bin` directory, which on *nix systems
would be `~/.cargo/bin`. You should add that directory to your `PATH`.

## Usage

After installing, you should be able to go to any cargo project's root directory and run the
following command:

``` bash
$ cargo src
```

TODO

## Contributing

You can run the tool locally by executing `cargo run src`. Note the subcommand -- it's necessary,
because it would ordinarily be called as `cargo src`.

If you'd like to run it on a different directory, you can either install the local program with
`cargo install --force`, or you can find the compiled binary in `target/debug/cargo-src`, and run
it by using the full path to the executable.

If you've made a change that's useful to you, consider preparing a pull request on [github](TODO).
If you've found a bug or are not sure how to implement a particular feature, feel free to open an
issue and ask for help.
