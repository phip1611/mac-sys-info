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

use crate::generated_sysctl_keys::SysctlKey;
use derive_more::Display;

#[derive(Debug, Display)]
pub enum MacSysInfoError {
    #[display(fmt = "Can't fetch system data.")]
    CantFetchData,
    #[display(fmt = "Can't parse key '{}' for field '{}' because of: '{}'", sysctl_key, field_name, err_msg)]
    ParseError{field_name: String, sysctl_key: SysctlKey, err_msg: String},
    #[display(fmt = "The key '{}' can't be found in \"sysctl -a\" output.", _0)]
    KeyNotFound(SysctlKey),
    #[display(fmt = "Unknown error occurred.")]
    Unknown,
}
