//extern crate link_cplusplus;
extern crate frida_gum;
use std::os::raw::{c_char, c_int, c_ulong};

extern "C" {
    pub fn init();
    // (Used definition from libc crate)
    pub fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    /// mode_t == u32
    pub fn open(fd: *const c_char, flags: c_int, mode: u32) -> c_int;
}
