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

//! The main struct that encapsulated all other info structs.

use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::structs::cpu_info::CpuInfo;
use crate::structs::os_info::OsInfo;
use crate::structs::mem_info::MemInfo;
use crate::error::MacSysInfoError;
use crate::structs::cpu_features_info::CpuFeaturesInfo;

/// Abstraction over MacOS system info.
/// Allows raw access to all obtained system configuration as
/// well as convenient abstractions over them.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "MacSysInfo (\n\
\x20    all_keys: <Map>,\n\
\x20    cpu_info: <CpuInfo>,\n\
\x20    cpu_features: <CpuFeaturesInfo>,\n\
\x20    os_info: <OsInfo>,\n\
\x20    mem_info: <MemoryInfo>,\n\
)")]
pub struct MacSysInfo {
    /// Raw presentation of all keys that `$ sysctl -a` outputs in alphabetically order.
    /// You can use `.name()` on any enum variant of
    /// [`crate::generated_sysctl_keys::SysctlKey`] to access
    /// this. The key is a string and not `SysctlKey` in order to guarantee,
    /// that even if `$ sysctl -a` outputs more keys than known
    /// (e.g. on newer AppleSi Macbooks), all keys can be accessed.
    all_keys: BTreeMap<String, String>,
    /// Basic CPU info including Cache info.
    cpu_info: CpuInfo,
    /// Advanced CPU info.
    cpu_features: CpuFeaturesInfo,
    /// Software info.
    os_info: OsInfo,
    /// Memory info.
    mem_info: MemInfo,
}

impl MacSysInfo {
    /// Constructor.
    pub(crate) fn new(all_keys: BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = MacSysInfo {
            cpu_info: CpuInfo::new(&all_keys)?,
            cpu_features: CpuFeaturesInfo::new(&all_keys)?,
            os_info: OsInfo::new(&all_keys)?,
            mem_info: MemInfo::new(&all_keys)?,
            all_keys,
        };
        Ok(x)
    }

    /// Getter for `all_keys`.
    pub fn all_keys(&self) -> &BTreeMap<String, String> {
        &self.all_keys
    }

    /// Getter for [`crate::structs::CpuFeaturesInfo`].
    pub fn cpu_features(&self) -> &CpuFeaturesInfo {
        &self.cpu_features
    }

    /// Getter for [`crate::structs::CpuInfo`].
    pub fn cpu_info(&self) -> &CpuInfo {
        &self.cpu_info
    }

    /// Getter for [`crate::structs::OsInfo`].
    pub fn os_info(&self) -> &OsInfo {
        &self.os_info
    }

    /// Getter for [`crate::structs::MemoryInfo`].
    pub fn mem_info(&self) -> &MemInfo {
        &self.mem_info
    }

}
