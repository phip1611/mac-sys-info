use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::HashMap;
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

    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        CpuArchitectureInfo::determine_architecture(sysinfo)
    }

    pub fn determine_architecture(sysinfo: &HashMap<String, String>) -> Result<CpuArchitectureInfo, MacSysInfoError> {
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


