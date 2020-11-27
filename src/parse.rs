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

//! Parse utility.

use crate::error::MacSysInfoError;
use std::collections::BTreeMap;
use crate::generated_sysctl_keys::SysctlKey;
use derive_more::Display;

#[derive(Debug, PartialEq, Display)]
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
                                 raw_values: &BTreeMap<String, String>,
                                 target: ParseAsType) -> Result<ParsedValue, MacSysInfoError> {
    let raw_value = raw_values.get(key.name())
        .ok_or_else(|| {
            debug!("KeyNotFound Error: field_name='{}', key='{}', target='{}'", &field_name, &key, &target);
            MacSysInfoError::KeyNotFound(key)
        })?;

    let x = match target {
        ParseAsType::String => {
            ParsedValue::String(
                raw_value.to_owned()
            )
        }
        ParseAsType::Isize => {
            ParsedValue::Isize(
                raw_value.parse::<isize>()
                    .map_err(|e| {
                        debug!("Parse Error: field_name='{}', key='{}', target='{}'", &field_name, &key, &target);
                        MacSysInfoError::ParseError{
                            field_name: field_name.to_string(),
                            sysctl_key: key,
                            err_msg: e.to_string(),
                        }
                    })?
            )
        }
        ParseAsType::Usize => {
            ParsedValue::Usize(
                raw_value.parse::<usize>()
                    .map_err(|e| {
                        debug!("Parse Error: field_name='{}', key='{}', target='{}'", &field_name, &key, &target);
                        MacSysInfoError::ParseError{
                            field_name: field_name.to_string(),
                            sysctl_key: key,
                            err_msg: e.to_string(),
                        }
                    })?
            )
        }
        ParseAsType::Bool => {
            let usize = parse_sysctl_value(field_name, key, raw_values, ParseAsType::Usize)
                .map_err(|_e| {
                    debug!("Parse Error: field_name='{}', key='{}', target='{}'", &field_name, &key, &target);
                    MacSysInfoError::ParseError{
                        field_name: field_name.to_string(),
                        sysctl_key: key,
                        err_msg: "Can't parse string '' as boolean. Valid values are '0' or '1'".to_owned()
                    }
                })?
                .get_usize();
            let bool = if usize == 1 { true } else { false };
            ParsedValue::Bool(bool)
        }
    };
    Ok(x)
}

/// Parses a line in the form of `hw.optional.sse: 1`
/// to `("hw.optional.sse", "1")`. The part before the
/// first ":" is the key. The rest is the value.
pub fn parse_sysctl_line(line: &str) -> (String, String) {
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use crate::generated_sysctl_keys::SysctlKey;
    use crate::parse::{parse_sysctl_value, ParseAsType};

    #[test]
    fn test() {
        let mut raw_values = BTreeMap::new();
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
