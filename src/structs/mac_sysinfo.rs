// use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::structs::cpu_info::CpuInfo;
use crate::structs::os_info::OsInfo;
use crate::structs::mem_info::MemoryInfo;
use crate::error::MacSysInfoError;
use crate::structs::cpu_features_info::CpuFeaturesInfo;

/// Abstraction over MacOS system info.
/// Allows raw access to all obtained system configuration as
/// well as convenient abstractions over them.
#[derive(Debug, Serialize)]
// #[derive(Debug, Serialize, DeriveMoreDisplay)]
// BTreeMap doesn't implement display :(
/*#[display(fmt = "MacSysInfo (\
\x20    all_keys: {},\
\x20    cpu_features: {},\
)", all_keys, cache_info, cpu_features)]*/
pub struct MacSysInfo {
    /// Raw presentation of all keys that `$ sysctl -a` outputs in alphabetically order.
    /// You can use `.name()` on any enum variant of
    /// `[crate::generated_sysctl_keys::SysctlKey]` to access
    /// this. The key is a string and not `SysctlKey` in order to guarantee,
    /// that even if `$ sysctl -a` outputs more keys than known
    /// (e.g. on newer AppleSi Macbooks), all keys can be accessed.
    all_keys: BTreeMap<String, String>,
    cpu_info: CpuInfo,
    cpu_features: CpuFeaturesInfo,
    os_info: OsInfo,
    mem_info: MemoryInfo,
}

impl MacSysInfo {
    pub fn new(all_keys: BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = MacSysInfo {
            cpu_info: CpuInfo::new(&all_keys)?,
            cpu_features: CpuFeaturesInfo::new(&all_keys)?,
            os_info: OsInfo::new(&all_keys)?,
            mem_info: MemoryInfo::new(&all_keys)?,
            all_keys,
        };
        Ok(x)
    }

    pub fn all_keys(&self) -> &BTreeMap<String, String> {
        &self.all_keys
    }

    pub fn cpu_features(&self) -> &CpuFeaturesInfo {
        &self.cpu_features
    }

    pub fn cpu_info(&self) -> &CpuInfo {
        &self.cpu_info
    }

    pub fn os_info(&self) -> &OsInfo {
        &self.os_info
    }

    pub fn mem_info(&self) -> &MemoryInfo {
        &self.mem_info
    }

}
