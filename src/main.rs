#![no_main]
#![no_std]
#![feature(abi_efiapi)]

extern crate alloc;

use alloc::vec::Vec;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
use tinybmp::Bmp;
use uefi::prelude::*;
use uefi::proto::console::gop::{BltOp, BltPixel, BltRegion, GraphicsOutput};
use uefi::ResultExt;

fn draw_bmp(gop: &mut GraphicsOutput) {
    let bmp_data = include_bytes!("../image.bmp");
    let bmp = Bmp::<Rgb888>::from_slice(bmp_data).unwrap();

    let width: usize = bmp.as_raw().size().width.try_into().unwrap();
    let height: usize = bmp.as_raw().size().height.try_into().unwrap();

    let mut buffer = Vec::with_capacity(width * height);
    for pixel in bmp.pixels() {
        let color = pixel.1;
        buffer.push(BltPixel::new(color.r(), color.g(), color.b()));
    }

    let op = BltOp::BufferToVideo {
        buffer: &buffer,
        src: BltRegion::Full,
        dest: (0, 0),
        dims: (width, height),
    };

    gop.blt(op).expect_success("Failed to draw bmp");
}

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap_success();

    let bt = system_table.boot_services();

    let gop = bt.locate_protocol::<GraphicsOutput>().unwrap_success();
    let gop = unsafe { &mut *gop.get() };

    draw_bmp(gop);

    Status::SUCCESS
}
