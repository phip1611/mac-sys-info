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

use unix_exec_output_catcher::fork_exec_and_catch;
use mac_sys_info::error::MacSysInfoError;

const SYSCTL_KEYS_ENUM_GEN_NAME: &str = "SysctlKey";

fn main() {

    let res = fork_exec_and_catch("sysctl", vec!["sysctl", "-a"])
        .map_err(|_| MacSysInfoError::CantFetchData).unwrap();

    // list of KeyValue-Pairs. Value is the value as the key is named
    // in the output of `$ sysctl -a`. The Key is a Rust-friendly version of it.
    let macos_sysctl_key_value_pairs = res.stdout_lines()
        .iter()
        .map(|s| String::from(s.as_str()))
        .map(|s| s.split(":")
            .collect::<Vec<&str>>()
            [0].to_string()
        )
        .map(|s| {
            let key = s
                .split(".")
                // we capitalize the first letter of each group
                .map(|s| {
                    s.chars().enumerate().map(|(i, mut c)| {
                        if i == 0 {
                            c = c.to_ascii_uppercase()
                        }
                        c
                    })
                        .collect::<String>()
                })
                .collect::<String>();
            let value = s;

            (key, value)
        })
        .collect::<Vec<(String,String)>>();

    // now transform to Rust enum
    {
        println!();
        println!();
        println!("/// This enum was generated using the binary `sysctl_output_keys_to_rust_code.rs`");
        println!("/// It contains (hopefully) all keys that the output of `$ sysctl -a` can show.");
        println!("/// This includes information about the CPU, the number of cores and caches.");
        println!("/// Might make trouble/inconsistencies with the AppleSi-Macbooks. I can't test it yet.");
        println!("#[derive(Debug, Display, PartialEq, Copy, Clone, Eq, Hash)]");
        println!("#[allow(non_camel_case_types)]");
        println!("pub enum {} {{", SYSCTL_KEYS_ENUM_GEN_NAME);
        /*for (key, value) in macos_sysctl_key_value_pairs {
            print!("    ");
            print!("{}(\"{}\"),", key, value);
            print!("\n");
        }*/
        for (key, _value) in &macos_sysctl_key_value_pairs {
            // print fmt-Macro from derive_more-Display-macro-impl-magic :)
            // see here https://crates.io/crates/derive_more

            // this increased the build to up to one minute because of the thousands
            // of enum variants and the code generation for each :D and as long
            // as this equals to the default value the code generation uses
            // we can remove it

            /*print!("    ");
            print!("#[display(fmt = \"{}\")]", key);
            print!("\n");*/

            // print actual enum variant
            print!("    ");
            print!("{},", key);
            print!("\n");
        }
        println!("}}");
        println!();
    }

    // and impl name() for each
    {
        println!("impl {} {{", SYSCTL_KEYS_ENUM_GEN_NAME);
        println!("    /// Returns the name of the key as it is named in");
        println!("    /// the output of `$ sysctl -a`");
        println!("    pub fn name(&self) -> &'static str {{");
        println!("        match self {{");
        for (key, value) in &macos_sysctl_key_value_pairs {
            print!("            ");
            print!("{}::{} => \"{}\",", SYSCTL_KEYS_ENUM_GEN_NAME, key, value);
            print!("\n");
        }
        println!("        }}");
        println!("    }}");
        println!("}}");
    }

    // and impl list() for enum
    {
        println!("impl {} {{", SYSCTL_KEYS_ENUM_GEN_NAME);
        println!("    /// Returns a vector containing all Enum elements.");
        println!("    pub fn list() -> Vec<{}> {{", SYSCTL_KEYS_ENUM_GEN_NAME);
        println!("        vec![");
        for (key, _value) in &macos_sysctl_key_value_pairs {
            print!("            ");
            print!("{}::{},", SYSCTL_KEYS_ENUM_GEN_NAME, key);
            print!("\n");
        }
        println!("        ]");
        println!("    }}");
        println!("}}");
    }
}