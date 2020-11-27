# mac-sys-info
CLI + Library to get detailed information about your Mac system. Including CPU, Cache (L1, L2, L3), Memory, and more.
The library is a wrapper around all information that the `$ sysctl -a` command outputs
on Mac.

## Usage
### Library
**Cargo.toml:**
```
mac-sys-info = "0.1.0"
```
**Code:**
```
use mac_sys_info::get_mac_sys_info;
use mac_sys_info::generated_sysctl_keys::SysctlKey;

fn main() {
    // get struct with all information
    let info = get_mac_sys_info().unwrap();
    
    // even if IDE shows that the Display trait is missing:
    // it is there. It gets implemented during compile time
    // by "derive_macro" crate.
    println!("{}", info.cpu_info());
    println!("{}", info.cpu_info().cache_info());
    // ...
    println!("CPU @ {} Ghz", info.cpu_info().frequency_ghz());
    
    // IMPORTANT: Raw Access to all SysctlKeys is
    // available! Keys are located in enum `SysctlKey`
    println!("L3 Cache size extracted manually: {}",
             info.all_keys().get(SysctlKey::HwL3cachesize.name()).unwrap()
    );
    
    // Some convenient getters.
    println!("L2 Cache Size in KB: {}",
             info.cpu_info().cache_info().l2_cache_size_kb()
    );
    println!("System has {} GB of RAM", info.mem_info().total_memory_mb())
}
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
But it doesn't show information about the size of L1 cache, or cache in general.
I need this information for a task I'm working on. Therefore I created this crate.
It was also fun and as always I learned new stuff. :)

## LICENSE
MIT License. See "LICENSE" file in repository.
