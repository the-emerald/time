//! TODO docs

use crate::{Date, Weekday};

/// Returns if the provided year is a leap year in the proleptic Gregorian
/// calendar.
///
/// ```rust
/// # use time::util::is_leap_year;
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2000));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2005));
/// assert!(!is_leap_year(2100));
/// ```
#[inline(always)]
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))
}

/// Get the number of calendar days in a given year.
///
/// The returned value will always be either 365 or 366.
///
/// ```rust
/// # use time::util::days_in_year;
/// assert_eq!(days_in_year(1900), 365);
/// assert_eq!(days_in_year(2000), 366);
/// assert_eq!(days_in_year(2004), 366);
/// assert_eq!(days_in_year(2005), 365);
/// assert_eq!(days_in_year(2100), 365);
/// ```
#[inline(always)]
pub fn days_in_year(year: i32) -> u16 {
    365 + is_leap_year(year) as u16
}

/// Get the number of weeks in the ISO year.
///
/// The returned value will always be either 52 or 53.
///
/// ```rust
/// # use time::util::weeks_in_year;
/// assert_eq!(weeks_in_year(2019), 52);
/// assert_eq!(weeks_in_year(2020), 53);
/// ```
#[inline(always)]
pub fn weeks_in_year(year: i32) -> u8 {
    let weekday = Date::from_yo_unchecked(year, 1).weekday();

    if (weekday == Weekday::Thursday) || (weekday == Weekday::Wednesday && is_leap_year(year)) {
        53
    } else {
        52
    }
}

/// Get the number of days in the month of a given year.
#[inline(always)]
pub(crate) fn days_in_year_month(year: i32, month: u8) -> u8 {
    /// The number of days in a month in both common and leap years.
    const DAYS_IN_MONTH_COMMON_LEAP: [[u16; 12]; 2] = [
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    ];

    DAYS_IN_MONTH_COMMON_LEAP[is_leap_year(year) as usize][month as usize - 1] as u8
}
