//! Linux no_libc synchronization primitives.
//!
//! ## Higher-level synchronization objects
//!
//! Sync-linux-no-libc currently reimplements the following
//! [`std::sync`](https://doc.rust-lang.org/std/sync/index.html)
//! synchronization objects:
//!
//! - [`Barrier`]: Ensures multiple threads will wait for each other
//!   to reach a point in the program, before continuing execution all
//!   together.
//!
//! - [`Condvar`]: Condition Variable, providing the ability to block
//!   a thread while waiting for an event to occur.
//!
//! - [`Mutex`]: Mutual Exclusion mechanism, which ensures that at
//!   most one thread at a time is able to access some data. Unlike the
//!   [std equivalent](https://doc.rust-lang.org/std/sync/struct.Mutex.html),
//!   it does not have a poison mechanism.
//!
//! [`Barrier`]: Barrier
//! [`Condvar`]: Condvar
//! [`Mutex`]: Mutex

mod mutex;
mod barrier;
mod condvar;

pub use barrier::Barrier;
pub use barrier::BarrierWaitResult;
pub use condvar::Condvar;
pub use condvar::WaitTimeoutResult;
pub use mutex::Mutex;
pub use mutex::MutexGuard;
pub use mutex::TryLockError;
pub use mutex::TryLockResult;
