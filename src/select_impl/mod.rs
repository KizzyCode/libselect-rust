use crate::{ error::Result, io_handle::InOutHandle };
use std::{ collections::HashMap, convert::TryFrom, time::Duration };


/// Declares the FFI types for the shim
mod ffi {
    use std::os::raw::c_int;

    #[repr(C)]
    #[allow(non_camel_case_types)]
    pub struct libselect_fd {
        /// The file descriptor handle to operate on
        pub handle: u64,
        /// A flag for the `read`-event
        pub read: u8,
        /// A flag for the `write`-event
        pub write: u8,
        /// A flag for an exceptional event
        pub exception: u8
    }

    extern "C" {
        /// Calls `select` with the given `fds`
        pub fn libselect_select(fds: *mut libselect_fd, fds_len: usize, timeout_ms: u64) -> c_int;
        /// Sets the blocking mode for the given FD
        pub fn set_blocking(fd: u64, blocking: u8) -> c_int;
    }
}


/// An event matrix defining the possible file descriptor events
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Events {
    /// A flag for the `read`-event
    pub read: bool,
    /// A flag for the `write`-event
    pub write: bool,
    /// A flag for an exceptional event
    pub exception: bool
}
impl Events {
    /// Tests whether one or more of the event flags is set to `true`
    pub fn has_event(&self) -> bool {
        self.read || self.write || self.exception
    }
}


/// A select set
#[derive(Debug, Clone)]
pub struct SelectSet {
    /// The file descriptors with their corresponding events
    fds: HashMap<u64, Events>
}
impl SelectSet {
    /// Creates a new select set
    pub fn new() -> Self {
        Self { fds: HashMap::new() }
    }

    /// Adds a file descriptor for a given event to select set
    pub fn insert<T>(&mut self, handle: &T, events: Events) -> Result where T: InOutHandle {
        let fd = handle.get_raw_fd()?;
        self.fds.insert(fd, events);
        Ok(())
    }
    /// Removes a file descriptor for a given event to select set
    pub fn remove<T>(&mut self, handle: &T) -> Result where T: InOutHandle {
        let fd = handle.get_raw_fd()?;
        self.fds.remove(&fd);
        Ok(())
    }

    /// Tests whether `self` contains a given `fd` and returns the corresponding events
    pub fn get<T>(&self, handle: &T) -> Result<Option<&Events>> where T: InOutHandle {
        let fd = handle.get_raw_fd()?;
        Ok(self.fds.get(&fd))
    }
    /// Tests whether `self` contains a given `fd` and returns the corresponding events
    pub fn get_mut<T>(&mut self, handle: &T) -> Result<Option<&mut Events>> where T: InOutHandle {
        let fd = handle.get_raw_fd()?;
        Ok(self.fds.get_mut(&fd))
    }
}


/// Performs a call to `select`
pub fn select(set: &mut SelectSet, timeout: Duration) -> Result {
    // Translate the set to the FFI set
    let mut ffi_fds = Vec::new();
    for (fd, events) in set.fds.iter() {
        // Create and push the FFI fd struct
        let ffi_fd = ffi::libselect_fd {
            handle: *fd,
            read: events.read.into(),
            write: events.write.into(),
            exception: events.exception.into()
        };
        ffi_fds.push(ffi_fd);
    }

    // Call select
    let timeout = u64::try_from(timeout.as_millis())
        .map_err(|e| einval!("Invalid timeout: {:?} ({})", timeout, e))?;
    let result = unsafe { 
        ffi::libselect_select(ffi_fds.as_mut_ptr(), ffi_fds.len(), timeout)
    };
    if result != 0 {
        Err(eio!("Call to libselect failed ({})", result))?
    }

    // Translate the FFI set back to the set
    for ffi_fd in ffi_fds {
        let events = set.fds.get_mut(&ffi_fd.handle).expect("Missing file descriptor in select set?!");
        *events = Events {
            read: ffi_fd.read > 0,
            write: ffi_fd.write > 0,
            exception: ffi_fd.exception > 0
        };
    }
    Ok(())
}


/// Sets the blocking mode for a given handle
pub fn set_blocking<T>(handle: T, blocking: bool) -> Result where T: InOutHandle {
    let result = unsafe {
        ffi::set_blocking(handle.get_raw_fd()?, blocking.into())
    };
    if result != 0 {
        Err(eio!("Call to libselect failed ({})", result))?
    }
    Ok(())
}
