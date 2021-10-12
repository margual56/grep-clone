# GREP clone
A clone of grep, simpler, slower and without unit testing (for the moment).

Check out [the wiki](https://github.com/margual56/grep-clone/wiki) if you want to dive deeper into... stuff :)

## Why is this "different"?
As mentioned in multiple places in this repo, this app was created following [the guide](https://rust-cli.github.io/book/index.html).

However, this version includes <b><u>piping support</u></b> (reading from stdin), meaning that the input file flag is optional. 
It also implements a <b><u>Buffered Reader</u></b> instead of loading the entire file (or stdin) into memory at once.

_Note: Using a Buffered Reader has a drawback, which is that you are creating a copy of the input, and processing it later._

_Note II: it also uses a Buffered output, which can cause performance issues as discussed in [the performance analysis](https://github.com/margual56/grep-clone/wiki/Why-slower%3F#why-is-grepclone-slower-than-gnus-grep)._


## Future features
 - [x] ~~Colored output, with an optional flag.~~
 - [x] ~~Multiple patterns~~
 - [ ] Doing the much needed optimizations
 - [ ] Multi-threading
 - [ ] Add a debug or verbose flag(?) but I don't know what it should output

## Usage 
Straight from `grepclone -h`:
```
grepclone 0.2.0
Just a simple grep clone programmed in Rust.
grepclone takes either an input file (`-i` option) or reads the stdin if no file is passed as an argument.

USAGE:
    grepclone [FLAGS] [OPTIONS] [patterns]...

FLAGS:
    -c, --color      Wether to print with colored output or not
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --input <input>    Input file (Optional)

ARGS:
    <patterns>...    Pattern(s) to search for. Required parameter
```

## Cross-compiling
I used the amazing and wonderful ["cross" cargo program](https://crates.io/crates/cross) to cross-compile the program to windows. Can't compile for Mac because I don't own one :) blame Apple.

## Why is it slower?
Check the [detailed explanation and analysis](https://github.com/margual56/grep-clone/wiki/Why-slower%3F#why-is-grepclone-slower-than-gnus-grep) in the wiki.

_TL;DR_: They do byte manipulations and program at a lower level, as well as being smarter xD.

## Multithreading?
It would be really cool, and a good learning experience, BUT!

Multithreading comes with certain overhead, meaning that it is only worth using for really large tasks. 
So the program would need to check the size of the input(?) and then use multithreading (single-threading otherwise).
