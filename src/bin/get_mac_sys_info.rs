/*
MIT License

Copyright (c) 2020 Philipp Schuster

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use mac_sys_info::get_mac_sys_info;
use mac_sys_info::generated_sysctl_keys::SysctlKey;

fn main() {
    let info = get_mac_sys_info().unwrap();
    // even if IDE shows that the Display trait is missing:
    // it is there. It gets implemented during compile time
    // by "derive_macro" crate.
    println!("{}", info.cpu_info().architecture());
    println!("{}", info.cpu_info().cache_info());
    println!("{}", info.cpu_features());
    println!("{}", info.os_info());
    println!("{}", info.mem_info());
    println!("CPU @ {} Ghz", info.cpu_info().frequency_ghz());
    println!("L3 Cache size extracted manually: {}",
             info.all_keys().get(SysctlKey::HwL3cachesize.name()).unwrap()
    );
    println!("L2 Cache Size in KB: {}",
             info.cpu_info().cache_info().l2_cache_size_kb()
    );
    println!("System has {} GB of RAM", info.mem_info().total_memory_mb())
}