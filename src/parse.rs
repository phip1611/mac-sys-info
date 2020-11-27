//! Parse utility.

use crate::error::MacSysInfoError;
use std::collections::HashMap;
use crate::generated_sysctl_keys::SysctlKey;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub(crate) enum ParseAsType {
    String,
    Isize,
    Usize,
    Bool,
}

#[derive(Debug, PartialEq)]
pub(crate) enum ParsedValue {
    String(String),
    Isize(isize),
    Usize(usize),
    Bool(bool),
}

#[allow(dead_code)]
impl ParsedValue {
    pub(crate) fn get_string(self) -> String {
        if let ParsedValue::String(val) = self {
            val
        } else {
            panic!("Not ParseTarget::String")
        }
    }
    pub(crate) fn get_isize(self) -> isize {
        if let ParsedValue::Isize(val) = self {
            val
        } else {
            panic!("Not ParseTarget::Isize")
        }
    }
    pub(crate) fn get_usize(self) -> usize {
        if let ParsedValue::Usize(val) = self {
            val
        } else {
            panic!("Not ParseTarget::Usize")
        }
    }
    pub(crate) fn get_bool(self) -> bool {
        if let ParsedValue::Bool(val) = self {
            val
        } else {
            panic!("Not ParseTarget::Bool")
        }
    }
}



pub(crate) fn parse_sysctl_value(field_name: &str,
                                 key: SysctlKey,
                                 raw_values: &HashMap<String, String>,
                                 target: ParseAsType) -> Result<ParsedValue, MacSysInfoError> {
    let raw_value = raw_values.get(key.name())
        .ok_or_else(|| MacSysInfoError::KeyNotFound(key))?;

    let x = match target {
        ParseAsType::String => {
            ParsedValue::String(
                raw_value.to_owned()
            )
        }
        ParseAsType::Isize => {
            ParsedValue::Isize(
                raw_value.parse::<isize>()
                    .map_err(|e| MacSysInfoError::ParseError{
                        field_name: field_name.to_string(),
                        sysctl_key: key,
                        err_msg: e.to_string(),
                    })?
            )
        }
        ParseAsType::Usize => {
            ParsedValue::Usize(
                raw_value.parse::<usize>()
                    .map_err(|e| MacSysInfoError::ParseError{
                        field_name: field_name.to_string(),
                        sysctl_key: key,
                        err_msg: e.to_string(),
                    })?
            )
        }
        ParseAsType::Bool => {
            let usize = raw_value.parse::<usize>()
                .map_err(|e| MacSysInfoError::ParseError{
                    field_name: field_name.to_string(),
                    sysctl_key: key,
                    err_msg: e.to_string(),
                })?;
            let bool = if usize == 1 { true } else { false };
            ParsedValue::Bool(bool)
        }
    };
    Ok(x)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::generated_sysctl_keys::SysctlKey;
    use crate::parse::{parse_sysctl_value, ParseAsType};

    #[test]
    fn test() {
        let mut raw_values = HashMap::new();
        raw_values.insert(SysctlKey::HwPhysicalcpu.name().to_string(), "4".to_string());
        let parsed = parse_sysctl_value(
            "field_foobar",
            SysctlKey::HwPhysicalcpu,
            &raw_values,
            ParseAsType::Usize,
        );
        println!("{:#?}", parsed);
    }

}
