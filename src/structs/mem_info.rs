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

//! Memory info.

use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::parse::{parse_sysctl_value, ParseAsType};
use crate::error::MacSysInfoError;
use crate::generated_sysctl_keys::SysctlKey;

/// Memory (RAM) info.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "MemoryInfo (\n\
\x20    total_memory: {},\n\
)", total_memory)]
pub struct MemInfo {
    total_memory: usize,
}

impl MemInfo {
    /// Constructor.
    pub(crate) fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let info = MemInfo {
            total_memory: parse_sysctl_value(
                "total_memory",
                SysctlKey::HwMemsize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
        };
        Ok(info)
    }

    /// Returns the amount of total memory in bytes.
    pub fn total_memory(&self) -> usize {
        self.total_memory
    }

    /// Returns the amount of total memory in MibiBytes.
    pub fn total_memory_mb(&self) -> usize {
        self.total_memory / 1024 / 1024
    }
    /// Returns the amount of total memory in GibiBytes.
    pub fn total_memory_gb(&self) -> usize {
        self.total_memory / 1024 / 1024 / 1024
    }
}
