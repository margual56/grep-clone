# GREP clone
A clone of grep, simpler, faster and without unit testing (for the moment).

## Why is this "different"?
Well, as mentioned in multiple places in this repo, this app was created following [the guide](https://rust-cli.github.io/book/index.html).

However, this version includes <b><u>piping support</u></b> (reading from stdin), meaning that the input file flag is optional. 
It also implements a <b><u>Buffered Reader</u></b> instead of loading the entire file (or stdin) into memory at once.

## Future features
 * Planning on coloring the output, or at least give the option.
 * Add a debug or verbose flag(?) but I don't know what it should output

## Multi-platforming or something
I used the amazing and wonderful ["cross" cargo program](https://crates.io/crates/cross) to cross-compile the program to windows. Can't compile for Mac because I don't own one :) blame Apple.

