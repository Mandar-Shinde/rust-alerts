//! A simple message box tool which is wrappers around C APIs.
//!
//!
//! # Example
//!
//! ```
//! #[macro_use]
//! extern crate alerts;
//!
//! fn main() {
//!     alert("Alert","Hello world");
//! }
//! ```

#[cfg(windows)] extern crate winapi;
use std::io::Error;

#[cfg(windows)]
fn alert_message(head: &str,msg: &str,_wflags: i32 ) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let whead: Vec<u16> = OsStr::new(head).encode_wide().chain(once(0)).collect();
    let wmsg: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wmsg.as_ptr(), whead.as_ptr(), MB_OK|winapi::um::winuser::MB_ICONEXCLAMATION)
    };
    if ret == 0 { Err(Error::last_os_error()) }
        else { Ok(ret) }
}
#[cfg(not(windows))]
fn alert_message(head: &str,msg: &str,_wflags: i32) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}

pub fn alert(head: &str,msg: &str) {
    alert_message( head,msg,0 ).unwrap();
}
