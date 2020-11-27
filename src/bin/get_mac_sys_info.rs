
use mac_sys_info::get_mac_sys_info;
use mac_sys_info::generated_sysctl_keys::SysctlKey;

fn main() {
    let info = get_mac_sys_info().unwrap();
    // even if IDE shows that the Display trait is missing:
    // it is there. It gets implemented during compile time
    // by "derive_macro" crate.
    println!("{}", info.cpu_info());
    println!("{}", info.cpu_info().cache_info());
    println!("{}", info.cpu_features());
    println!("{}", info.os_info());
    println!("{}", info.mem_info());
    println!("CPU @ {} Ghz", info.cpu_info().frequency_ghz());
    println!("L3 Cache size extracted manually: {}",
             info.all_keys().get(SysctlKey::HwL3cachesize.name()).unwrap()
    );
    println!("L2 Cache Size in KB: {}",
             info.cpu_info().cache_info().l2_cache_size_kb()
    );
    println!("System has {} GB of RAM", info.mem_info().total_memory_gb())
}