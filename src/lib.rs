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

//! Library to get Mac-specific system info. The information is
//! retrieved from the `$ sysctl -a` command.

#[macro_use]
extern crate log;

use crate::error::MacSysInfoError;
use unix_exec_output_catcher::fork_exec_and_catch;
use crate::parse::{parse_sysctl_line};
use crate::structs::mac_sysinfo::MacSysInfo;
use std::collections::{BTreeMap};

pub mod error;
pub mod generated_sysctl_keys;
mod parse;
pub mod structs;

fn fetch_info_from_sysctl() -> Result<Vec<(String, String)>, MacSysInfoError> {
    let res = fork_exec_and_catch("sysctl", vec!["sysctl", "-a"])
        .map_err(|_| MacSysInfoError::CantFetchData)?;
    let res = res.stdout_lines();
    let res = res.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let key_value_vector = res.iter()
        .map(|s| parse_sysctl_line(s))
        .collect::<Vec<(String, String)>>();

    Ok(key_value_vector)
}

/// Returns system information about your Mac. The information is retrieved
/// from the `sysctl -a` command.
pub fn get_mac_sys_info() -> Result<MacSysInfo, MacSysInfoError> {
    let key_value_vector = fetch_info_from_sysctl()?;

    // build the raw string to string map of all supported information
    let mut all_keys = BTreeMap::new();
    key_value_vector.into_iter()
        .for_each(|(sys_key, sys_value)| {
            all_keys.insert(sys_key, sys_value);
        });

    Ok(
        MacSysInfo::new(all_keys)?
    )
}

#[cfg(test)]
mod tests {
    use crate::get_mac_sys_info;

    #[test]
    fn test_get_sys_info() {
        get_mac_sys_info();
    }
}
