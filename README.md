# sync-linux-no-libc
[![Crates.io](https://img.shields.io/crates/l/sync-linux-no-libc)](https://github.com/MikolajKolek/sync-linux-no-libc/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/sync-linux-no-libc)](https://crates.io/crates/sync-linux-no-libc)
[![Crates.io](https://img.shields.io/crates/v/sync-linux-no-libc)](https://crates.io/crates/sync-linux-no-libc)
[![Sync-linux-no-libc documentation](https://docs.rs/sync-linux-no-libc/badge.svg)](https://docs.rs/sync-linux-no-libc)

This project aims to reimplement some of the most basic Rust `std::sync` utilities on Linux, like [`std::sync::Mutex`](https://doc.rust-lang.org/std/sync/struct.Mutex.html) and [`std::sync::Barrier`](https://doc.rust-lang.org/std/sync/struct.Barrier.html), all without the use of libc. Instead, it will use the [`nc`](https://docs.rs/nc/latest/nc/) to make Linux syscalls directly.

# License
Sync-linux-no-libc is licensed under the [MIT Licence](https://github.com/MikolajKolek/sync-linux-no-libc/blob/master/LICENSE) 