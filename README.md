*Note: The original codebase was taken from the [`cargo open`](TODO) project, and was adapted to
work for generating tags instead.*

# `cargo tags`

TODO build status

[![Build Status](https://travis-ci.org/AndrewRadev/cargo-tags.svg?branch=master)](https://travis-ci.org/AndrewRadev/cargo-tags)

A third-party cargo extension that generates a ctags tags file for all of your cargo dependencies.

## Installing

You can install the tool from cargo.io by running:

``` bash
$ cargo install cargo-tags
```

This will install the executable `cargo-tags` in your cargo `bin` directory, which on *nix systems
would be `~/.cargo/bin`. You should add that directory to your `PATH`.

In order to generate the tags, you'll need a compatible `ctags` executable. The two main options are:

- [Universal ctags](TODO). Should work out of the box for Rust.
- [Exuberant ctags](TODO). You'll need to add Rust support with regexes. There's an example ctags
  config [here](TODO) that will probably do a reasonable job.

## Usage

After installing, you should be able to go to any cargo project's root directory and run the
following command:

``` bash
$ cargo tags
```

Provided you have a `ctags` program, this will eventually generate a file named `Cargo.tags`, which
will contain all symbols and their source locations on your system. Make sure you've run a `cargo
build`, so that the source code for all of the dependencies is fetched (if you haven't, there won't
be any source to index for that package quite yet).

## Contributing

You can run the tool locally by executing `cargo run tags`. Note the subcommand -- it's necessary,
because it would ordinarily be called as `cargo tags`.

If you'd like to run it on a different directory, you can either install the local program with
`cargo install --force`, or you can find the compiled binary in `target/debug/cargo-tags`, and run
it by using the full path to the executable.

If you've made a change that's useful to you, consider preparing a pull request on [github](TODO).
If you've found a bug or are not sure how to implement a particular feature, feel free to open an
issue and ask for help.
