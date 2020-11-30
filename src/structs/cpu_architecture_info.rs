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

//! CPU-Architecture info.

use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::error::MacSysInfoError;
use crate::generated_sysctl_keys::SysctlKey;

/// Information about the CPU architecture. Because this library
/// is Mac-specific, there is only X86_64 (since the beginning)
/// and Apple Silicon (AppleSi) since late 2020.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum CpuArchitectureInfo {
    /// x86_64
    x86_64,
    /// Apple Silicon
    AppleSi,
}


#[allow(dead_code)]
impl CpuArchitectureInfo {

    /// Constructor
    pub(crate) fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        CpuArchitectureInfo::determine_architecture(sysinfo)
    }

    /// Reads the CPU architecture from the "sysctl -a" output.
    pub(crate) fn determine_architecture(sysinfo: &BTreeMap<String, String>) -> Result<CpuArchitectureInfo, MacSysInfoError> {
        let is_x86 = sysinfo.get(SysctlKey::KernVersion.name())
            .ok_or(MacSysInfoError::KeyNotFound(SysctlKey::KernVersion))?
            .to_lowercase()
            .contains("x86_64");
        if is_x86 {
            Ok(CpuArchitectureInfo::x86_64)
        } else {
            Ok(CpuArchitectureInfo::AppleSi)
        }
    }

    /// Convenient getter to check if the current system is a x86_64 system.
    pub fn is_x86_64(&self) -> bool {
        match self {
            CpuArchitectureInfo::x86_64 => { true }
            CpuArchitectureInfo::AppleSi => { false }
        }
    }

    /// Convenient getter to check if the current system is a Apple Silicon system.
    pub fn is_apple_si(&self) -> bool {
        !self.is_x86_64()
    }
}


