//! [![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
//! [![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
//! [![AppVeyor CI](https://ci.appveyor.com/api/projects/status/github/KizzyCode/libselect-rust?svg=true)](https://ci.appveyor.com/project/KizzyCode/libselect-rust)
//! [![docs.rs](https://docs.rs/libselect/badge.svg)](https://docs.rs/libselect)
//! [![crates.io](https://img.shields.io/crates/v/libselect.svg)](https://crates.io/crates/libselect)
//! [![Download numbers](https://img.shields.io/crates/d/libselect.svg)](https://crates.io/crates/libselect)
//! [![dependency status](https://deps.rs/crate/libselect/0.2.0/status.svg)](https://deps.rs/crate/libselect/0.2.0)
//! 
//! 
//! # `libselect`
//! Welcome to `libselect` 🎉
//! 
//! This crate provides a high-level APIs to perform `select`-operations on I/O-handles.
//! 
//! `libselect` uses Rust's [`AsRawFd`](https://doc.rust-lang.org/stable/std/os/unix/io/trait.AsRawFd.html) /
//! [`AsRawSocket`](https://doc.rust-lang.org/stable/std/os/windows/io/trait.AsRawSocket.html) on windows and works with all
//! I/O-handles that expose raw file descriptors.

#[macro_use] pub mod error;
pub mod select;
mod io;
mod easy;

// Re-export the public API
pub use crate::{
    easy::{ select_read, select_write, select_readwrite },
    io::{ InOutHandle, AsInOutHandle }
};
