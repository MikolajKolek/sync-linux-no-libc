use core::num::niche_types::Nanoseconds;
use core::time::Duration;
use nc::timespec_t;

const NSEC_PER_SEC: u64 = 1_000_000_000;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct Timespec {
    tv_sec: i64,
    tv_nsec: Nanoseconds,
}

impl Timespec {
    const unsafe fn new_unchecked(tv_sec: i64, tv_nsec: i64) -> Timespec {
        Timespec { tv_sec, tv_nsec: unsafe { Nanoseconds::new_unchecked(tv_nsec as u32) } }
    }

    const fn new(tv_sec: i64, tv_nsec: i64) -> Timespec {
        if tv_nsec >= 0 && tv_nsec < NSEC_PER_SEC as i64 {
            unsafe { Self::new_unchecked(tv_sec, tv_nsec) }
        } else {
            panic!("invalid timestamp");
        }
    }

    pub fn now(clock: nc::clockid_t) -> Timespec {
        use core::mem::MaybeUninit;

        let mut t: MaybeUninit<timespec_t> = MaybeUninit::uninit();
        unsafe { nc::clock_gettime(clock, &mut *t.as_mut_ptr()) }.unwrap();
        let t = unsafe { t.assume_init() };
        Timespec::new(t.tv_sec as i64, t.tv_nsec as i64)
    }

    pub fn checked_add_duration(&self, other: &Duration) -> Option<Timespec> {
        let mut secs = self.tv_sec.checked_add_unsigned(other.as_secs())?;

        // Nano calculations can't overflow because nanos are <1B which fit
        // in a u32.
        let mut nsec = other.subsec_nanos() + self.tv_nsec.as_inner();
        if nsec >= NSEC_PER_SEC as u32 {
            nsec -= NSEC_PER_SEC as u32;
            secs = secs.checked_add(1)?;
        }
        Some(unsafe { Timespec::new_unchecked(secs, nsec.into()) })
    }

    pub fn to_timespec(&self) -> Option<timespec_t> {
        Some(timespec_t {
            tv_sec: self.tv_sec.try_into().ok()?,
            tv_nsec: self.tv_nsec.as_inner().try_into().ok()?,
        })
    }
}
