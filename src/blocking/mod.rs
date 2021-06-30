//! Blocking API
//!
//! In some cases it's possible to implement these blocking traits on top of one of the core HAL
//! traits. To save boilerplate when that's the case a `Default` marker trait may be provided.
//! Implementing that marker trait will opt in your type into a blanket implementation.

pub mod delay;
pub mod digital;
pub mod i2c;
pub mod pwm;
pub mod qei;
pub mod serial;
pub mod spi;
pub mod watchdog;
