use super::helpers::*;
use std::convert::From;
use std::ops::*;
use std::time;

/// A duration type to represent an approximate span of time
#[derive(Copy, Clone, Debug, Hash, Ord, Eq, PartialOrd, PartialEq, Default)]
pub struct Duration(u64);

impl Duration {
    /// Creates a new `Duration` from the specified number of seconds and additional nanosecond
    /// precision
    #[inline]
    pub fn new(sec: u64, nanos: u32) -> Duration {
        Duration(_timespec_to_u64(sec, nanos))
    }

    /// Creates a new Duration from the specified number of seconds
    #[inline]
    pub fn from_secs(sec: u64) -> Duration {
        Duration(_sec_to_u64(sec))
    }

    /// Creates a new Duration from the specified number of milliseconds
    #[inline]
    pub fn from_millis(millis: u64) -> Duration {
        Duration(_millis_to_u64(millis))
    }

    /// Returns the number of whole seconds represented by this duration
    #[inline]
    pub fn as_secs(&self) -> u64 {
        self.0 >> 32
    }

    /// Returns the nanosecond precision represented by this duration
    #[inline]
    pub fn subsec_nanos(&self) -> u32 {
        ((self.0 as u32 as u64 * 125_000_000) >> 29) as u32
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

    /// Returns the duration as a floating point number, representing the number of seconds
    #[inline]
    pub fn as_f64(&self) -> f64 {
        (self.0 as f64) / ((1u64 << 32) as f64)
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

impl Into<time::Duration> for Duration {
    #[inline]
    fn into(self) -> time::Duration {
        time::Duration::new(self.as_secs(), self.subsec_nanos())
    }
}

impl From<time::Duration> for Duration {
    #[inline]
    fn from(duration_sys: time::Duration) -> Duration {
        Duration::new(duration_sys.as_secs(), duration_sys.subsec_nanos())
    }
}
