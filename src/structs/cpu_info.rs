use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::HashMap;
use crate::error::MacSysInfoError;
use crate::generated_sysctl_keys::SysctlKey;
use crate::structs::cache_info::CacheInfo;
use crate::parse::{parse_sysctl_value, ParseAsType};
use crate::structs::cpu_architecture_info::CpuArchitectureInfo;


/// General CPU info.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "CpuInfo (\n\
\x20    phys_cores: {},\n\
\x20    logic_cores: {},\n\
\x20    num_cores: {},\n\
\x20    cache_info: <nested structure; print directly>,\n\
\x20    brand_string: {},\n\
\x20    frequency: {},\n\
\x20    min_frequency: {},\n\
\x20    max_frequency: {},\n\
\x20    architecture: {},\n\
)", phys_cores, logic_cores, num_cores, brand_string, frequency, min_frequency, max_frequency, architecture)]
pub struct CpuInfo {
    /// Physical cores on the CPU.
    phys_cores: usize,
    /// Logical cores on the CPU. Due to hardware multithreading (Like Intel Hyperthreading)
    /// this can be more than the physical cores.
    logic_cores: usize,
    /// Actual available cores on the system.
    num_cores: usize,
    /// Information about the CPU cache.
    cache_info: CacheInfo,
    /// For example "Intel(R) Core(TM) i5-5257U CPU @ 2.70GH"
    brand_string: String,
    /// Base frequency in Hertz.
    frequency: usize,
    /// Min frequency in Hertz.
    min_frequency: usize,
    /// Max frequency in Hertz.
    max_frequency: usize,
    /// Architecture. x86_64 or AppleSi
    architecture: CpuArchitectureInfo,
}

impl CpuInfo {

    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = CpuInfo {
            phys_cores: parse_sysctl_value(
                "phys_cores",
                SysctlKey::HwPhysicalcpu,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            logic_cores: parse_sysctl_value(
                "logic_cores",
                SysctlKey::HwLogicalcpu,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            num_cores: parse_sysctl_value(
                "num_cores",
                SysctlKey::HwActivecpu,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            brand_string: parse_sysctl_value(
                "brand_string",
                SysctlKey::MachdepCpuBrand_string,
                sysinfo,
                ParseAsType::String)?
                .get_string(),
            frequency: parse_sysctl_value(
                "frequency",
                SysctlKey::HwCpufrequency,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            min_frequency: parse_sysctl_value(
                "min_frequency",
                SysctlKey::HwCpufrequency_min,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            max_frequency: parse_sysctl_value(
                "max_frequency",
                SysctlKey::HwCpufrequency_max,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            cache_info: CacheInfo::new(sysinfo)?,
            architecture: CpuArchitectureInfo::new(sysinfo)?,
        };
        Ok(x)
    }

    pub fn phys_cores(&self) -> usize {
        self.phys_cores
    }

    pub fn logic_cores(&self) -> usize {
        self.logic_cores
    }

    pub fn num_cores(&self) -> usize {
        self.num_cores
    }

    pub fn cache_info(&self) -> &CacheInfo {
        &self.cache_info
    }
    pub fn brand_string(&self) -> &str {
        &self.brand_string
    }
    pub fn frequency(&self) -> usize {
        self.frequency
    }
    pub fn min_frequency(&self) -> usize {
        self.min_frequency
    }
    pub fn max_frequency(&self) -> usize {
        self.max_frequency
    }
    pub fn frequency_ghz(&self) -> f64 {
        self.frequency as f64 / (1E9 as f64)
    }
    pub fn min_frequency_ghz(&self) -> f64 {
        self.min_frequency as f64 / (1E9 as f64)
    }
    pub fn max_frequency_ghz(&self) -> f64 {
        self.max_frequency as f64 / (1E9 as f64)
    }
}
