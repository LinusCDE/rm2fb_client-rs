//! Just to test it's working.
//! Many stuff is taken from
//! https://github.com/canselcik/libremarkable/ .
//! Also a lot of now magic values.

use std::ffi::CString;
use std::os::raw::c_char;
use std::thread;
use std::time::Duration;

/// Source: https://github.com/canselcik/libremarkable/blob/340bc9ab838562997628620db14c300bae7e422f/src/framebuffer/mxcfb.rs#L51
#[derive(Debug)]
#[repr(C)]
pub struct mxcfb_update_data {
    pub update_region: mxcfb_rect,
    pub waveform_mode: u32,
    pub update_mode: u32,
    pub update_marker: u32,
    pub temp: i32,
    pub flags: u32,
    pub dither_mode: i32,
    pub quant_bit: i32,
    pub alt_buffer_data: mxcfb_alt_buffer_data,
}

/// Source: https://github.com/canselcik/libremarkable/blob/340bc9ab838562997628620db14c300bae7e422f/src/framebuffer/mxcfb.rs#L36
#[derive(Debug)]
#[repr(C)]
pub struct mxcfb_alt_buffer_data {
    pub phys_addr: u32,
    pub width: u32,
    pub height: u32,
    pub alt_update_region: mxcfb_rect,
}

/// Source: https://github.com/canselcik/libremarkable/blob/6e5ce1508157e7f0978284e10e9069c61bd35284/src/framebuffer/common.rs#L157
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct mxcfb_rect {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    unsafe {
        rm2fb_client::init();
        println!("Ran init()");

        let c_str = CString::new("/dev/fb0").unwrap();
        let fd = rm2fb_client::open(c_str.as_ptr() as *const c_char, 2 /* O_RDWR */, 0);
        println!("Opened /dev/fb0. fd is {}", fd);

        let refresh_data = mxcfb_update_data {
            dither_mode: 0x30_0f30,
            temp: 0xFFFF,
            waveform_mode: 0x02,
            flags: 0,
            quant_bit: 0,
            update_region: mxcfb_rect {
                top: 0,
                left: 0,
                width: 1404,
                height: 1872,
            },
            alt_buffer_data: mxcfb_alt_buffer_data {
                alt_update_region: mxcfb_rect {
                    top: 0,
                    left: 0,
                    width: 0,
                    height: 0,
                },
                width: 0,
                height: 0,
                phys_addr: 0,
            },
            update_marker: 1337,
            update_mode: 1,
        };

        rm2fb_client::ioctl(fd, 0x4048462e, &refresh_data);

        println!("Full refresh triggered. Exiting in 3 seconds...");
        thread::sleep(Duration::from_secs(3));

        // Let linux clean up the open fd. xD
    }
    Ok(())
}
