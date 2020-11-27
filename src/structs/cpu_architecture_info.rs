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
use crate::generated_sysctl_keys::SysctlKey;

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum CpuArchitectureInfo {
    X86_64,
    AppleSi,
}


#[allow(dead_code)]
impl CpuArchitectureInfo {

    pub fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        CpuArchitectureInfo::determine_architecture(sysinfo)
    }

    pub fn determine_architecture(sysinfo: &BTreeMap<String, String>) -> Result<CpuArchitectureInfo, MacSysInfoError> {
        let is_x86 = sysinfo.get(SysctlKey::KernVersion.name())
            .ok_or_else(|| MacSysInfoError::KeyNotFound(SysctlKey::KernVersion))?
            .to_lowercase()
            .contains("x86_64");
        if is_x86 {
            Ok(CpuArchitectureInfo::X86_64)
        } else {
            Ok(CpuArchitectureInfo::AppleSi)
        }
    }

    pub fn is_x86_64(&self) -> bool {
        match self {
            CpuArchitectureInfo::X86_64 => { true }
            CpuArchitectureInfo::AppleSi => { false }
        }
    }

    pub fn is_apple_si(&self) -> bool {
        !self.is_x86_64()
    }
}


