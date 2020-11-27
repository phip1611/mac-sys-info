use mac_sys_info::generated_sysctl_keys::SysctlKey;

fn main() {
    // even if IDE (CLion) tells that Display is not implemented:
    // this is not true: it get's implemented during build by
    // "derive_more"-crates code generation
    println!("{}", SysctlKey::list()[0]);
    println!("{:#?}", SysctlKey::list());
}