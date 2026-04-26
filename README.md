# Simple Tee
## Purpose
This is similar to the command **tee** on Unix.
It takes the stdin content and concatenates it to the content of speciifed files, and then out the result to the stdout. 
A file specified by `-o` option can be used instead of the stdout. The same behavior can be achieved by redirecting the stdout to a file.

`-r` option can reverse the behavior and use specified files to get content of the stdin similary to
the standard **tee** command. It can't be used with `-o` option..

The option `-a` instructs to append the generated result insted of writing a new file. The utility doesn't 
overwrites existing files unless option `-w` specified. These two options are mutually exclusive.

## Build
Use the provided [bee.7b](https://github.com/vernisaz/simtee/blob/master/bee.7b) script
with [rb](https://github.com/vernisaz/rust_bee) and [common scripts](https://github.com/vernisaz/simscript),
or **cargo** with a `.toml` manifest.

## Dependencies
- [Simple CLI](https://github.com/vernisaz/simcli)
