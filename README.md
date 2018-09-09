*Note: The original codebase was taken from the [`cargo open`](TODO) project, and was adapted to work for listing source files instead.*

# `cargo src`

TODO build status

[![Build Status](https://travis-ci.org/AndrewRadev/cargo-src.svg?branch=master)](https://travis-ci.org/AndrewRadev/cargo-src)

A third-party cargo extension that lists local source locations of dependencies.

## Installing

You can install the tool from crates.io by running:

``` bash
$ cargo install cargo-src
```

This will install the executable `cargo-src` in your cargo `bin` directory, which on *nix systems would be `~/.cargo/bin`. You should add that directory to your `PATH`.

## Usage

After installing, you should be able to go to any cargo project's root directory and run the following command:

``` bash
$ cargo src
```

This will output a list of all of the dependencies' source locations, or at least the ones that exist in the filesystem. If you provide a list of package names, the tool will show the location of only those. Example output:

``` bash
$ cargo src clap cargo
/home/andrew/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.32.0
/home/andrew/.cargo/registry/src/github.com-1ecc6299db9ec823/cargo-0.28.0
```

On its own, this isn't super interesting, but it can be used as a component in other tools like shell scripts. A few examples follow:

### cargo-open

The original codebase for this tool, can be more or less reimplemented with:

``` bash
$ $EDITOR `cargo src <package-name>`
```

A longer script that handles errors might look like this:

``` bash
#! /bin/sh

if [ $# -lt 2 ]; then
  echo "USAGE: cargo open <package-name>"
  exit 1
fi

_subcommand=$1 # ignore the "open" subcommand
package=$2
src=$(cargo src $package)
status=$?
if [ $status -ne 0 ]; then
  exit $status
fi

$EDITOR "$src"
```

### cargo-tags

The original intent for this fork. A tool like this already exists as [rusty-tags](TODO), but it's easy to do something similar with a one-liner: `ctags -o Cargo.tags -R $(cargo src)`. A longer script with error handling:

``` bash
#! /bin/sh

sources=$(cargo src)
status=$?
if [ $status -ne 0 ]; then
  exit $status
fi

# TODO check for problems with spaces and stuff
ctags -o Cargo.tags -R $sources
```

## Contributing

You can run the tool locally by executing `cargo run src`. Note the subcommand -- it's necessary, because it would ordinarily be called as `cargo src`.

If you'd like to run it on a different directory, you can either install the local program with `cargo install --force`, or you can find the compiled binary in `target/debug/cargo-src`, and run it by using the full path to the executable.

If you've made a change that's useful to you, consider preparing a pull request on [github](TODO). If you've found a bug or are not sure how to implement a particular feature, feel free to open an issue and ask for help.
