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

//! Info about MacOS/software.

use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::error::MacSysInfoError;
use crate::parse::{parse_sysctl_value, ParseAsType};
use crate::generated_sysctl_keys::SysctlKey;

/// Info about MacOS/software.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "OsInfo (\n\
\x20    kern_version: {},\n\
\x20    os_version: {},\n\
)", kern_version, os_version)]
pub struct OsInfo {
    /// Kern version, e.g. "Darwin Kernel Version 19.6.0: Thu Oct 29 22:56:45 PDT 2020; root:xnu-6153.141.2.2~1/RELEASE_X86_64"
    kern_version: String,
    /// MacOS version, e.g. "10.15.7"
    os_version: String,
}

impl OsInfo {
    /// Constructor.
    pub(crate) fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = Self {
            kern_version: parse_sysctl_value(
                "kern_version",
                SysctlKey::KernVersion,
                sysinfo,
                ParseAsType::String)?
                .get_string(),
            os_version: parse_sysctl_value(
                "os_version",
                SysctlKey::KernOsproductversion,
                sysinfo,
                ParseAsType::String)?
                .get_string(),
        };
        Ok(x)
    }

    /// Getter for the field `kern_version`.
    pub fn kern_version(&self) -> &str {
        &self.kern_version
    }

    /// Getter for the field `os_version`.
    pub fn os_version(&self) -> &str {
        &self.os_version
    }
}
