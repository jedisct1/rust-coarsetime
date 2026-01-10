use std::convert::From;
use std::ops::*;
use std::time;

use super::helpers::*;

/// A duration type to represent an approximate span of time
#[derive(Copy, Clone, Debug, Hash, Ord, Eq, PartialOrd, PartialEq, Default)]
pub struct Duration(u64);

impl Duration {
    /// Creates a new `Duration` from the specified number of seconds and
    /// additional nanosecond precision
    #[inline]
    pub const fn new(sec: u64, nanos: u32) -> Duration {
        Duration(_timespec_to_u64(sec, nanos))
    }

    /// Creates a new Duration from the specified number of days
    #[inline]
    pub const fn from_days(days: u64) -> Duration {
        Duration(_sec_to_u64(days * 86400))
    }

    /// Creates a new Duration from the specified number of hours
    #[inline]
    pub const fn from_hours(hours: u64) -> Duration {
        Duration(_sec_to_u64(hours * 3600))
    }

    /// Creates a new Duration from the specified number of minutes
    #[inline]
    pub const fn from_mins(mins: u64) -> Duration {
        Duration(_sec_to_u64(mins * 60))
    }

    /// Creates a new Duration from the specified number of seconds
    #[inline]
    pub const fn from_secs(secs: u64) -> Duration {
        Duration(_sec_to_u64(secs))
    }

    /// Creates a new Duration from the specified number of milliseconds
    #[inline]
    pub const fn from_millis(millis: u64) -> Duration {
        Duration(_millis_to_u64(millis))
    }

    /// Returns the number of days represented by this duration
    #[inline]
    pub const fn as_days(&self) -> u64 {
        self.as_secs() / 86400
    }

    /// Returns the number of hours represented by this duration
    #[inline]
    pub const fn as_hours(&self) -> u64 {
        self.as_secs() / 3600
    }

    /// Returns the number of minutes represented by this duration
    #[inline]
    pub const fn as_mins(&self) -> u64 {
        self.as_secs() / 60
    }

    /// Returns the number of whole seconds represented by this duration
    #[inline]
    pub const fn as_secs(&self) -> u64 {
        self.0 >> 32
    }

    /// Returns the number of whole milliseconds represented by this duration
    #[inline]
    pub const fn as_millis(&self) -> u64 {
        ((self.0 as u128 * 125) >> 29) as u64
    }

    /// Returns the number of whole microseconds represented by this duration
    #[inline]
    pub const fn as_micros(&self) -> u64 {
        ((self.0 as u128 * 125_000) >> 29) as u64
    }

    /// Returns the number of whole nanoseconds represented by this duration
    #[inline]
    pub const fn as_nanos(&self) -> u64 {
        ((self.0 as u128 * 125_000_000) >> 29) as u64
    }

    /// Returns the nanosecond precision represented by this duration
    #[inline]
    pub const fn subsec_nanos(&self) -> u32 {
        ((self.0 as u32 as u64 * 125_000_000) >> 29) as u32
    }

    /// Return this duration as a number of "ticks".
    ///
    /// Note that length of a 'tick' is not guaranteed to represent
    /// the same amount of time across different platforms, or from
    /// one version of `coarsetime` to another.
    #[inline]
    pub const fn as_ticks(&self) -> u64 {
        self.as_u64()
    }

    /// Creates a new Duration from the specified number of "ticks".
    ///
    /// Note that length of a 'tick' is not guaranteed to represent
    /// the same amount of time across different platforms, or from
    /// one version of `coarsetime` to another.
    #[inline]
    pub const fn from_ticks(ticks: u64) -> Duration {
        Self::from_u64(ticks)
    }

    #[doc(hidden)]
    #[inline]
    pub const fn as_u64(&self) -> u64 {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    pub const fn from_u64(ts: u64) -> Duration {
        Duration(ts)
    }

    /// Returns the duration as a floating point number, representing the number
    /// of seconds
    #[inline]
    pub fn as_f64(&self) -> f64 {
        (self.0 as f64) / ((1u64 << 32) as f64)
    }

    /// Returns the absolute difference between two `Duration`s
    #[inline]
    pub const fn abs_diff(&self, other: Duration) -> Duration {
        Duration(self.0.abs_diff(other.0))
    }

    /// Add two durations, saturating on overflow
    #[inline]
    pub const fn saturating_add(self, rhs: Duration) -> Duration {
        Duration(self.0.saturating_add(rhs.0))
    }

    /// Add two durations, returning `None` on overflow
    #[inline]
    pub fn checked_add(self, rhs: Duration) -> Option<Duration> {
        self.0.checked_add(rhs.0).map(Duration)
    }

    /// Subtract two durations, saturating on underflow/overflow
    #[inline]
    pub const fn saturating_sub(self, rhs: Duration) -> Duration {
        Duration(self.0.saturating_sub(rhs.0))
    }

    /// Subtract two durations, returning `None` on underflow/overflow
    #[inline]
    pub fn checked_sub(self, rhs: Duration) -> Option<Duration> {
        self.0.checked_sub(rhs.0).map(Duration)
    }

    /// Multiply a duration by a scalar, saturating on overflow
    #[inline]
    pub const fn saturating_mul(self, rhs: u32) -> Duration {
        Duration(self.0.saturating_mul(rhs as u64))
    }

    /// Multiply a duration by a scalar, returning `None` on overflow
    #[inline]
    pub fn checked_mul(self, rhs: u32) -> Option<Duration> {
        self.0.checked_mul(rhs as u64).map(Duration)
    }

    /// Divide a duration by a scalar, returning `None` for division by zero
    #[inline]
    pub fn checked_div(self, rhs: u32) -> Option<Duration> {
        self.0.checked_div(rhs as u64).map(Duration)
    }
}

#[doc(hidden)]
impl From<u64> for Duration {
    #[doc(hidden)]
    #[inline]
    fn from(ts: u64) -> Duration {
        Duration::from_u64(ts)
    }
}

impl Add for Duration {
    type Output = Duration;

    #[inline]
    fn add(self, rhs: Duration) -> Duration {
        Duration(self.0 + rhs.0)
    }
}

impl AddAssign for Duration {
    #[inline]
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Duration) -> Duration {
        Duration(self.0 - rhs.0)
    }
}

impl SubAssign for Duration {
    #[inline]
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;

    #[inline]
    fn mul(self, rhs: u32) -> Duration {
        Duration(self.0 * rhs as u64)
    }
}

impl MulAssign<u32> for Duration {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Duration {
    type Output = Duration;

    #[inline]
    fn div(self, rhs: u32) -> Duration {
        Duration(self.0 / rhs as u64)
    }
}

impl DivAssign<u32> for Duration {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl From<Duration> for time::Duration {
    #[inline]
    fn from(duration: Duration) -> time::Duration {
        time::Duration::new(duration.as_secs(), duration.subsec_nanos())
    }
}

impl From<time::Duration> for Duration {
    #[inline]
    fn from(duration_sys: time::Duration) -> Duration {
        Duration::new(duration_sys.as_secs(), duration_sys.subsec_nanos())
    }
}
