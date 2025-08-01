use core::sync::atomic::Atomic;
use core::time::Duration;
use syscalls::{syscall, Errno, Sysno};

/// An atomic for use as a futex that is at least 32-bits but may be larger
pub type Futex = Atomic<Primitive>;
/// Must be the underlying type of Futex
pub type Primitive = u32;

/// An atomic for use as a futex that is at least 8-bits but may be larger.
pub type SmallFutex = Atomic<SmallPrimitive>;
/// Must be the underlying type of SmallFutex
pub type SmallPrimitive = u32;

/// Waits for a `futex_wake` operation to wake us.
///
/// Returns directly if the futex doesn't hold the expected value.
///
/// Returns false on timeout, and true in all other cases.
pub fn futex_wait(futex: &Atomic<u32>, expected: u32, timeout: Option<Duration>) -> bool {
    use crate::sys::timespec::Timespec;
    use core::ptr::null;
    use core::sync::atomic::Ordering::Relaxed;

    // Calculate the timeout as an absolute timespec.
    //
    // Overflows are rounded up to an infinite timeout (None).
    let timespec = timeout
        .and_then(|d| Timespec::now(nc::CLOCK_MONOTONIC).checked_add_duration(&d))
        .and_then(|t| t.to_timespec());

    loop {
        // No need to wait if the value already changed.
        if futex.load(Relaxed) != expected {
            return true;
        }

        #[cfg(feature = "not_process_private")]
        let op = nc::FUTEX_WAIT_BITSET;
        #[cfg(not(feature = "not_process_private"))]
        let op = nc::FUTEX_WAIT_BITSET | nc::FUTEX_PRIVATE_FLAG;
        let r = unsafe {
            // Use FUTEX_WAIT_BITSET rather than FUTEX_WAIT to be able to give an
            // absolute time rather than a relative time.
            syscall!(
                Sysno::futex,
                futex as *const Atomic<u32>,
                op,
                expected,
                timespec.as_ref().map_or(null(), |t| t as *const nc::timespec_t),
                null::<u32>(), // This argument is unused for FUTEX_WAIT_BITSET.
                !0u32          // A full bitmask, to make it behave like a regular FUTEX_WAIT.
            )
        };

        match r {
            Err(Errno::ETIMEDOUT) => return false,
            Err(Errno::EINTR) => continue,
            _ => return true,
        }
    }
}

/// Wakes up one thread that's blocked on `futex_wait` on this futex.
///
/// Returns true if this actually woke up such a thread,
/// or false if no thread was waiting on this futex.
pub fn futex_wake(futex: &Atomic<u32>) -> bool {
    let ptr = futex as *const Atomic<u32>;

    #[cfg(feature = "not_process_private")]
    let op = nc::FUTEX_WAKE;
    #[cfg(not(feature = "not_process_private"))]
    let op = nc::FUTEX_WAKE | nc::FUTEX_PRIVATE_FLAG;
    unsafe {
        syscall!(Sysno::futex, ptr, op, 1)
            .expect("futex_wake failed") > 0
    }
}

/// Wakes up all threads that are waiting on `futex_wait` on this futex.
pub fn futex_wake_all(futex: &Atomic<u32>) {
    let ptr = futex as *const Atomic<u32>;

    #[cfg(feature = "not_process_private")]
    let op = nc::FUTEX_WAKE;
    #[cfg(not(feature = "not_process_private"))]
    let op = nc::FUTEX_WAKE | nc::FUTEX_PRIVATE_FLAG;
    unsafe {
        syscall!(Sysno::futex, ptr, op, i32::MAX)
            .expect("futex_wake_all failed");
    }
}
