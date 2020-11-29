# mac-sys-info
CLI + Library to get detailed information about your Mac system. Including CPU, Cache (L1, L2, L3), Memory, and more.
The library is a wrapper around all information that the `$ sysctl -a` command outputs
on Mac.

## Usage
### Library
**Cargo.toml:**
```toml
# please check if this is the latest version
# maybe I forget to update the README sometime
mac-sys-info = "0.1.6"
```
**Code:**
```rust
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
### run/install CLI
```
$ cargo install mac-sys-info
$ mac-sys-info      # simple
$ mac-sys-info -h   # help
$ mac-sys-info -c   # complex
$ mac-sys-info -r   # raw
```
#### Example output
```
CPU:
    name     : Intel(R) Core(TM) i5-5257U CPU @ 2.70GHz
    arch     : X86_64
    cores    : 4
    frequency: 2.7 GHz
    L1D-Cache: 32KB
    L1I-Cache: 32KB
    L2-Cache : 256KB
    L3-Cache : 3072KB

Memory:
    total: 8192MB

OS:
    version: 10.15.7
    kernel : Darwin Kernel Version 19.6.0Thu Oct 29 225645 PDT 2020; rootxnu-6153.141.2.2~1/RELEASE_X86_64
```

## Why another tool?
There is already a popular crate called [sys-info](https://crates.io/crates/sys-info), that's true.
But it doesn't show information about the size of L1 cache, or cache in general.
I need this information for a task I'm working on. Therefore I created this crate.
It was also fun and as always I learned new stuff. :)

## LICENSE
MIT License. See "LICENSE" file in repository.
