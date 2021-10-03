use crate::error::Result;
use std::{
    convert::TryFrom,
    ops::{ Deref, DerefMut }
};


/// A trait for I/O-types that have an underlying raw file descriptor
pub trait InOutHandle {
    /// Provides access to the raw file descriptor
    ///
    /// # Warning:
    /// Since this function exposes the native, copyable file descriptor it's easy to violate lifetime and get  a dangling
    /// or invalid file descriptor. While this is not as dangerous as e.g. a dangling ponter, this can lead to confusing or
    /// weird behaviour.
    fn get_raw_fd(&self) -> Result<u64>;
}
#[cfg(target_family = "unix")]
impl<T> InOutHandle for &T where T: std::os::unix::io::AsRawFd {
    fn get_raw_fd(&self) -> Result<u64> {
        let raw_fd = self.as_raw_fd();
        u64::try_from(raw_fd).map_err(|e| einval!("Invalid file descriptor: {} ({})", raw_fd, e))
    }
}
#[cfg(target_family = "windows")]
impl<T> InOutHandle for &T where T: std::os::windows::io::AsRawSocket {
    fn get_raw_fd(&self) -> Result<u64> {
        let raw_fd = self.as_raw_socket();
        u64::try_from(raw_fd).map_err(|e| einval!("Invalid file descriptor: {} ({})", raw_fd, e))
    }
}


/// A wrapper around types that can expose their underlying raw file descriptor. This box can be useful if you have custom
/// types that do not implement `AsRawFd`/`AsRawSocket` but have an underlying raw descriptor nonetheless.
pub struct GenericInOutHandle<T> {
    /// The underlying native file/socket handle
    inner: T,
    /// Gets a raw fd from `inner`
    get_raw_fd: fn(&T) -> Result<u64>
}
impl<T> GenericInOutHandle<T> {
    /// Wraps an I/O-handle
    pub fn new(handle: T, get_raw_fd: fn(&T) -> Result<u64>) -> Self {
        Self { inner: handle, get_raw_fd }
    }

    /// Gets the underlying native file/socket handle
    pub fn into_inner(self) -> T {
        self.inner
    }
}
impl<T> InOutHandle for GenericInOutHandle<T> {
    fn get_raw_fd(&self) -> Result<u64> {
        (self.get_raw_fd)(&self.inner)
    }
}
impl<T> AsRef<T> for GenericInOutHandle<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}
impl<T> AsMut<T> for GenericInOutHandle<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}
impl<T> Deref for GenericInOutHandle<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}
impl<T> DerefMut for GenericInOutHandle<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
