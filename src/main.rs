mod util;
use crate::util::fetch::fetch;

fn main() {
    println!("{username}@{hostname}",
             username = fetch::username(),
             hostname = fetch::hostname());

    println!("OS\t{}", fetch::osname());

    println!("KERNEL\t{}", fetch::kernel());

    println!("EDITOR\t{}", fetch::editor());

    println!("TERM\t{}", fetch::terminal());

    println!("DESKTOP\t{}", fetch::desktop());

    println!("SHELL\t{}", fetch::shell());
}
