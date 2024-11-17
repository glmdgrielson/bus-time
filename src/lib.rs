//! A library for measuring time in the style of Desert Bus's shift clocks.

use std::fmt;

/// The offset associated with Zeta Shift.
pub const ZETA_SHIFT: usize = 0;
/// The offset associated with Dawn Guard.
pub const DAWN_GUARD: usize = 6;
/// The offset associated with Alpha Flight.
pub const ALPHA_FLIGHT: usize = 12;
/// The offset associated with Night Watch.
pub const NIGHT_WATCH: usize = 18;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The shift associated with a BusTime.
pub enum Shift {
	ZetaShift,
	DawnGuard,
	AlphaFlight,
	NightWatch,
}

impl fmt::Display for Shift {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::ZetaShift => f.write_str("Zeta Shift"),
			Shift::DawnGuard => f.write_str("Dawn Guard"),
			Shift::AlphaFlight => f.write_str("Alpha Flight"),
			Shift::NightWatch => f.write_str("Night Watch"),
		}
	}
}

/// A time, per the bus clock.
///
/// This currently doesn't represent nanoseconds, so conversion to other
/// time formats may be a bit lossy. That could probably be fixed though.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct BusTime {
	/// The hour currently represented.
	hour: u32,
	/// The minute currently represented.
	minute: u8,
	/// The second currently represented.
	second: u8,
}

impl BusTime {
	/// Construct a BusTime from a given timestamp.
	///
	/// This converts hours into a form less than 24, so that direct pastes of bus time
	/// will convert properly. It will however
	/// return `None` if given invalid minutes or seconds, because it is assumed
	/// that such action is a mistake.
	pub fn from_hms(hours: u32, minutes: u8, seconds: u8) -> Option<BusTime> {
		if minutes >= 60 || seconds >= 60 {
			None
		} else {
			Some(BusTime {
				hour: hours % 24,
				minute: minutes,
				second: seconds,
			})
		}
	}

	/// Return the shift assoctiated with this BusTime.
	pub fn shift(&self) -> Shift {
		match self.hour {
			_ if self.hour < 6 => Shift::ZetaShift,
			_ if self.hour < 12 => Shift::DawnGuard,
			_ if self.hour < 18 => Shift::AlphaFlight,
			_ if self.hour < 24 => Shift::NightWatch,
			_ => unreachable!("Invalid hour"),
		}
	}

	/// Get the number of hours into the current shift.
	fn hours_into_shift(&self) -> u32 {
		self.hour % 6
	}
}

impl fmt::Display for BusTime {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let shift = match self.shift() {
			Shift::ZetaShift => "ZS",
			Shift::DawnGuard => "DG",
			Shift::AlphaFlight => "AF",
			Shift::NightWatch => "NW",
		};
		let hours = self.hours_into_shift();

		write!(f, "{} + {}:{}:{}", shift, hours, self.minute, self.second)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_from_hms() {
		let time = BusTime::from_hms(21, 0, 0);

		assert_eq!(
			time,
			Some(BusTime {
				hour: 21,
				minute: 0,
				second: 0
			})
		)
	}

	#[test]
	fn test_rounding_time() {
		let week = BusTime::from_hms(168, 0, 0);

		assert_eq!(
			week,
			Some(BusTime {
				hour: 0,
				minute: 0,
				second: 0
			})
		);
	}

	#[test]
	fn test_shift() {
		let zeta = BusTime {
			hour: 3,
			minute: 0,
			second: 0,
		};
		let dawn = BusTime {
			hour: 9,
			minute: 0,
			second: 0,
		};
		let alpha = BusTime {
			hour: 15,
			minute: 0,
			second: 0,
		};
		let night = BusTime {
			hour: 21,
			minute: 0,
			second: 0,
		};

		assert_eq!(zeta.shift(), Shift::ZetaShift);
		assert_eq!(dawn.shift(), Shift::DawnGuard);
		assert_eq!(alpha.shift(), Shift::AlphaFlight);
		assert_eq!(night.shift(), Shift::NightWatch);
	}
}
