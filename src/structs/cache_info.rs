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

use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::error::MacSysInfoError;
use crate::parse::{parse_sysctl_value, ParseAsType};
use crate::generated_sysctl_keys::SysctlKey;

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "CacheInfo (\n\
\x20    l1i_cache_size: {},\n\
\x20    l1d_cache_size: {},\n\
\x20    l2_cache_size: {},\n\
\x20    l3_cache_size: {},\n\
)", l1i_cache_size, l1d_cache_size, l2_cache_size, l3_cache_size)]
pub struct CacheInfo {
    /// Size of L1 instruction cache in byte.
    l1i_cache_size: usize,
    /// Size of L1 data cache in byte.
    l1d_cache_size: usize,
    /// Size of L2 in byte.
    l2_cache_size: usize,
    /// Size of L2 in byte.
    l3_cache_size: usize,
}

impl CacheInfo {

    pub fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = CacheInfo {
            l1i_cache_size: parse_sysctl_value(
                "l1i_cache_size",
                SysctlKey::HwL1icachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            l1d_cache_size: parse_sysctl_value(
                "l1d_cache_size",
                SysctlKey::HwL1dcachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            l2_cache_size: parse_sysctl_value(
                "l2_cache_size",
                SysctlKey::HwL2cachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            l3_cache_size: parse_sysctl_value(
                "l3_cache_size",
                SysctlKey::HwL3cachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
        };
        Ok(x)
    }

    /// Size of L1 instruction cache in byte.
    pub fn l1i_cache_size(&self) -> usize {
        self.l1i_cache_size
    }

    /// Size of L1 data cache in byte.
    pub fn l1d_cache_size(&self) -> usize {
        self.l1d_cache_size
    }

    /// Size of L2 in byte.
    pub fn l2_cache_size(&self) -> usize {
        self.l2_cache_size
    }

    /// Size of L2 in byte.
    pub fn l3_cache_size(&self) -> usize {
        self.l3_cache_size
    }

    /// Size of L1 instruction cache in kibibyte.
    pub fn l1i_cache_size_kb(&self) -> f64 {
        self.l1i_cache_size as f64 / 1024_f64
    }

    /// Size of L1 data cache in kibibyte.
    pub fn l1d_cache_size_kb(&self) -> f64 {
        self.l1d_cache_size as f64 / 1024_f64
    }

    /// Size of L2 cache in kibibyte.
    pub fn l2_cache_size_kb(&self) -> f64 {
        self.l2_cache_size as f64 / 1024_f64
    }

    /// Size of L3 cache in kibibyte.
    pub fn l3_cache_size_kb(&self) -> f64 {
        self.l3_cache_size as f64 / 1024_f64
    }
}

