#![crate_name = "egl"]
#![crate_type = "rlib"]
#![allow(non_camel_case_types)]

extern crate libc;

pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
pub type khronos_uint64_t = libc::uint64_t;
pub type khronos_ssize_t = libc::c_long;
pub type EGLint = libc::int32_t;
pub type EGLNativeDisplayType = *const libc::c_void;
pub type EGLNativePixmapType = *const libc::c_void;
pub type EGLNativeWindowType = *const libc::c_void;
pub type NativeDisplayType = EGLNativeDisplayType;
pub type NativePixmapType = EGLNativePixmapType;
pub type NativeWindowType = EGLNativeWindowType;
include!(concat!(env!("OUT_DIR"), "/egl_bindings.rs"));

