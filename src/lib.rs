//! # PTY
//!
//! [![Crate][crate-badge]][crate] [![docs-badge][]][docs] [![license-badge][]][license] [![travis-badge][]][travis]
//!
//! [crate-badge]: https://img.shields.io/badge/crates.io-v0.2.0-orange.svg?style=flat-square
//! [crate]: https://crates.io/crates/pty
//!
//! [docs-badge]: https://img.shields.io/badge/API-docs-blue.svg?style=flat-square
//! [docs]: http://note.hibariya.org/pty-rs/pty/index.html
//!
//! [license-badge]: https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square
//! [license]: https://github.com/hibariya/pty-rs/blob/master/LICENSE.txt
//!
//! [travis-badge]: https://travis-ci.org/hibariya/pty-rs.svg?branch=master&style=flat-square
//! [travis]: https://travis-ci.org/hibariya/pty-rs
//!
//! The `pty` crate provides `pty::fork()`. That makes a parent process fork with new pseudo-terminal (PTY).
//!
//! This crate depends on followings:
//!
//! * `libc` library
//! * POSIX environment
//!
//! ## Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! pty = "0.2"
//! ```
//!
//! and this to your crate root:
//!
//! ```rust
//! extern crate shpool_pty;
//! ```
//!
//! ### pty::fork()
//!
//! This function returns `pty::Child`. It represents the child process and its PTY.
//!
//! For example, the following code spawns `tty(1)` command by `pty::fork()` and outputs the result of the command.
//!
//! ```rust
//! extern crate shpool_pty;
//! extern crate libc;
//!
//! use std::ffi::CString;
//! use std::io::Read;
//! use std::process::{Command};
//!
//! use shpool_pty::fork::*;
//!
//! fn main() {
//!   let fork = Fork::from_ptmx().unwrap();
//!
//!   if let Some(mut master) = fork.is_parent().ok() {
//!     // Read output via PTY master
//!     let mut output = String::new();
//!
//!     match master.read_to_string(&mut output) {
//!       Ok(_nread) => println!("child tty is: {}", output.trim()),
//!       Err(e)     => panic!("read error: {}", e),
//!     }
//!   }
//!   else {
//!     // Child process just exec `tty`
//!     Command::new("tty").status().expect("could not execute tty");
//!   }
//! }
//! ```

extern crate errno;
extern crate libc;
extern crate log;

mod descriptor;
pub mod fork;
pub mod prelude;

const DEFAULT_PTMX: &str = "/dev/ptmx";
