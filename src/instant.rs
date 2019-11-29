use super::duration::*;
#[allow(unused_imports)]
use super::helpers::*;
use libc;
use std::cell::RefCell;
#[allow(unused_imports)]
use std::mem::MaybeUninit;
use std::ops::*;
#[allow(unused_imports)]
use std::ptr::*;
use std::sync::atomic::{AtomicPtr, Ordering};

/// A measurement of a monotonically increasing clock. Opaque and useful only with `Duration`.
#[derive(Copy, Clone, Debug, Hash, Ord, Eq, PartialOrd, PartialEq)]
pub struct Instant(u64);

lazy_static! {
    static ref RECENT: AtomicPtr<u64> = AtomicPtr::new(null_mut());
}

thread_local! {
    static LOCAL_RECENT: RefCell<ThreadRecent> = RefCell::new(ThreadRecent::new());
}

struct ThreadRecent {
    recent: u64,
}

impl ThreadRecent {
    pub fn new() -> ThreadRecent {
        ThreadRecent { recent: 0 }
    }

    pub fn update(&mut self, now: u64) {
        self.recent = now;
        RECENT.store(&mut self.recent as *mut u64, Ordering::Relaxed);
    }
}

impl Drop for ThreadRecent {
    fn drop(&mut self) {
        RECENT.compare_and_swap(&mut self.recent as *mut u64, null_mut(), Ordering::Relaxed);
    }
}

#[cfg(windows)]
extern "system" {
    pub fn GetTickCount() -> libc::c_ulong;
}

#[cfg(any(target_os = "macos", target_os = "freebsd"))]
#[allow(non_camel_case_types)]
type clockid_t = libc::c_int;

#[cfg(target_os = "macos")]
const CLOCK_MONOTONIC_RAW_APPROX: clockid_t = 5;

#[cfg(target_os = "macos")]
extern "system" {
    pub fn clock_gettime_nsec_np(clk_id: clockid_t) -> u64;
}

#[cfg(target_os = "freebsd")]
const CLOCK_MONOTONIC_FAST: clockid_t = 12;

impl Instant {
    /// Returns an instant corresponding to "now"
    ///
    /// This function also updates the stored instant.
    pub fn now() -> Instant {
        let now = Self::_now();
        Self::_update(now);
        Instant(now)
    }

    /// Returns an instant corresponding to the latest update
    pub fn recent() -> Instant {
        match Self::_recent() {
            0 => Instant::now(),
            recent => Instant(recent),
        }
    }

    /// Update the stored instant
    ///
    /// This function should be called frequently, for example in an event loop or using an
    /// `Updater` task.
    pub fn update() {
        let now = Self::_now();
        Self::_update(now);
    }

    /// Returns the amount of time elapsed from another instant to this one
    #[inline]
    pub fn duration_since(&self, earlier: Instant) -> Duration {
        *self - earlier
    }

    /// Returns the amount of time elapsed between the this instant was created and the latest
    /// update
    #[inline]
    pub fn elapsed_since_recent(&self) -> Duration {
        Self::recent() - *self
    }

    /// Returns the amount of time elapsed since this instant was created
    ///
    /// This function also updates the stored instant.
    #[inline]
    pub fn elapsed(&self) -> Duration {
        Self::now() - *self
    }

    #[doc(hidden)]
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    #[cfg(any(target_os = "linux", target_os = "android"))]
    fn _now() -> u64 {
        let mut tp = MaybeUninit::<libc::timespec>::uninit();
        let tp = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC_COARSE, tp.as_mut_ptr());
            tp.assume_init()
        };
        _timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u32)
    }

    #[cfg(target_os = "macos")]
    fn _now() -> u64 {
        let nsec = unsafe { clock_gettime_nsec_np(CLOCK_MONOTONIC_RAW_APPROX) };
        _nsecs_to_u64(nsec)
    }

    #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
    fn _now() -> u64 {
        let mut tp = MaybeUninit::<libc::timespec>::uninit();
        let tp = unsafe {
            libc::clock_gettime(libc::CLOCK_MONOTONIC_FAST, tp.as_mut_ptr());
            tp.assume_init()
        };
        _timespec_to_u64(tp.tv_sec as u64, tp.tv_nsec as u32)
    }

    #[cfg(all(
        unix,
        not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "android",
            target_os = "freebsd",
            target_os = "dragonfly"
        ))
    ))]
    fn _now() -> u64 {
        let mut tv = MaybeUninit::<libc::timeval>::uninit();
        let tv = unsafe {
            libc::gettimeofday(tv.as_mut_ptr(), null_mut());
            tv.assume_init()
        };
        _timeval_to_u64(tv.tv_sec as u64, tv.tv_usec as u32)
    }

    #[cfg(windows)]
    fn _now() -> u64 {
        let tc = unsafe { GetTickCount() } as u64;
        _millis_to_u64(tc)
    }

    #[cfg(not(any(windows, unix)))]
    fn _now() -> u64 {
        panic!("Unsupported target");
    }

    #[inline]
    fn _update(now: u64) {
        LOCAL_RECENT.with(|tr| tr.borrow_mut().update(now));
    }

    #[inline]
    fn _recent() -> u64 {
        let ptr = RECENT.load(Ordering::Relaxed);

        if ptr.is_null() {
            let now = Self::_now();
            Self::_update(now);
            Self::_recent()
        } else {
            unsafe { *ptr }
        }
    }
}

impl Default for Instant {
    fn default() -> Instant {
        Self::now()
    }
}

impl Sub<Instant> for Instant {
    type Output = Duration;

    #[cfg(all(
        unix,
        not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "android",
            target_os = "freebsd",
            target_os = "dragonfly"
        ))
    ))]
    #[inline]
    fn sub(self, other: Instant) -> Duration {
        Duration::from_u64(self.0.saturating_sub(other.0))
    }

    #[cfg(not(all(
        unix,
        not(any(
            target_os = "macos",
            target_os = "linux",
            target_os = "android",
            target_os = "freebsd",
            target_os = "dragonfly"
        ))
    )))]
    #[inline]
    fn sub(self, other: Instant) -> Duration {
        Duration::from_u64(self.0 - other.0)
    }
}

impl Sub<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn sub(self, rhs: Duration) -> Instant {
        Instant(self.0 - rhs.as_u64())
    }
}

impl SubAssign<Duration> for Instant {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    #[inline]
    fn add(self, rhs: Duration) -> Instant {
        Instant(self.0 + rhs.as_u64())
    }
}

impl AddAssign<Duration> for Instant {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}
