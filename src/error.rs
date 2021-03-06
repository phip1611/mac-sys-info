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
//! Contains all error-related structs and functions that can occur in this library.

use crate::generated_sysctl_keys::SysctlKey;
use derive_more::Display;
use std::error::Error;
use unix_exec_output_catcher::error::UECOError;

/// Errors that can happen inside the library.
#[derive(Debug, Display)]
pub enum MacSysInfoError {
    #[display(fmt = "Can't fetch system data because the library reported the error: {}", _0)]
    CantFetchData(UECOError),
    #[display(fmt = "Could not capture any stdout lines from `$ sysctl -a` execution. There might be an error in crate 'unix-exec-output-catcher'.")]
    NoCapturedOutput,
    #[display(fmt = "Can't parse key '{}' for field '{}' because of: '{}'", sysctl_key, field_name, err_msg)]
    ParseError{field_name: String, sysctl_key: SysctlKey, err_msg: String},
    #[display(fmt = "The key '{}' can't be found in \"sysctl -a\" output.", _0)]
    KeyNotFound(SysctlKey),
}

// IDE might show that display is not implemented but it gets implemented
// during build by "derive_more" crate
impl Error for MacSysInfoError {

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MacSysInfoError::CantFetchData(inner) => {
                Some(inner)
            },
            _ => None
        }
    }
}
