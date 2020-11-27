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

//! Binary for the library.

use mac_sys_info::get_mac_sys_info;
use std::process::exit;
use mac_sys_info::structs::mac_sysinfo::MacSysInfo;

enum Action {
    ShowHelp,
    ShowSimple,
    ShowComplex,
    ShowRaw,
}

fn main() {
    let info = get_mac_sys_info();
    if info.is_err() {
        // IDE may does not find the Display trait impl, but its there
        // it gets implemented during compilation by "derive_more" crate
        eprintln!("There was an error. Can't get system information. Error: {}", info.unwrap_err());
        exit(-1);
    }
    let info = info.unwrap();

    let action = parse_args();
    match action {
        Action::ShowHelp => { print_help() }
        Action::ShowSimple => { print_simple(&info) }
        Action::ShowComplex => { print_complex(&info) }
        Action::ShowRaw => { print_raw(&info) }
    }

}

fn parse_args() -> Action {
    let args: Vec<String> = std::env::args()
        .map(|s| s.to_lowercase())
        .collect();
    if args.len() == 1 {
        return Action::ShowSimple;
    };

    if args[1] == "-h" || args[1] == "--help" || args[1] == "help" || args[1] == "h" {
        return Action::ShowHelp;
    }

    if args[1] == "-c" {
        return Action::ShowComplex;
    }

    if args[1] == "-r" {
        return Action::ShowRaw;
    }

    Action::ShowSimple
}

fn print_simple(info: &MacSysInfo) {
    println!("CPU:");
    println!("    name     : {}", info.cpu_info().brand_string());
    // IDE may does not find the Display trait impl, but its there
    // it gets implemented during compilation by "derive_more" crate
    println!("    arch     : {}", info.cpu_info().architecture());
    println!("    cores    : {}", info.cpu_info().num_cores());
    println!("    frequency: {} GHz", info.cpu_info().frequency_ghz());
    println!("    L1D-Cache: {}KB", info.cpu_info().cache_info().l1d_cache_size_kb());
    println!("    L1I-Cache: {}KB", info.cpu_info().cache_info().l1i_cache_size_kb());
    println!("    L2-Cache : {}KB", info.cpu_info().cache_info().l2_cache_size_kb());
    println!("    L3-Cache : {}KB", info.cpu_info().cache_info().l3_cache_size_kb());
    println!();
    println!("Memory:");
    println!("    total: {}MB", info.mem_info().total_memory_mb());
    println!();
    println!("OS:");
    println!("    version: {}", info.os_info().os_version());
    println!("    kernel : {}", info.os_info().kern_version());
}

fn print_help() {
    println!("Simple output     : '$ mac-sys-info'");
    println!("Complex output    : '$ mac-sys-info -c'");
    println!("Raw (all options)): '$ mac-sys-info -r'");
    println!("Help              : '$ mac-sys-info -h'");
}

fn print_complex(info: &MacSysInfo) {
    println!("{}", info.cpu_info());
    println!("{}", info.cpu_info().cache_info());
    println!("{}", info.mem_info());
    println!("{}", info.os_info());
    println!("{}", info.cpu_features());
}

fn print_raw(info: &MacSysInfo) {
    println!("This list is equivalent to `$ sysctl -a`, but it's ordered alphabetically.");
    println!("{:#?}", info.all_keys());
}
