# Simple Tee
## Purpose
This is a similar to command **tee** on Unix.
It takes the std in content and concatenates it to the content of speciifed files, and then out the result to the std out. 
A file specified by `-o` option can be used instead of the std out or capturing a redirect out.
`-r` option can reverse the behavior and use specified files to get content of the std in similary to
the standard **tee** command. These two options can't be used together.

Option `-a` instructs to append the generated result insted of writing a new file. The utility doesn't 
overwrites existing files unless option `-w` specified. These two options can't be specified together.

## Build
Use the provided [bee.7b](https://github.com/vernisaz/simtee/blob/master/bee.7b) script
with [rb](https://github.com/vernisaz/rust_bee), or **cargo** with `.toml` manifest.

## Dependencies
[Simple CLI](https://github.com/vernisaz/simcli)
