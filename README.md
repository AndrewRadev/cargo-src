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
- [Exuberant ctags](TODO). You'll need to add Rust support with regexes. There's ctags configs
  [here](https://github.com/BurntSushi/mempool/blob/master/ctags.rust) and
  [here](https://github.com/nikomatsakis/rust-ctags/blob/master/ctags.rust) that will probably do a
  reasonable job. Save them as `~/.ctags`, and you can even tweak them, if you'd like.

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

TODO add to Vim
TODO options

## Contributing

You can run the tool locally by executing `cargo run tags`. Note the subcommand -- it's necessary,
because it would ordinarily be called as `cargo tags`.

If you'd like to run it on a different directory, you can either install the local program with
`cargo install --force`, or you can find the compiled binary in `target/debug/cargo-tags`, and run
it by using the full path to the executable.

If you've made a change that's useful to you, consider preparing a pull request on [github](TODO).
If you've found a bug or are not sure how to implement a particular feature, feel free to open an
issue and ask for help.

## Similar projects

- [`cargo-open`](TODO): The original codebase was taken from this project and adapted to work with
  tags instead.
- [`rusty-tags`](https://github.com/dan-t/rusty-tags): Pretty much does the same thing as this
  project, but hadn't realize it existed before I started it :). There's some differences in
  functionality. For instance, cargo-tags doesn't generate tags for the project itself, unless
  explicitly requested by a setting. That way, local tags and dependency tags can be maintained
  separately.
