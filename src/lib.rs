//! Library to get Mac-specific system info. The information is
//! retrieved from the `$ sysctl -a` command.

use crate::error::MacSysInfoError;
use std::collections::HashMap;
use unix_exec_output_catcher::fork_exec_and_catch;
use crate::generated_sysctl_keys::SysctlKey;
use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use crate::parse::{parse_sysctl_value, ParseAsType};

pub mod error;
pub mod generated_sysctl_keys;
mod parse;

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "MemoryInfo (\n\
\x20    total_memory: {},\n\
)", total_memory)]
pub struct MemoryInfo {
    total_memory: usize,
}

impl MemoryInfo {
    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
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
}

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[allow(non_camel_case_types)]
pub enum CpuArchitecture {
    X86_64,
    AppleSi,
}

impl CpuArchitecture {

    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        CpuArchitecture::determine_architecture(sysinfo)
    }

    pub fn determine_architecture(sysinfo: &HashMap<String, String>) -> Result<CpuArchitecture, MacSysInfoError> {
        let is_x86 = sysinfo.get(SysctlKey::KernVersion.name())
            .ok_or_else(|| MacSysInfoError::KeyNotFound(SysctlKey::KernVersion))?
            .to_lowercase()
            .contains("x86_64");
        if is_x86 {
            Ok(CpuArchitecture::X86_64)
        } else {
            Ok(CpuArchitecture::AppleSi)
        }
    }

    pub fn is_x86_64(&self) -> bool {
        match self {
            CpuArchitecture::X86_64 => { true }
            CpuArchitecture::AppleSi => { false }
        }
    }

    pub fn is_apple_si(&self) -> bool {
        !self.is_x86_64()
    }
}

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

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "CacheInfo (\n\
\x20    l1i_cache_size: {},\n\
\x20    l1d_cache_size: {},\n\
\x20    l2_cache_size: {},\n\
\x20    l3_cache_size: {},\n\
)", l1i_cache_size, l1d_cache_size, l2_cache_size, l3_cache_size)]
pub struct CacheInfo {
    /// Size of L1 instruction cache in byte.
    l1i_cache_size: usize,
    /// Size of L1 data cache in byte.
    l1d_cache_size: usize,
    /// Size of L2 in byte.
    l2_cache_size: usize,
    /// Size of L2 in byte.
    l3_cache_size: usize,
}

impl CacheInfo {

    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = CacheInfo {
            l1i_cache_size: parse_sysctl_value(
                "l1i_cache_size",
                SysctlKey::HwL1icachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            l1d_cache_size: parse_sysctl_value(
                "l1d_cache_size",
                SysctlKey::HwL1dcachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            l2_cache_size: parse_sysctl_value(
                "l2_cache_size",
                SysctlKey::HwL2cachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
            l3_cache_size: parse_sysctl_value(
                "l3_cache_size",
                SysctlKey::HwL3cachesize,
                sysinfo,
                ParseAsType::Usize)?
                .get_usize(),
        };
        Ok(x)
    }

    /// Size of L1 instruction cache in byte.
    pub fn l1i_cache_size(&self) -> usize {
        self.l1i_cache_size
    }

    /// Size of L1 data cache in byte.
    pub fn l1d_cache_size(&self) -> usize {
        self.l1d_cache_size
    }

    /// Size of L2 in byte.
    pub fn l2_cache_size(&self) -> usize {
        self.l2_cache_size
    }

    /// Size of L2 in byte.
    pub fn l3_cache_size(&self) -> usize {
        self.l3_cache_size
    }

    /// Size of L1 instruction cache in kibibyte.
    pub fn l1i_cache_size_kb(&self) -> f64 {
        self.l1i_cache_size as f64 / 1024_f64
    }

    /// Size of L1 data cache in kibibyte.
    pub fn l1d_cache_size_kb(&self) -> f64 {
        self.l1d_cache_size as f64 / 1024_f64
    }

    /// Size of L2 cache in kibibyte.
    pub fn l2_cache_size_kb(&self) -> f64 {
        self.l2_cache_size as f64 / 1024_f64
    }

    /// Size of L3 cache in kibibyte.
    pub fn l3_cache_size_kb(&self) -> f64 {
        self.l3_cache_size as f64 / 1024_f64
    }
}

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
)", phys_cores, logic_cores, num_cores, brand_string, frequency, min_frequency, max_frequency)]
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
    architecture: CpuArchitecture,
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
            architecture: CpuArchitecture::new(sysinfo)?,
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

/// Apple silicon specific CPU features.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
pub struct AppleSiCpuFeatures {
    // TODO, anyone got one? :D
}

impl AppleSiCpuFeatures {
    pub fn new(_sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        Ok(AppleSiCpuFeatures {})
    }
}

/// x86 specific CPU features, like AVX.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[display(fmt = "X86CpuFeatures (\n\
\x20    mmx: {},\n\
\x20    sse: {},\n\
\x20    sse2: {},\n\
\x20    sse3: {},\n\
\x20    sse4_1: {},\n\
\x20    sse4_2: {},\n\
\x20    aes: {},\n\
\x20    avx1_0: {},\n\
\x20    avx2_0: {},\n\
\x20    avx512f: {},\n\
\x20    avx512cd: {},\n\
\x20    avx512dq: {},\n\
\x20    avx512bw: {},\n\
\x20    avx512vl: {},\n\
\x20    avx512ifma: {},\n\
\x20    avx512vbmi: {},\n\
)", mmx, sse, sse2, sse3, sse4_1, sse4_2, aes, avx1_0, avx2_0, avx512f, avx512cd, avx512dq, avx512bw, avx512vl, avx512ifma, avx512vbmi)]
#[allow(non_camel_case_types)]
pub struct X86_64CpuFeatures {
    mmx: bool,
    sse: bool,
    sse2: bool,
    sse3: bool,
    sse4_1: bool,
    sse4_2: bool,
    aes: bool,
    avx1_0: bool,
    avx2_0: bool,
    avx512f: bool,
    avx512cd: bool,
    avx512dq: bool,
    avx512bw: bool,
    avx512vl: bool,
    avx512ifma: bool,
    avx512vbmi: bool,
}

impl X86_64CpuFeatures {

    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = X86_64CpuFeatures {
            mmx: parse_sysctl_value(
                "mmx",
                SysctlKey::HwOptionalMmx,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            sse: parse_sysctl_value(
                "sse",
                SysctlKey::HwOptionalSse,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            sse2: parse_sysctl_value(
                "sse2",
                SysctlKey::HwOptionalSse2,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            sse3: parse_sysctl_value(
                "sse3",
                SysctlKey::HwOptionalSse3,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            sse4_1: parse_sysctl_value(
                "sse4_1",
                SysctlKey::HwOptionalSse4_1,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            sse4_2: parse_sysctl_value(
                "sse4_2",
                SysctlKey::HwOptionalSse4_2,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            aes: parse_sysctl_value(
                "aes",
                SysctlKey::HwOptionalAes,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx1_0: parse_sysctl_value(
                "avx1_0",
                SysctlKey::HwOptionalAvx1_0,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx2_0: parse_sysctl_value(
                "avx2_0",
                SysctlKey::HwOptionalAvx2_0,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx512f: parse_sysctl_value(
                "avx512f",
                SysctlKey::HwOptionalAvx512f,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx512cd: parse_sysctl_value(
                "avx512cd",
                SysctlKey::HwOptionalAvx512cd,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx512dq: parse_sysctl_value(
                "avx512dq",
                SysctlKey::HwOptionalAvx512dq,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx512bw: parse_sysctl_value(
                "avx512bw",
                SysctlKey::HwOptionalAvx512bw,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx512vl: parse_sysctl_value(
                "avx512vl",
                SysctlKey::HwOptionalAvx512vl,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx512ifma: parse_sysctl_value(
                "avx512ifma",
                SysctlKey::HwOptionalAvx512ifma,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
            avx512vbmi: parse_sysctl_value(
                "avx512vbmi",
                SysctlKey::HwOptionalAvx512vbmi,
                sysinfo,
                ParseAsType::Bool)?
                .get_bool(),
        };
        Ok(x)
    }

    pub fn mmx(&self) -> bool {
        self.mmx
    }
    pub fn sse(&self) -> bool {
        self.sse
    }
    pub fn sse2(&self) -> bool {
        self.sse2
    }
    pub fn sse3(&self) -> bool {
        self.sse3
    }
    pub fn sse4_1(&self) -> bool {
        self.sse4_1
    }
    pub fn sse4_2(&self) -> bool {
        self.sse4_2
    }
    pub fn aes(&self) -> bool {
        self.aes
    }
    pub fn avx1_0(&self) -> bool {
        self.avx1_0
    }
    pub fn avx2_0(&self) -> bool {
        self.avx2_0
    }
    pub fn avx512f(&self) -> bool {
        self.avx512f
    }
    pub fn avx512cd(&self) -> bool {
        self.avx512cd
    }
    pub fn avx512dq(&self) -> bool {
        self.avx512dq
    }
    pub fn avx512bw(&self) -> bool {
        self.avx512bw
    }
    pub fn avx512vl(&self) -> bool {
        self.avx512vl
    }
    pub fn avx512ifma(&self) -> bool {
        self.avx512ifma
    }
    pub fn avx512vbmi(&self) -> bool {
        self.avx512vbmi
    }
}

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[allow(non_camel_case_types)]
pub enum CpuFeatures {
    // #[display(fmt = "AppleSi(\n{}\n)", _0)]
    AppleSi(AppleSiCpuFeatures),
    // #[display(fmt = "X86(\n{}\n)", _0)]
    X86_64(X86_64CpuFeatures),
}

impl CpuFeatures {
    pub fn new(sysinfo: &HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        let arch = CpuArchitecture::determine_architecture(sysinfo)?;
        if arch.is_x86_64() {
            Ok(
                CpuFeatures::X86_64(
                    X86_64CpuFeatures::new(sysinfo)?
                )
            )
        } else {
            Ok(
                CpuFeatures::AppleSi(
                    AppleSiCpuFeatures::new(sysinfo)?
                )
            )
        }
    }

    pub fn is_apple_si(&self) -> bool {
        match self {
            CpuFeatures::AppleSi(_) => { true }
            CpuFeatures::X86_64(_) => { false }
        }
    }

    pub fn is_x86(&self) -> bool {
        !self.is_apple_si()
    }

    pub fn get_apple_si_features(self) -> AppleSiCpuFeatures {
        if let CpuFeatures::AppleSi(val) = self {
            val
        } else {
            panic!("Not Apple Silicon!")
        }
    }

    pub fn get_x86_features(self) -> X86_64CpuFeatures {
        if let CpuFeatures::X86_64(val) = self {
            val
        } else {
            panic!("Not x86!")
        }
    }
}

/// Abstraction over MacOS system info.
#[derive(Debug, Serialize)]
// #[derive(Debug, Serialize, DeriveMoreDisplay)]
// HashMap doesn't implement display :(
/*#[display(fmt = "MacSysInfo (\
\x20    all_keys: {},\
\x20    cpu_features: {},\
)", all_keys, cache_info, cpu_features)]*/
pub struct MacSysInfo {
    /// Raw presentation of all keys that `$ sysctl -a` outputs.
    /// You can use `.name()` on any enum variant of
    /// `[crate::generated_sysctl_keys::SysctlKey]` to access
    /// this. The key is a string and not `SysctlKey` in order to guarantee,
    /// that even if `$ sysctl -a` outputs more keys than known
    /// (e.g. on newer AppleSi Macbooks), all keys can be
    /// accessed.
    all_keys: HashMap<String, String>,
    cpu_info: CpuInfo,
    cpu_features: CpuFeatures,
    os_info: OsInfo,
    mem_info: MemoryInfo,
}

impl MacSysInfo {
    pub fn new(all_keys: HashMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = MacSysInfo {
            cpu_info: CpuInfo::new(&all_keys)?,
            cpu_features: CpuFeatures::new(&all_keys)?,
            os_info: OsInfo::new(&all_keys)?,
            mem_info: MemoryInfo::new(&all_keys)?,
            all_keys,
        };
        Ok(x)
    }

    pub fn all_keys(&self) -> &HashMap<String, String> {
        &self.all_keys
    }

    pub fn cpu_features(&self) -> &CpuFeatures {
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

/// Parses a line in the form of `hw.optional.sse: 1`
/// to `("hw.optional.sse", "1")`. The part before the
/// first ":" is the key. The rest is the value.
fn parse_sysctl_line(line: &str) -> (String, String) {
    let str_parts = line.split(":")
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let key = str_parts[0].to_string();
    let mut value = String::new();
    for i in 1..str_parts.len() {
        value.push_str(
            str_parts[i]
        )
    }

    (key, value)
}

fn fetch_info_from_sysctl() -> Result<Vec<(String, String)>, MacSysInfoError> {
    let res = fork_exec_and_catch("sysctl", vec!["sysctl", "-a"])
        .map_err(|_| MacSysInfoError::CantFetchData)?;
    let res = res.stdout_lines();
    let res = res.iter().map(|s| s.as_str()).collect::<Vec<&str>>();
    let key_value_vector = res.iter()
        .map(|s| parse_sysctl_line(s))
        .collect::<Vec<(String, String)>>();

    Ok(key_value_vector)
}

/// Returns system information about your Mac. The information is retrieved
/// from the `sysctl -a` command.
pub fn get_mac_sys_info() -> Result<MacSysInfo, MacSysInfoError> {
    let key_value_vector = fetch_info_from_sysctl()?;

    // build the raw string to string map of all supported information
    let mut all_keys = HashMap::new();
    key_value_vector.into_iter()
        .for_each(|(sys_key, sys_value)| {
            all_keys.insert(sys_key, sys_value);
        });

    Ok(
        MacSysInfo::new(all_keys)?
    )
}

#[cfg(test)]
mod tests {
    use crate::get_mac_sys_info;

    #[test]
    fn test_get_sys_info() {
        get_mac_sys_info();
    }
}
