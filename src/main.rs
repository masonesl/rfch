mod util;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

use crate::util::fetch::fetch;

fn main() {
    let system_info: System = System::new_with_specifics(
        RefreshKind::new()
            .with_cpu(CpuRefreshKind::new())
            .with_memory(MemoryRefreshKind::new().with_ram()),
    );

    println!(
        "{username}@{hostname}",
        username = fetch::username(),
        hostname = fetch::hostname()
    );

    println!("OS\t{}", fetch::osname());

    println!("KERNEL\t{}", fetch::kernel());

    println!("EDITOR\t{}", fetch::editor());

    println!("TERM\t{}", fetch::terminal());

    println!("DESKTOP\t{}", fetch::desktop());

    println!("SHELL\t{}", fetch::shell());

    println!("CPU\t{}", fetch::cpu(&system_info));
}
