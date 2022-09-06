// Src: https://github.com/vrmiguel/bustd/blob/master/src/errno.rs
use libc::{self, c_int};

unsafe fn _errno() -> *mut c_int {
    libc::__errno_location()
}

pub fn errno() -> i32 {
    unsafe { (*_errno()) as i32 }
}
