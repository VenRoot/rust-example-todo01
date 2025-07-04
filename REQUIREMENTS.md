# Packages

## CLI
I use clap for arguments. It is very common and can use arguments like `-all`, `--completed` etc.

## Filesystem

I use `serde` and `serde-yaml` to save files in YAML. 
I considered using `serde-json` first as it's my first choice as a web-dev but yaml seems to be more efficient here

Even tho `serde-yaml` is deprecated, it just means that it's not currently maintained and yaml doesn't really change


# Error Handling

For error handling, I'll use the simple package `anyhow`.
It's not as powerful as `thiserror`, but it will do the trick for user feedback