//! Implements a thin wrapper around the raw select call

use crate::{
    error::Result,
    io::{ InOutHandle, AsInOutHandle }
};
use std::{ convert::TryFrom, io::Error, time::Duration };


/// Declares the FFI types for the shim
mod ffi {
    use std::os::raw::c_int;

    // struct libselect_fd {
    //     uint64_t handle;
    //     bool read;
    //     bool write;
    //     bool exception;
    // };
    #[repr(C)]
    #[allow(non_camel_case_types)]
    pub struct libselect_fd {
        pub handle: u64,
        pub read: bool,
        pub write: bool,
        pub exception: bool
    }

    extern "C" {
        // int libselect_select(struct libselect_fd* fds, size_t fds_len, uint64_t timeout_ms)
        pub fn libselect_select(fds: *mut libselect_fd, fds_len: usize, timeout_ms: u64) -> c_int;
        // int set_blocking(uint64_t fd, bool blocking);
        pub fn set_blocking(fd: u64, blocking: bool) -> c_int;
    }
}


/// An event matrix defining the possible file descriptor events
/// 
/// # Note
/// This struct is memory compatible to `ffi::libselect_fd`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct Event<'a> {
    /// The file handle to operate on
    handle: InOutHandle<'a>,
    /// A flag for the `read`-event
    read: bool,
    /// A flag for the `write`-event
    write: bool,
    /// A flag for an exceptional event
    exception: bool
}
impl<'a> Event<'a> {
    /// Creates a new event handle
    pub fn new<T>(handle: &'a T, read: bool, write: bool, exception: bool) -> Result<Self> where T: AsInOutHandle {
        let handle = handle.as_io_handle()?;
        Ok(Self { handle, read, write, exception })
    }

    /// The underlying in-out-handle
    pub const fn handle(&self) -> InOutHandle {
        self.handle
    }
    /// Checks whether the read-event flag is set
    pub const fn has_read(&self) -> bool {
        self.read
    }
    /// Checks whether the write-event flag is set
    pub const fn has_write(&self) -> bool {
        self.write
    }
    /// Checks whether the exception-event flag is set
    pub const fn has_exception(&self) -> bool {
        self.exception
    }
    /// Tests whether one or more of the event flags is set to `true`
    pub const fn has_event(&self) -> bool {
        self.read || self.write || self.exception
    }
}


/// Performs a call to `select`
pub fn select(mut set: Vec<Event>, timeout: Duration) -> Result<Vec<Event>> {
    // Cast the pointer to the memory-compatible `ffi::libselect_fd` and convert the timeout
    let set_ptr = set.as_mut_ptr() as *mut ffi::libselect_fd;
    let timeout = u64::try_from(timeout.as_millis())
        .map_err(|e| einval!("Invalid timeout: {:?} ({})", timeout, e))?;
    
    // Call select
    let result = unsafe { ffi::libselect_select(set_ptr, set.len(), timeout) };
    if result != 0 {
        let io_error = Error::from_raw_os_error(result);
        return Err(eio!("Call to libselect failed: {} ({})", io_error, result));
    }
    Ok(set)
}


/// Sets the blocking mode for a given handle
pub fn set_blocking<T>(handle: T, blocking: bool) -> Result where T: AsInOutHandle {
    // Get the in-out-handle
    let handle = handle.as_io_handle()?;
    
    // Set the blocking mode
    let result = unsafe { ffi::set_blocking(handle.raw(), blocking) };
    if result != 0 {
        let io_error = Error::from_raw_os_error(result);
        return Err(eio!("Call to libselect failed: {} ({})", io_error, result));
    }
    Ok(())
}
