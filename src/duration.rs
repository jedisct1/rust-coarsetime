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
    pub fn new(sec: u64, nanos: u32) -> Duration {
        Duration(_timespec_to_u64(sec, nanos))
    }

    /// Creates a new Duration from the specified number of days
    #[inline]
    pub fn from_days(days: u64) -> Duration {
        Duration(_sec_to_u64(days.saturating_mul(86400)))
    }

    /// Creates a new Duration from the specified number of hours
    #[inline]
    pub fn from_hours(hours: u64) -> Duration {
        Duration(_sec_to_u64(hours.saturating_mul(3600)))
    }

    /// Creates a new Duration from the specified number of minutes
    #[inline]
    pub fn from_mins(mins: u64) -> Duration {
        Duration(_sec_to_u64(mins.saturating_mul(60)))
    }

    /// Creates a new Duration from the specified number of seconds
    #[inline]
    pub fn from_secs(secs: u64) -> Duration {
        Duration(_sec_to_u64(secs))
    }

    /// Creates a new Duration from the specified number of milliseconds
    #[inline]
    pub fn from_millis(millis: u64) -> Duration {
        Duration(_millis_to_u64(millis))
    }

    /// Returns the number of days represented by this duration
    #[inline]
    pub fn as_days(&self) -> u64 {
        self.as_secs() / 86400
    }

    /// Returns the number of minutes represented by this duration
    #[inline]
    pub fn as_hours(&self) -> u64 {
        self.as_secs() / 3600
    }

    /// Returns the number of minutes represented by this duration
    #[inline]
    pub fn as_mins(&self) -> u64 {
        self.as_secs() / 60
    }

    /// Returns the number of whole seconds represented by this duration
    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.0 >> 32
    }

    /// Returns the number of whole milliseconds represented by this duration
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // false positive
    pub fn as_millis(&self) -> u64 {
        ((self.0 as u128 * 125) >> 29) as u64
    }

    /// Returns the number of whole microseconds represented by this duration
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // false positive
    pub fn as_micros(&self) -> u64 {
        ((self.0 as u128 * 125_000) >> 29) as u64
    }

    /// Returns the number of whole nanoseconds represented by this duration
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // false positive
    pub fn as_nanos(&self) -> u64 {
        ((self.0 as u128 * 125_000_000) >> 29) as u64
    }

    /// Returns the nanosecond precision represented by this duration
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // false positive
    pub fn subsec_nanos(&self) -> u32 {
        ((self.0 as u32 as u64 * 125_000_000) >> 29) as u32
    }

    /// Return this duration as a number of "ticks".
    ///
    /// Note that length of a 'tick' is not guaranteed to represent
    /// the same amount of time across different platforms, or from
    /// one version of `coarsetime` to another.
    #[inline]
    pub fn as_ticks(&self) -> u64 {
        self.as_u64()
    }

    /// Creates a new Duration from the specified number of "ticks".
    ///
    /// Note that length of a 'tick' is not guaranteed to represent
    /// the same amount of time across different platforms, or from
    /// one version of `coarsetime` to another.
    #[inline]
    pub fn from_ticks(ticks: u64) -> Duration {
        Self::from_u64(ticks)
    }

    #[doc(hidden)]
    #[inline]
    pub fn as_u64(&self) -> u64 {
        self.0
    }

    #[doc(hidden)]
    #[inline]
    pub fn from_u64(ts: u64) -> Duration {
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
    pub fn abs_diff(&self, other: Duration) -> Duration {
        Duration(self.0.abs_diff(other.0))
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
        Duration(self.0.saturating_add(rhs.0))
    }
}

impl AddAssign for Duration {
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // clippy#11220
    fn add_assign(&mut self, rhs: Duration) {
        *self = *self + rhs;
    }
}

impl Sub for Duration {
    type Output = Duration;

    #[inline]
    fn sub(self, rhs: Duration) -> Duration {
        Duration(self.0.saturating_sub(rhs.0))
    }
}

impl SubAssign for Duration {
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // clippy#11220
    fn sub_assign(&mut self, rhs: Duration) {
        *self = *self - rhs;
    }
}

impl Mul<u32> for Duration {
    type Output = Duration;

    #[inline]
    fn mul(self, rhs: u32) -> Duration {
        Duration(self.0.saturating_mul(rhs as u64))
    }
}

impl MulAssign<u32> for Duration {
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // clippy#11220
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for Duration {
    type Output = Duration;

    #[inline]
    fn div(self, rhs: u32) -> Duration {
        // rust-lang/rust#122821
        Duration(self.0.checked_div(rhs as u64).unwrap_or(u64::MAX))
    }
}

impl DivAssign<u32> for Duration {
    #[inline]
    #[allow(clippy::arithmetic_side_effects)] // clippy#11220
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

#[test]
fn no_overflow() {
    let _: u64 = Duration(u64::MAX).as_millis();
    let _: u64 = Duration(u64::MAX).as_micros();
    let _: u64 = Duration(u64::MAX).as_nanos();
    let _: u32 = Duration(u64::MAX).subsec_nanos();
}
