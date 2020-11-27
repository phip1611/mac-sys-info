use derive_more::Display as DeriveMoreDisplay;
use serde::{Serialize};
use std::collections::BTreeMap;
use crate::error::MacSysInfoError;
use crate::parse::{parse_sysctl_value, ParseAsType};
use crate::generated_sysctl_keys::SysctlKey;
use crate::structs::cpu_architecture_info::CpuArchitectureInfo;

#[derive(Debug, Serialize, DeriveMoreDisplay)]
#[allow(non_camel_case_types)]
pub enum CpuFeaturesInfo {
    // #[display(fmt = "AppleSi(\n{}\n)", _0)]
    AppleSi(AppleSiCpuFeaturesInfo),
    // #[display(fmt = "X86(\n{}\n)", _0)]
    X86_64(X86_64CpuFeaturesInfo),
}

impl CpuFeaturesInfo {
    pub fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let arch = CpuArchitectureInfo::determine_architecture(sysinfo)?;
        if arch.is_x86_64() {
            Ok(
                CpuFeaturesInfo::X86_64(
                    X86_64CpuFeaturesInfo::new(sysinfo)?
                )
            )
        } else {
            Ok(
                CpuFeaturesInfo::AppleSi(
                    AppleSiCpuFeaturesInfo::new(sysinfo)?
                )
            )
        }
    }

    pub fn is_apple_si(&self) -> bool {
        match self {
            CpuFeaturesInfo::AppleSi(_) => { true }
            CpuFeaturesInfo::X86_64(_) => { false }
        }
    }

    pub fn is_x86(&self) -> bool {
        !self.is_apple_si()
    }

    pub fn get_apple_si_features(self) -> AppleSiCpuFeaturesInfo {
        if let CpuFeaturesInfo::AppleSi(val) = self {
            val
        } else {
            panic!("Not Apple Silicon!")
        }
    }

    pub fn get_x86_features(self) -> X86_64CpuFeaturesInfo {
        if let CpuFeaturesInfo::X86_64(val) = self {
            val
        } else {
            panic!("Not x86!")
        }
    }
}

/// Apple silicon specific CPU features.
#[derive(Debug, Serialize, DeriveMoreDisplay)]
pub struct AppleSiCpuFeaturesInfo {
    // TODO, anyone got one? :D
}

impl AppleSiCpuFeaturesInfo {
    pub fn new(_sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        Ok(AppleSiCpuFeaturesInfo {})
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
pub struct X86_64CpuFeaturesInfo {
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

impl X86_64CpuFeaturesInfo {

    pub fn new(sysinfo: &BTreeMap<String, String>) -> Result<Self, MacSysInfoError> {
        let x = X86_64CpuFeaturesInfo {
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

