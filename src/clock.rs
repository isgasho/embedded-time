//! Clock abstractions

use crate::{
    duration::Duration, instant::Instant, period::Period, time_int::TimeInt, timer::param,
    timer::Timer,
};

/// Potential `Clock` errors
#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum Error<E: crate::Error> {
    /// implementation-specific error
    Other(E),
}

/// The `Clock` trait provides an abstraction of hardware-specific timer peripherals, external timer
/// devices, RTCs, etc.
///
/// The `Clock` is characterized by an inner unsigned integer storage type (either [`u32`] or
/// [`u64`]), a [`u32`]/[`u32`] fraction ([`Period`]) defining the duration (in seconds) of one
/// count of this `Clock`, and a custom error type (must implement [`crate::Error`]) representing
/// errors that may be generated by the implementation.
///
/// In addition to the [`Clock::now()`] method which returns an [`Instant`], an unlimited number of
/// software [`Timer`]s can be spawned from a single `Clock` instance.
pub trait Clock: Sized {
    /// The type to hold the tick count
    type Rep: TimeInt;

    /// The duration of one clock tick in seconds, AKA the clock precision.
    const PERIOD: Period;

    /// Implementation-specific error type
    ///
    /// This type can be returned using the [`Error::Other(E)`](enum.Error.html#variant.Other)
    type ImplError: crate::Error;

    /// Get the current Instant
    ///
    /// # Errors
    /// Implementation-specific error returned as
    /// [`Error::Other(Self::ImplError)`](enum.Error.html#variant.Other)
    fn now(&self) -> Result<Instant<Self>, Error<Self::ImplError>>;

    /// Spawn a new, `OneShot` [`Timer`] from this clock
    fn new_timer<Dur: Duration>(
        &self,
        duration: Dur,
    ) -> Timer<param::OneShot, param::Armed, Self, Dur> {
        Timer::<param::None, param::None, Self, Dur>::new(&self, duration)
    }
}
