/*!
This project aims to reimplement some of the most basic Rust
`std::sync` utilities on Linux, like
[`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) and
[`std::sync::Barrier`](https://doc.rust-lang.org/std/sync/struct.Barrier.html),
all without the use of libc. Instead, it makes Linux syscalls directly.

# Crate features

* **not_process_private** -
  Allows for sharing the synchronization primitives with other processes.
*/

#![cfg(target_os = "linux")]
#![no_std]

#![feature(must_not_suspend)]
#![feature(negative_impls)]
#![feature(generic_atomic)]
#![feature(core_io_borrowed_buf)]
#![feature(temporary_niche_types)]
#![feature(decl_macro)]

pub mod sync;
mod sys;
mod tests;