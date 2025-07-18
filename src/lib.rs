/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/monotonic-time-rs
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
pub mod wasm;

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::time::{Duration, Instant};

#[cfg(feature = "metricator-compat")]
pub mod num;

/// Represents a monotonic absolute timestamp with millisecond resolution.
///
/// This struct encapsulates a `u64` value representing the number of milliseconds since a
/// implementation specific epoch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Millis(u64);

impl Millis {
    /// Creates a new `Millis` instance from an absolute time in milliseconds.
    ///
    /// # Arguments
    ///
    /// * `absolute_time` - The absolute time in milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// let timestamp = Millis::new(1_614_834_000);
    /// ```
    #[inline]
    pub fn new(absolute_time: u64) -> Self {
        Self(absolute_time)
    }

    /// Returns the underlying milliseconds value.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// let timestamp = Millis::new(1_614_834_000);
    /// assert_eq!(timestamp.absolute_milliseconds(), 1_614_834_000);
    /// ```
    #[inline]
    pub fn absolute_milliseconds(&self) -> u64 {
        self.0
    }

    /// Extracts the lower 16 bits from the timestamp.
    ///
    /// This is useful for efficient serialization scenarios where only a subset of the timestamp
    /// is needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// let timestamp = Millis::new(0x12345678);
    /// let lower_bits = timestamp.to_lower();
    /// assert_eq!(lower_bits, 0x5678);
    /// ```
    pub const fn to_lower(&self) -> MillisLow16 {
        (self.0 & 0xffff) as u16
    }

    /// Reconstructs the full monotonic timestamp from the current time and lower bits.
    ///
    /// If the lower bits indicate a wrap-around, adjusts the timestamp accordingly.
    ///
    /// # Arguments
    ///
    /// * `lower_bits` - The lower 16 bits of a previously recorded timestamp.
    ///
    /// # Returns
    ///
    /// * `Some(Millis)` - The reconstructed monotonic timestamp if the difference is within 3000 milliseconds.
    /// * `None` - If the difference between `now` and the reconstructed time exceeds 3000 milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// let current = Millis::new(0x00010000);
    /// let lower = current.to_lower();
    /// let reconstructed = current.from_lower(lower).unwrap();
    /// assert_eq!(reconstructed, current);
    /// ```
    pub fn from_lower(&self, lower_bits: MillisLow16) -> Option<Millis> {
        let now_bits = (self.0 & 0xffff) as u16;
        let received_lower_bits = lower_bits;
        let top: u64 = self.0 & 0xffffffffffff0000;

        let mut received_monotonic = top | (received_lower_bits as u64);

        // Adjust for wrap-around if lower bits have wrapped
        if received_lower_bits > now_bits {
            received_monotonic = received_monotonic.wrapping_sub(0x10000);
        }

        let diff = self.0.wrapping_sub(received_monotonic);

        if diff > 3000 {
            return None;
        }

        Some(Millis::new(received_monotonic))
    }

    /// Calculates the duration since another `Millis`.
    ///
    /// # Arguments
    ///
    /// * `earlier` - The earlier monotonic timestamp.
    ///
    /// # Returns
    ///
    /// A `Duration` representing the elapsed time.
    ///
    /// # Panics
    ///
    /// Panics if `self` is earlier than `earlier`.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// use std::time::Duration;
    /// let start = Millis::new(1000);
    /// let end = Millis::new(5000);
    /// let duration = end.duration_since(start);
    /// assert_eq!(duration, Duration::from_millis(4000));
    /// ```
    pub fn duration_since(&self, earlier: Millis) -> Duration {
        self.checked_duration_since(earlier)
            .expect("Millis::duration_since called with a later timestamp")
    }

    /// Calculates the duration since another `Millis`, returning `None` if `self` is earlier.
    ///
    /// # Arguments
    ///
    /// * `earlier` - The earlier monotonic timestamp.
    ///
    /// # Returns
    ///
    /// * `Some(Duration)` - The elapsed time if `self` is later than or equal to `earlier`.
    /// * `None` - If `self` is earlier than `earlier`.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// use std::time::Duration;
    /// let start = Millis::new(1000);
    /// let end = Millis::new(5000);
    /// assert_eq!(end.checked_duration_since(start), Some(Duration::from_millis(4000)));
    /// ```
    pub fn checked_duration_since(&self, earlier: Millis) -> Option<Duration> {
        if self.0 >= earlier.0 {
            Some(Duration::from_millis(self.0 - earlier.0))
        } else {
            None
        }
    }

    /// Calculates the duration since another `Millis`, returning `None` if `self` is earlier.
    ///
    /// # Arguments
    ///
    /// * `earlier` - The earlier monotonic timestamp.
    ///
    /// # Returns
    ///
    /// * `Some(MillisDuration)` - The elapsed time in milliseconds if `self` is later than or equal to `earlier`.
    /// * `None` - If `self` is earlier than `earlier`.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// let start = Millis::new(1000);
    /// let end = Millis::new(5000);
    /// let duration = end.checked_duration_since_ms(start).unwrap();
    /// assert_eq!(duration.as_millis(), 4000);
    /// ```
    pub fn checked_duration_since_ms(&self, earlier: Millis) -> Option<MillisDuration> {
        if self.0 >= earlier.0 {
            Some(MillisDuration::from_millis(self.0 - earlier.0))
        } else {
            None
        }
    }

    /// Calculates the duration since another `Millis`, panicking if `self` is earlier.
    ///
    /// # Arguments
    ///
    /// * `earlier` - The earlier monotonic timestamp.
    ///
    /// # Returns
    ///
    /// A `MillisDuration` representing the elapsed time.
    ///
    /// # Panics
    ///
    /// Panics if `self` is earlier than `earlier`.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::Millis;
    /// let start = Millis::new(1000);
    /// let end = Millis::new(5000);
    /// let duration = end.duration_since_ms(start);
    /// assert_eq!(duration.as_millis(), 4000);
    /// ```
    pub fn duration_since_ms(&self, earlier: Millis) -> MillisDuration {
        self.checked_duration_since_ms(earlier)
            .expect("Millis::duration_since_ms called with a later timestamp")
    }
}

impl AddAssign<MillisDuration> for Millis {
    fn add_assign(&mut self, other: MillisDuration) {
        self.0 += other.0;
    }
}

impl SubAssign<MillisDuration> for Millis {
    fn sub_assign(&mut self, other: MillisDuration) {
        self.0 -= other.0;
    }
}

impl Add<MillisDuration> for Millis {
    type Output = Self;

    fn add(self, other: MillisDuration) -> Self::Output {
        Self(self.0 + other.0)
    }
}

impl Sub<MillisDuration> for Millis {
    type Output = Self;

    fn sub(self, other: MillisDuration) -> Self::Output {
        Self(self.0 - other.0)
    }
}

/// Represents the lower 16 bits of a timestamp in milliseconds.
///
/// This type alias is used for efficient serialization scenarios where only a subset of the
/// timestamp is needed.
pub type MillisLow16 = u16;

/// Represents a duration in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct MillisDuration(u64);

impl MillisDuration {
    /// Creates a new `MillisDuration` instance from milliseconds.
    ///
    /// # Arguments
    ///
    /// * `millis` - The duration in milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::MillisDuration;
    /// let duration = MillisDuration::from_millis(4000);
    /// ```
    #[inline]
    pub const fn from_millis(millis: u64) -> Self {
        Self(millis)
    }

    /// Creates a new `MillisDuration` from a number of seconds.
    /// Returns an error if the input is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::MillisDuration;
    /// let duration = MillisDuration::from_secs(2.5).unwrap();
    /// assert_eq!(duration.as_millis(), 2500);
    /// ```
    #[inline]
    pub fn from_secs(seconds: f32) -> Result<Self, &'static str> {
        if seconds < 0.0 {
            return Err("must be a positive value");
        }
        Ok(Self((seconds * 1000.0) as u64))
    }

    /// Returns the duration in milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::MillisDuration;
    /// let duration = MillisDuration::from_millis(4000);
    /// assert_eq!(duration.as_millis(), 4000);
    /// ```
    #[inline]
    pub fn as_millis(&self) -> u64 {
        self.0
    }

    pub fn as_secs(&self) -> f32 {
        self.0 as f32 / 1000.0
    }
}

impl fmt::Display for MillisDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ms", self.0)
    }
}

impl From<u64> for MillisDuration {
    #[inline]
    fn from(ms: u64) -> Self {
        MillisDuration::from_millis(ms)
    }
}

impl From<MillisDuration> for u64 {
    #[inline]
    fn from(duration: MillisDuration) -> Self {
        duration.0
    }
}

impl Mul<f32> for MillisDuration {
    type Output = MillisDuration;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_millis(((self.0 as f32) * rhs) as u64)
    }
}

impl Mul<MillisDuration> for f32 {
    type Output = MillisDuration;

    fn mul(self, rhs: MillisDuration) -> Self::Output {
        MillisDuration::from_millis((self * (rhs.0 as f32)) as u64)
    }
}

impl Mul<u32> for MillisDuration {
    type Output = MillisDuration;
    #[inline]
    fn mul(self, rhs: u32) -> Self::Output {
        Self::from_millis(((self.0 as u32) * rhs) as u64)
    }
}

impl Mul<MillisDuration> for u32 {
    type Output = MillisDuration;

    #[inline]
    fn mul(self, rhs: MillisDuration) -> Self::Output {
        MillisDuration::from_millis((self * (rhs.0 as u32)) as u64)
    }
}

impl Add for MillisDuration {
    type Output = MillisDuration;

    #[inline]
    fn add(self, rhs: MillisDuration) -> MillisDuration {
        MillisDuration::from_millis(
            self.0
                .checked_add(rhs.0)
                .expect("overflow on add millisduration"),
        )
    }
}

impl AddAssign for MillisDuration {
    #[inline]
    fn add_assign(&mut self, rhs: MillisDuration) {
        *self = *self + rhs;
    }
}

impl Sub for MillisDuration {
    type Output = MillisDuration;

    #[inline]
    fn sub(self, rhs: MillisDuration) -> MillisDuration {
        Self::from_millis(
            self.0
                .checked_sub(rhs.0)
                .expect("overflow on sub millisduration"),
        )
    }
}

impl SubAssign for MillisDuration {
    #[inline]
    fn sub_assign(&mut self, rhs: MillisDuration) {
        *self = *self - rhs;
    }
}

impl MulAssign<u32> for MillisDuration {
    #[inline]
    fn mul_assign(&mut self, rhs: u32) {
        *self = *self * rhs;
    }
}

impl Div<u32> for MillisDuration {
    type Output = MillisDuration;

    #[inline]
    fn div(self, rhs: u32) -> MillisDuration {
        Self::from_millis(
            self.0
                .checked_div(rhs as u64)
                .expect("divide by zero error millisduration"),
        )
    }
}

impl DivAssign<u32> for MillisDuration {
    #[inline]
    fn div_assign(&mut self, rhs: u32) {
        *self = *self / rhs;
    }
}

impl Div<u64> for MillisDuration {
    type Output = MillisDuration;

    #[inline]
    fn div(self, rhs: u64) -> MillisDuration {
        Self::from_millis(
            self.0
                .checked_div(rhs)
                .expect("divide by zero error millisduration"),
        )
    }
}

impl DivAssign<u64> for MillisDuration {
    #[inline]
    fn div_assign(&mut self, rhs: u64) {
        *self = *self / rhs;
    }
}

impl Div<MillisDuration> for MillisDuration {
    type Output = MillisDuration;

    fn div(self, rhs: MillisDuration) -> Self::Output {
        self / rhs.0
    }
}

impl DivAssign<MillisDuration> for MillisDuration {
    #[inline]
    fn div_assign(&mut self, rhs: MillisDuration) {
        *self = *self / rhs;
    }
}

/// Implements subtraction between two `Millis` instances, returning a `MillisDuration`.
///
/// # Panics
///
/// Panics if the first timestamp (`self`) is less than the second timestamp (`other`).
///
/// # Examples
///
/// ```
/// use monotonic_time_rs::Millis;
/// let start = Millis::new(1000);
/// let end = Millis::new(5000);
/// let duration = end - start;
/// assert_eq!(duration.as_millis(), 4000);
/// ```
impl Sub for Millis {
    type Output = MillisDuration;

    fn sub(self, other: Millis) -> MillisDuration {
        if self.0 >= other.0 {
            MillisDuration::from_millis(self.0 - other.0)
        } else {
            panic!(
                "Attempted to subtract a later Millis from an earlier one: {self:?} - {other:?}"
            );
        }
    }
}

impl From<u64> for Millis {
    #[inline]
    fn from(ms: u64) -> Self {
        Millis::new(ms)
    }
}

impl From<Millis> for u64 {
    #[inline]
    fn from(millis: Millis) -> Self {
        millis.0
    }
}

impl fmt::Display for Millis {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ms", self.0)
    }
}

/// A trait for providing monotonic time measurements.
///
/// Implementors of this trait should provide a method to retrieve the current
/// monotonic time in milliseconds. Monotonic time is guaranteed to be non-decreasing
/// and is not affected by system clock updates.
///
/// # Examples
///
/// ```
/// use monotonic_time_rs::{MonotonicClock, Millis};
/// struct SystemClock;
///
/// impl MonotonicClock for SystemClock {
///     fn now(&self) -> Millis {
///         Millis::new(1_614_834_000)
///     }
/// }
/// ```
pub trait MonotonicClock {
    /// Returns the current monotonic time as a `Millis` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::{MonotonicClock, Millis};
    /// struct SystemClock;
    ///
    /// impl MonotonicClock for SystemClock {
    ///     fn now(&self) -> Millis {
    ///         Millis::new(1_614_834_000)
    ///     }
    /// }
    /// ```
    fn now(&self) -> Millis;
}

/// A concrete implementation of `MonotonicClock` using `std::time::Instant`.
///
/// This struct captures the instant when it was created and provides
/// the elapsed time since then as a `Millis` timestamp.
pub struct InstantMonotonicClock {
    started: Instant,
}

impl InstantMonotonicClock {
    /// Creates a new `InstantMonotonicClock` instance, capturing the current instant.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::InstantMonotonicClock;
    /// let clock = InstantMonotonicClock::new();
    /// ```
    pub fn new() -> Self {
        Self {
            started: Instant::now(),
        }
    }
}

impl Default for InstantMonotonicClock {
    fn default() -> Self {
        Self::new()
    }
}

impl MonotonicClock for InstantMonotonicClock {
    /// Returns the elapsed monotonic time since the creation of the `InstantMonotonicClock`.
    ///
    /// # Examples
    ///
    /// ```
    /// use monotonic_time_rs::{Millis, MonotonicClock, InstantMonotonicClock};
    /// let clock = InstantMonotonicClock::new();
    /// std::thread::sleep(std::time::Duration::from_millis(500));
    /// let current_time = clock.now();
    /// assert!(current_time.absolute_milliseconds() >= 500);
    /// ```
    fn now(&self) -> Millis {
        let duration = Instant::now().duration_since(self.started);
        Millis::new(duration.as_millis() as u64)
    }
}

pub fn create_monotonic_clock() -> impl MonotonicClock {
    #[cfg(target_arch = "wasm32")]
    use crate::wasm::WasmMonotonicClock;
    #[cfg(target_arch = "wasm32")]
    {
        WasmMonotonicClock::new()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        InstantMonotonicClock::new()
    }
}
