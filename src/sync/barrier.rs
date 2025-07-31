use core::fmt;
use crate::sync::{Condvar, Mutex};

/// A barrier enables multiple threads to synchronize the beginning
/// of some computation.
///
/// # Examples
///
/// ```
/// use sync_linux_no_libc::sync::Barrier;
/// use std::thread;
///
/// let n = 10;
/// let barrier = Barrier::new(n);
/// thread::scope(|s| {
///     for _ in 0..n {
///         // The same messages will be printed together.
///         // You will NOT see any interleaving.
///         s.spawn(|| {
///             println!("before wait");
///             barrier.wait();
///             println!("after wait");
///         });
///     }
/// });
/// ```
pub struct Barrier {
    lock: Mutex<BarrierState>,
    cvar: Condvar,
    num_threads: usize,
}

// The inner state of a double barrier
struct BarrierState {
    count: usize,
    generation_id: usize,
}

/// A `BarrierWaitResult` is returned by [`Barrier::wait()`] when all threads
/// in the [`Barrier`] have rendezvoused.
///
/// # Examples
///
/// ```
/// use sync_linux_no_libc::sync::Barrier;
///
/// let barrier = Barrier::new(1);
/// let barrier_wait_result = barrier.wait();
/// ```
pub struct BarrierWaitResult(bool);

impl fmt::Debug for Barrier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Barrier").finish_non_exhaustive()
    }
}

impl Barrier {
    /// Creates a new barrier that can block a given number of threads.
    ///
    /// A barrier will block `n`-1 threads which call [`wait()`] and then wake
    /// up all threads at once when the `n`th thread calls [`wait()`].
    ///
    /// [`wait()`]: Barrier::wait
    ///
    /// # Examples
    ///
    /// ```
    /// use sync_linux_no_libc::sync::Barrier;
    ///
    /// let barrier = Barrier::new(10);
    /// ```
    #[must_use]
    #[inline]
    pub const fn new(n: usize) -> Barrier {
        Barrier {
            lock: Mutex::new(BarrierState { count: 0, generation_id: 0 }),
            cvar: Condvar::new(),
            num_threads: n,
        }
    }

    /// Blocks the current thread until all threads have rendezvoused here.
    ///
    /// Barriers are re-usable after all threads have rendezvoused once, and can
    /// be used continuously.
    ///
    /// A single (arbitrary) thread will receive a [`BarrierWaitResult`] that
    /// returns `true` from [`BarrierWaitResult::is_leader()`] when returning
    /// from this function, and all other threads will receive a result that
    /// will return `false` from [`BarrierWaitResult::is_leader()`].
    ///
    /// # Examples
    ///
    /// ```
    /// use sync_linux_no_libc::sync::Barrier;
    /// use std::thread;
    ///
    /// let n = 10;
    /// let barrier = Barrier::new(n);
    /// thread::scope(|s| {
    ///     for _ in 0..n {
    ///         // The same messages will be printed together.
    ///         // You will NOT see any interleaving.
    ///         s.spawn(|| {
    ///             println!("before wait");
    ///             barrier.wait();
    ///             println!("after wait");
    ///         });
    ///     }
    /// });
    /// ```
    pub fn wait(&self) -> BarrierWaitResult {
        let mut lock = self.lock.lock();
        let local_gen = lock.generation_id;
        lock.count += 1;
        if lock.count < self.num_threads {
            let _guard =
                self.cvar.wait_while(lock, |state| local_gen == state.generation_id);
            BarrierWaitResult(false)
        } else {
            lock.count = 0;
            lock.generation_id = lock.generation_id.wrapping_add(1);
            self.cvar.notify_all();
            BarrierWaitResult(true)
        }
    }
}

impl fmt::Debug for BarrierWaitResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BarrierWaitResult").field("is_leader", &self.is_leader()).finish()
    }
}

impl BarrierWaitResult {
    /// Returns `true` if this thread is the "leader thread" for the call to
    /// [`Barrier::wait()`].
    ///
    /// Only one thread will have `true` returned from their result, all other
    /// threads will have `false` returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use sync_linux_no_libc::sync::Barrier;
    ///
    /// let barrier = Barrier::new(1);
    /// let barrier_wait_result = barrier.wait();
    /// println!("{:?}", barrier_wait_result.is_leader());
    /// ```
    #[must_use]
    pub fn is_leader(&self) -> bool {
        self.0
    }
}
