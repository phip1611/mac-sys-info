use crate::generated_sysctl_keys::SysctlKey;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum MacSysInfoError {
    CantFetchData,
    #[display(fmt = "Can't parse key {} for field {}", sysctl_key, field_name)]
    ParseError{field_name: String, sysctl_key: SysctlKey, err_msg: String},
    KeyNotFound(SysctlKey),
    Unknown,
}
