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
use x86::segmentation::{
    DescriptorBuilder, 
    BuildDescriptor,
    DataSegmentType, 
    CodeSegmentType, 
    SegmentDescriptorBuilder,
    Descriptor,
    };
use x86::dtables::{DescriptorTablePointer, lgdt};
use x86::Ring;

#[no_mangle]
pub fn _start() -> ! {
    let slice = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };
    let mut vga = Vga::new(slice);

    let null_descriptor = Descriptor::NULL;
    let data_descriptor:Descriptor = DescriptorBuilder::data_descriptor(0, 4*1024*1024, DataSegmentType::ReadWrite)
        .present()
        .dpl(Ring::Ring0)
        .limit_granularity_4kb()
        .db()
        .finish();
    let code_descriptor = DescriptorBuilder::code_descriptor(0, 4*1024*1024, CodeSegmentType::ExecuteRead)
        .present()
        .dpl(Ring::Ring0)
        .limit_granularity_4kb()
        .db()
        .finish();

    let gdt_pointer = DescriptorTablePointer::new_from_slice(&[null_descriptor, data_descriptor, code_descriptor]);

    kprintln!(vga, "Loading GDT...");
    unsafe { lgdt(&gdt_pointer) };
    kprintln!(vga, "Done!");

    kprintln!(vga, "hello world");

    loop {}
}
