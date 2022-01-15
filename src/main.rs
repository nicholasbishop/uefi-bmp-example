#![no_main]
#![no_std]
#![feature(abi_efiapi)]

use uefi::prelude::*;
use uefi::ResultExt;
use uefi::proto::console::gop::{BltOp, BltPixel, FrameBuffer, GraphicsOutput, PixelFormat};

// Set a larger graphics mode.
fn set_graphics_mode(gop: &mut GraphicsOutput) {
    // We know for sure QEMU has a 1024x768 mode.
    let mode = gop
        .modes()
        .map(|mode| mode.expect("Warnings encountered while querying mode"))
        .find(|mode| {
            let info = mode.info();
            info.resolution() == (1024, 768)
        })
        .unwrap();

    gop.set_mode(&mode)
        .expect_success("Failed to set graphics mode");
}

// Fill the screen with color.
fn fill_color(gop: &mut GraphicsOutput) {
    let op = BltOp::VideoFill {
        // Cornflower blue.
        color: BltPixel::new(100, 149, 237),
        dest: (0, 0),
        dims: (1024, 768),
    };

    gop.blt(op)
        .expect_success("Failed to fill screen with color");
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap_success();

    let bt = system_table.boot_services();

    let gop = bt.locate_protocol::<GraphicsOutput>().unwrap_success();
    let gop = unsafe { &mut *gop.get() };
    set_graphics_mode(gop);
    fill_color(gop);

    Status::SUCCESS
}
