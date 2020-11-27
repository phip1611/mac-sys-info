use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::parse::{parse_sysctl_value, ParseAsType};
use crate::error::MacSysInfoError;
use crate::generated_sysctl_keys::SysctlKey;

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "MemoryInfo (\n\
\x20    total_memory: {},\n\
)", total_memory)]
pub struct MemoryInfo {
    total_memory: usize,
}

impl MemoryInfo {
    pub fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let info = MemoryInfo {
            total_memory: parse_sysctl_value(
                "total_memory",
                SysctlKey::HwMemsize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
        };
        Ok(info)
    }

    pub fn total_memory(&self) -> usize {
        self.total_memory
    }

    /// Returns the amount of memory in MibiBytes.
    pub fn total_memory_mb(&self) -> usize {
        self.total_memory / 1024 / 1024
    }
    /// Returns the amount of memory in GibiBytes.
    pub fn total_memory_gb(&self) -> usize {
        self.total_memory / 1024 / 1024 / 1024
    }
}
