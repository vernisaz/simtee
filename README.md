# Simple Tee
## Purpose
This is a similar command to Unix **tee** utility.
It takes std in content and concatenates it to the content of speciifed files, and then out the result to std out. 
A file specified by `-o` option can be used instead of std out or capturing a redirect out.
`-r` option can reverse the behavior and use specified files to get content of std in similary to
the standard **tee** command. These two options can't be specified together.

Option `-a` instructs to append the generated result insted of writing a new file. The utility doesn't 
overwrites existing files unless option `-w` specified. These two options can't be used together.

## Build
Use the provided `bee.7b` script with [rb](https://github.com/vernisaz/rust_bee), or **cargo** with `.toml` description.

## Dependencies
(Simple CLI)[https://github.com/vernisaz/simcli]
