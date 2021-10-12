# GREP clone
A clone of grep, simpler, faster and without unit testing (for the moment).

## Why is this "different"?
Well, as mentioned in multiple places in this repo, this app was created following [the guide](https://rust-cli.github.io/book/index.html).

However, this version includes <b><u>piping support</u></b> (reading from stdin), meaning that the input file flag is optional. 
It also implements a <b><u>Buffered Reader</u></b> instead of loading the entire file (or stdin) into memory at once.

## Future features
 - [x] ~~Colored output, with an optional flag.~~
 - [x] ~~Multiple patterns~~
 - [ ] Multi-threading
 - [ ] Add a debug or verbose flag(?) but I don't know what it should output

## Multi-platforming or something
I used the amazing and wonderful ["cross" cargo program](https://crates.io/crates/cross) to cross-compile the program to windows. Can't compile for Mac because I don't own one :) blame Apple.

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

## Multithreading?
It would be really cool, and a good learning experience, BUT!

Multithreading comes with certain overhead, meaning that it is only worth using for really large tasks. 
So the program would need to check the size of the input and then use multithreading (single-threading otherwise).
