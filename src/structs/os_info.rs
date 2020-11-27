use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::HashMap;
use crate::error::MacSysInfoError;
use crate::parse::{parse_sysctl_value, ParseAsType};
use crate::generated_sysctl_keys::SysctlKey;

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "OsInfo (\n\
\x20    kern_version: {},\n\
\x20    os_version: {},\n\
)", kern_version, os_version)]
pub struct OsInfo {
    /// e.g. "Darwin Kernel Version 19.6.0: Thu Oct 29 22:56:45 PDT 2020; root:xnu-6153.141.2.2~1/RELEASE_X86_64"
    kern_version: String,
    /// e.g. "10.15.7"
    os_version: String,
}

impl OsInfo {
    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
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

    pub fn kern_version(&self) -> &str {
        &self.kern_version
    }

    pub fn os_version(&self) -> &str {
        &self.os_version
    }
}
