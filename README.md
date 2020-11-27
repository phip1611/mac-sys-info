# mac-sys-info
CLI + Library to get detailed information about your Mac system. Including CPU, Cache (L1, L2, L3), Memory, and more.
The library is a wrapper around all information that the `sysctl -a` command outputs
on Mac.

## Usage
### Library
**Cargo.toml:**
```
mac-sys-info = "0.1.0"
```
### CLI
```
$ cargo install mac-sys-info
$ mac-sys-info      # simple
$ mac-sys-info -h   # help
$ mac-sys-info -c   # complex
$ mac-sys-info -r   # raw
```

## Why another tool?
There is already a popular crate called [sys-info](https://crates.io/crates/sys-info), that's true.
But it doesn't display information about the size of L1 cache, or cache in general.
I need this for a task I'm working on. Therefore I created this crate.

## LICENSE
MIT License. See "LICENSE" file in repository.
