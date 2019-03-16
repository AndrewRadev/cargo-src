*Note: The original codebase was taken from the [`cargo open`](https://github.com/carols10cents/cargo-open) project, and was adapted to work for listing source files instead.*

# `cargo local`

A third-party cargo extension that lists local source locations of dependencies.

## Installing

You can install the tool from crates.io by running:

``` bash
$ cargo install cargo-local
```

This will install the executable `cargo-local` in your cargo `bin` directory, which on *nix systems would be `~/.cargo/bin`. You should add that directory to your `PATH`.

## Usage

After installing, you should be able to go to any cargo project's root directory and run the following command:

``` bash
$ cargo local
```

This will output a list of all of the dependencies' source locations, or at least the ones that exist in the filesystem. If you provide a list of package names, the tool will show the location of only those. Example output:

``` bash
$ cargo local clap cargo
/home/andrew/.cargo/registry/src/github.com-1ecc6299db9ec823/clap-2.32.0
/home/andrew/.cargo/registry/src/github.com-1ecc6299db9ec823/cargo-0.28.0
```

You can run the command with `--only-names` to list package names alone -- useful for generating shell completion, for instance:

``` bash
$ cargo local --only-names
toml
semver-parser
fuchsia-zircon
atty
...
```

On its own, this isn't super interesting, but it can be used as a component in other tools like shell scripts. A few examples follow:

### cargo-open

The project that this codebase was more-or-less copied from can be more-or-less implemented like so:

``` bash
$ $EDITOR `cargo local <package-name>`
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
path=$(cargo local $package)
status=$?
if [ $status -ne 0 ]; then
  exit $status
fi

$EDITOR "$path"
```

### cargo-tags

The original plan for this fork. A tool like this already exists as [rusty-tags](https://github.com/dan-t/rusty-tags), but it's easy to do something similar with a one-liner: `ctags -o Cargo.tags -R $(cargo local)`. A longer script with error handling:

``` bash
#! /bin/sh

sources=$(cargo local)
status=$?
if [ $status -ne 0 ]; then
  exit $status
fi

echo "$sources" | xargs --delimiter="\n" ctags -o Cargo.tags -R
```

## Contributing

You can run the tool locally by executing `cargo run local`. Note the subcommand -- it's necessary, because it would ordinarily be called as `cargo local`.

If you'd like to run it on a different directory, you can either install the local program with `cargo install --path . --force`, or you can find the compiled binary in `target/debug/cargo-local`, and run it by using the full path to the executable.

If you've made a change that's useful to you, consider preparing a pull request on [github](https://github.com/AndrewRadev/cargo-local). If you've found a bug or are not sure how to implement a particular feature, feel free to open an issue and ask for help.
