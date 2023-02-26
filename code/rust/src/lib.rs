#![no_std]
#![feature(alloc_error_handler)]
#![feature(abi_x86_interrupt)]
#![feature(panic_info_message)]

extern crate alloc;

use alloc::{boxed::Box, string::String};
use lazy_static::lazy_static;
use log::{info, Level};
use memory::kernel_heap;
use serial::Serial;

use crate::{
    fs::{
        rootfs::RootDir,
        vfs::{self, NodeKind},
    },
    panic::die,
};

mod arch;
mod fs;
mod io;
mod memory;
mod panic;
mod serial;
mod timer;

lazy_static! {
    static ref SERIAL: Serial = match Serial::new(serial::COM1, 115200) {
        Ok(s) => s,
        Err(_) => panic!("Failed to initialize serial"),
    };
}

fn init_rootfs() {
    let root_dir: vfs::Directory = vfs::Directory {
        operation: Box::new(RootDir::new()),
    };
    let root_node = vfs::Node {
        name: String::from("root"),
        kind: NodeKind::Directory(root_dir),
    };
    let vfs: vfs::Vfs = vfs::Vfs::new(root_node);

    vfs::set_root(vfs);
}

#[no_mangle]
pub extern "C" fn k_main() -> ! {
    kernel_heap::init();

    if log::set_logger(&*SERIAL).is_err() {
        SERIAL.write_string(&String::from("Error"));
    }

    log::set_max_level(Level::Debug.to_level_filter());

    info!(target:"k_main", "Kernel starting");

    init_rootfs();

    info!(target:"k_main", "Virtual FS initialized");

    arch::x86_64::init();

    info!(target:"k_main", "Arch initialized");

    die();
}
