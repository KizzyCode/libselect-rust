//! Implements a lifetime scoped wrapper for raw file descriptors

use crate::error::Result;
use std::{ convert::TryFrom, marker::PhantomData };


/// A lifetime aware in-out handle
/// 
/// # Note
/// This struct is memory compatible to `u64`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct InOutHandle<'a> {
    /// The underling raw file descriptor
    fd: u64,
    /// The associated lifetime
    _lifetime: PhantomData<&'a u64>
}
impl<'a> InOutHandle<'a> {
    /// Creates a new in-out-handle from a raw file descriptor 
    pub const fn with_raw(fd: u64) -> Self {
        Self { fd, _lifetime: PhantomData }
    }

    /// Provides access to the raw file descriptor
    ///
    /// # Safety
    /// Since this function exposes the native, copyable file descriptor it's easy to violate lifetime  guarantees and get
    /// a dangling or invalid file descriptor. While this is usually not as dangerous as e.g. a dangling ponter, it can
    /// lead to confusing or weird behaviour.
    pub const unsafe fn raw(&self) -> u64 {
        self.fd
    }
}
impl<'a> AsInOutHandle for InOutHandle<'a> {
    fn as_io_handle(&self) -> Result<InOutHandle> {
        Ok(*self)
    }
}


/// A trait for I/O-types that have an underlying raw file descriptor
pub trait AsInOutHandle {
    /// Creates a lifetime scoped in-out-handle from `self`
    fn as_io_handle(&self) -> Result<InOutHandle>;
}
#[cfg(target_family = "unix")]
impl<T> AsInOutHandle for T where T: std::os::unix::io::AsRawFd {
    fn as_io_handle(&self) -> Result<InOutHandle> {
        let raw_fd = self.as_raw_fd();
        let fd = u64::try_from(raw_fd).map_err(|e| einval!("Invalid file descriptor: {} ({})", raw_fd, e))?;
        Ok(InOutHandle::with_raw(fd))
    }
}
#[cfg(target_family = "windows")]
impl<T> AsInOutHandle for T where T: std::os::windows::io::AsRawSocket {
    fn as_io_handle(&self) -> Result<InOutHandle> {
        let raw_fd = self.as_raw_socket();
        let fd = u64::try_from(raw_fd).map_err(|e| einval!("Invalid file descriptor: {} ({})", raw_fd, e))?;
        Ok(InOutHandle::with_raw(fd))
    }
}
