#![feature(panic_handler)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate rlibc;
extern crate vga;
#[macro_use]
extern crate intermezzos;
extern crate bootloader_precompiled;
extern crate x86;

#[cfg(not(test))]
pub mod panic;

use vga::Vga;
use x86::segmentation::{DescriptorBuilder, DataSegmentType, CodeSegmentType};

#[no_mangle]
pub fn _start() -> ! {
    let dataDescriptor = DescriptorBuilder::data_descriptor(0, 4*1024*1024, DataSegmentType::ReadWrite);
    let codeDescriptor = DescriptorBuilder::code_descriptor(0, 4*1024*1024, CodeSegmentType::ExecuteRead);
    let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

    let mut vga = Vga::new(slice);

    kprintln!(vga, "hello world");

    loop {}
}
