use crate::engine::builtin_engines::webgl_engine;

#[derive(Copy, Clone)]
pub struct WebDate {
    stamp: u64,
}

impl Default for WebDate {
    fn default() -> Self {
        Self {
            stamp: webgl_engine::date_now()
        }
    }
}

impl WebDate {
    pub fn current_millisecond(&self) -> u64 {
        milliseconds(self.stamp)
    }

    pub fn current_second(&self) -> u64 {
        seconds(self.stamp)
    }

    pub fn current_minute(&self) -> u64 {
        minutes(self.stamp)
    }

    pub fn current_hour(&self) -> u64 {
        current_hour(self.stamp)
    }

    pub fn current_day_of_month(&self) -> u64 {
        current_day_of_month(self.stamp)
    }

    pub fn current_day_of_week(&self) -> u64 {
        current_day_of_week(self.stamp)
    }

    pub fn current_month(&self) -> u64 {
        current_month(self.stamp)
    }

    pub fn current_year(&self) -> u64 {
        current_year(self.stamp)
    }
}

#[inline]
fn total_millennia(milliseconds: u64) -> u64 {
    milliseconds / 189216000000000
}

#[inline]
fn total_centuries(milliseconds: u64) -> u64 {
    milliseconds / 18921600000000
}

#[inline]
fn total_decades(milliseconds: u64) -> u64 {
    milliseconds / 1892160000000
}

#[inline]
fn total_years(milliseconds: u64) -> u64 {
    milliseconds / 31557807360
}

#[inline]
fn total_months(milliseconds: u64) -> u64 {
    milliseconds / 15778540800
}

#[inline]
fn total_days(milliseconds: u64) -> u64 {
    milliseconds / 86400000
}

#[inline]
fn current_year(milliseconds_since: u64) -> u64 {
    1970 + total_years(milliseconds_since)
}

#[inline]
fn leap_days(milliseconds_since: u64) -> u64 {
    milliseconds_since / 126144000000
}

#[inline]
fn current_hour(milliseconds_since: u64) -> u64 {
    milliseconds_since / 360000 % 24 + 1
}

#[inline]
fn hours(milliseconds: u64) -> u64 {
    milliseconds / 360000 % 24
}

#[inline]
fn minutes(milliseconds: u64) -> u64 {
    milliseconds / 60000 % 60
}

#[inline]
fn seconds(milliseconds: u64) -> u64 {
    milliseconds / 1000 % 60
}

#[inline]
fn milliseconds(milliseconds: u64) -> u64 {
    milliseconds % 1000
}

fn days_before_month(month: u64) -> u64 {
    match month {
        1 => 0,
        2 => 31,
        3 => 60,
        4 => 91,
        5 => 121,
        6 => 152,
        7 => 182,
        8 => 213,
        9 => 244,
        10 => 274,
        11 => 305,
        12 => 335,
        _ => 0
    }
}

fn days_before_month_leap(month: u64) -> u64 {
    match month {
        1 => 0,
        2 => 31,
        3 => 59,
        4 => 90,
        5 => 120,
        6 => 151,
        7 => 181,
        8 => 212,
        9 => 243,
        10 => 273,
        11 => 304,
        12 => 334,
        _ => 0
    }
}

#[inline]
fn days_in_year(milliseconds: u64) -> u64 {
    let milliseconds_of_year = milliseconds % 31557807360;
    total_days(milliseconds_of_year)
}

fn current_month(milliseconds: u64) -> u64 {
    let days_in_year = days_in_year(milliseconds);

    if is_leap_year(current_year(milliseconds) + 70) {
        match days_in_year {
            0..=30 => 1, //31
            31..=59 => 2,        //29
            60..=90 => 3,
            91..=120 => 4,
            121..=151 => 5,
            152..=181 => 6,
            182..=212 => 7,
            213..=243 => 8,
            244..=273 => 9,
            274..=304 => 10,
            305..=334 => 11,
            335..=365 => 12,
            _ => {13}
        }
    } else {
        match days_in_year {
            0..=30 => 1, //31
            31..=58 => 2,        //29
            59..=89 => 3,
            90..=119 => 4,
            120..=150 => 5,
            151..=180 => 6,
            181..=211 => 7,
            212..=242 => 8,
            243..=272 => 9,
            273..=303 => 10,
            304..=333 => 11,
            334..=364 => 12,
            _ => {13}
        }
    }

}

/*The Gregorian calendar consists of the following 12 months:

January – 31 days
February – 28 days in a common year and 29 days in leap years
March – 31 days
April – 30 days
May – 31 days
June – 30 days
July – 31 days
August – 31 days
September – 30 days
October – 31 days
November – 30 days
December – 31 days*/

#[inline]
fn current_day_of_month(milliseconds: u64) -> u64 {
    let days_in_year = days_in_year(milliseconds);

    if is_leap_year(current_year(milliseconds)) {
        days_in_year - days_before_month_leap(current_month(milliseconds)) + 1
    } else {
        days_in_year - days_before_month(current_month(milliseconds)) + 1
    }
}

#[inline]
fn current_day_of_week(milliseconds: u64) -> u64 {
    (total_days(milliseconds) - 5) % 7
}

#[derive(Copy, Clone)]
pub struct TimeInterval {
    since: u64,
}

#[inline]
fn is_leap_year(year: u64) -> bool {
    year % 4 == 0 && year % 100 != 0
}


mod time_test {
    use crate::engine::builtin_engines::webgl_engine::time::{current_day_of_month, current_day_of_week, current_hour, current_month, current_year, hours, is_leap_year, milliseconds, minutes, seconds};

    const TESTING_VAL: u64 = 1637736860927;
    //11/23/21 10:54pm or in other words 22:54pm

    #[test]
    fn test_year() {
        assert_eq!(2021, current_year(TESTING_VAL))
    }

    #[test]
    fn test_minute() {
        assert_eq!(54, minutes(TESTING_VAL))
    }

    #[test]
    fn test_hour() {
        assert_eq!(22, current_hour(TESTING_VAL))
    }

    #[test]
    fn test_second() {
        assert_eq!(20, seconds(TESTING_VAL))
    }

    #[test]
    fn test_millisecond() {
        assert_eq!(927, milliseconds(TESTING_VAL))
    }

    #[test]
    fn test_turn_of_century() {
        assert!(!is_leap_year(2000))
    }

    #[test]
    fn test_leap() {
        assert!(is_leap_year(2004))
    }

    #[test]
    fn test_non_leap() {
        assert!(!is_leap_year(2007))
    }

    #[test]
    fn test_current_month() {
        assert_eq!(11, current_month(TESTING_VAL))
    }

    #[test]
    fn test_current_day() {
        assert_eq!(23, current_day_of_month(TESTING_VAL))
    }

    #[test]
    fn test_day_of_week() {
        assert_eq!(1, current_day_of_week(TESTING_VAL))
    }
}

