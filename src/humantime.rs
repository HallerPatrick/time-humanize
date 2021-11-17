use std::borrow::Cow;
use std::cmp::max;
use std::fmt;
use std::ops::{Add, Sub};
use std::time::{Duration, SystemTime};

use std::convert::TryInto;

#[cfg(feature = "time")]
use time::OffsetDateTime;

/// Indicates the time of the period in relation to the time of the utterance
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub enum Tense {
    Past,
    Present,
    Future,
}

/// The accuracy of the representation
#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd)]
pub enum Accuracy {
    /// Rough approximation, easy to grasp, but not necessarily accurate
    Rough,
    /// Concise expression, accurate, but not necessarily easy to grasp
    Precise,
}

impl Accuracy {
    /// Returns whether this accuracy is precise
    #[must_use]
    pub fn is_precise(self) -> bool {
        self == Self::Precise
    }

    /// Returns whether this accuracy is rough
    #[must_use]
    pub fn is_rough(self) -> bool {
        self == Self::Rough
    }
}

// Number of seconds in various time periods
const S_MINUTE: u64 = 60;
const S_HOUR: u64 = S_MINUTE * 60;
const S_DAY: u64 = S_HOUR * 24;
const S_WEEK: u64 = S_DAY * 7;
const S_MONTH: u64 = S_DAY * 30;
const S_YEAR: u64 = S_DAY * 365;

#[derive(Clone, Copy, Debug)]
enum TimePeriod {
    Now,
    Nanos(u64),
    Micros(u64),
    Millis(u64),
    Seconds(u64),
    Minutes(u64),
    Hours(u64),
    Days(u64),
    Weeks(u64),
    Months(u64),
    Years(u64),
    Eternity,
}

impl TimePeriod {
    fn to_text_precise(self) -> Cow<'static, str> {
        match self {
            Self::Now => "now".into(),
            Self::Nanos(n) => format!("{} ns", n).into(),
            Self::Micros(n) => format!("{} µs", n).into(),
            Self::Millis(n) => format!("{} ms", n).into(),
            Self::Seconds(1) => "1 second".into(),
            Self::Seconds(n) => format!("{} seconds", n).into(),
            Self::Minutes(1) => "1 minute".into(),
            Self::Minutes(n) => format!("{} minutes", n).into(),
            Self::Hours(1) => "1 hour".into(),
            Self::Hours(n) => format!("{} hours", n).into(),
            Self::Days(1) => "1 day".into(),
            Self::Days(n) => format!("{} days", n).into(),
            Self::Weeks(1) => "1 week".into(),
            Self::Weeks(n) => format!("{} weeks", n).into(),
            Self::Months(1) => "1 month".into(),
            Self::Months(n) => format!("{} months", n).into(),
            Self::Years(1) => "1 year".into(),
            Self::Years(n) => format!("{} years", n).into(),
            Self::Eternity => "eternity".into(),
        }
    }

    fn to_text_rough(self) -> Cow<'static, str> {
        match self {
            Self::Now => "now".into(),
            Self::Nanos(n) => format!("{} ns", n).into(),
            Self::Micros(n) => format!("{} µs", n).into(),
            Self::Millis(n) => format!("{} ms", n).into(),
            Self::Seconds(n) => format!("{} seconds", n).into(),
            Self::Minutes(1) => "a minute".into(),
            Self::Minutes(n) => format!("{} minutes", n).into(),
            Self::Hours(1) => "an hour".into(),
            Self::Hours(n) => format!("{} hours", n).into(),
            Self::Days(1) => "a day".into(),
            Self::Days(n) => format!("{} days", n).into(),
            Self::Weeks(1) => "a week".into(),
            Self::Weeks(n) => format!("{} weeks", n).into(),
            Self::Months(1) => "a month".into(),
            Self::Months(n) => format!("{} months", n).into(),
            Self::Years(1) => "a year".into(),
            Self::Years(n) => format!("{} years", n).into(),
            Self::Eternity => "eternity".into(),
        }
    }

    fn to_text(self, accuracy: Accuracy) -> Cow<'static, str> {
        match accuracy {
            Accuracy::Rough => self.to_text_rough(),
            Accuracy::Precise => self.to_text_precise(),
        }
    }
}

/// `Duration` wrapper that helps expressing the duration in human languages
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct HumanTime {
    duration: Duration,
    is_positive: bool,
}

impl HumanTime {
    const DAYS_IN_MONTH: u64 = 30;

    /// Create `HumanTime` object that corresponds to the current point in time.
    ///. Similar to `chrono::Utc::now()`
    pub fn now() -> Self {
        Self {
            duration: Duration::new(0, 0),
            is_positive: true,
        }
    }

    /// Gives English text representation of the `HumanTime` with given `accuracy` and 'tense`
    #[must_use]
    pub fn to_text_en(self, accuracy: Accuracy, tense: Tense) -> String {
        let mut periods = match accuracy {
            Accuracy::Rough => self.rough_period(),
            Accuracy::Precise => self.precise_period(),
        };

        let first = periods.remove(0).to_text(accuracy);
        let last = periods.pop().map(|last| last.to_text(accuracy));

        let mut text = periods.into_iter().fold(first, |acc, p| {
            format!("{}, {}", acc, p.to_text(accuracy)).into()
        });

        if let Some(last) = last {
            text = format!("{} and {}", text, last).into();
        }

        match tense {
            Tense::Past => format!("{} ago", text),
            Tense::Future => format!("in {}", text),
            Tense::Present => text.into_owned(),
        }
    }

    /// Return `HumanTime` for given seconds from epoch start
    pub fn from_duration_since_timestamp(timestamp: u64) -> HumanTime {
        let since_epoch_duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let ts = Duration::from_secs(timestamp);

        let duration = since_epoch_duration - ts;

        // Can something happen when casting from unsigned to signed?
        let duration = duration.as_secs() as i64;

        // Cause we calculate since a timestamp till today, we negate the duration
        HumanTime::from(-duration)
    }

    /// Returns the unix timestamp till Duration
    pub fn to_unix_timestamp(&self) -> i64 {
        let since_epoch_duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();

        let duration = if self.is_positive {
            since_epoch_duration + self.duration
        } else {
            since_epoch_duration - self.duration
        };

        duration.as_secs() as i64
    }

    fn tense(self, accuracy: Accuracy) -> Tense {
        match self.duration.as_secs() {
            0..=10 if accuracy.is_rough() => Tense::Present,
            _ if !self.is_positive => Tense::Past,
            _ if self.is_positive => Tense::Future,
            _ => Tense::Present,
        }
    }

    fn rough_period(self) -> Vec<TimePeriod> {
        let period = match self.duration.as_secs() {
            n if n > 547 * S_DAY => TimePeriod::Years(max(n / S_YEAR, 2)),
            n if n > 345 * S_DAY => TimePeriod::Years(1),
            n if n > 45 * S_DAY => TimePeriod::Months(max(n / S_MONTH, 2)),
            n if n > 29 * S_DAY => TimePeriod::Months(1),
            n if n > 10 * S_DAY + 12 * S_HOUR => TimePeriod::Weeks(max(n / S_WEEK, 2)),
            n if n > 6 * S_DAY + 12 * S_HOUR => TimePeriod::Weeks(1),
            n if n > 36 * S_HOUR => TimePeriod::Days(max(n / S_DAY, 2)),
            n if n > 22 * S_HOUR => TimePeriod::Days(1),
            n if n > 90 * S_MINUTE => TimePeriod::Hours(max(n / S_HOUR, 2)),
            n if n > 45 * S_MINUTE => TimePeriod::Hours(1),
            n if n > 90 => TimePeriod::Minutes(max(n / S_MINUTE, 2)),
            n if n > 45 => TimePeriod::Minutes(1),
            n if n > 10 => TimePeriod::Seconds(n),
            0..=10 => TimePeriod::Now,
            _ => TimePeriod::Eternity,
        };

        vec![period]
    }

    fn precise_period(self) -> Vec<TimePeriod> {
        let mut periods = vec![];

        let (years, reminder) = self.split_years();
        if let Some(years) = years {
            periods.push(TimePeriod::Years(years));
        }

        let (months, reminder) = reminder.split_months();
        if let Some(months) = months {
            periods.push(TimePeriod::Months(months));
        }

        let (weeks, reminder) = reminder.split_weeks();
        if let Some(weeks) = weeks {
            periods.push(TimePeriod::Weeks(weeks));
        }

        let (days, reminder) = reminder.split_days();
        if let Some(days) = days {
            periods.push(TimePeriod::Days(days));
        }

        let (hours, reminder) = reminder.split_hours();
        if let Some(hours) = hours {
            periods.push(TimePeriod::Hours(hours));
        }

        let (minutes, reminder) = reminder.split_minutes();
        if let Some(minutes) = minutes {
            periods.push(TimePeriod::Minutes(minutes));
        }

        let (seconds, reminder) = reminder.split_seconds();
        if let Some(seconds) = seconds {
            periods.push(TimePeriod::Seconds(seconds));
        }

        let (millis, reminder) = reminder.split_milliseconds();
        if let Some(millis) = millis {
            periods.push(TimePeriod::Millis(millis));
        }

        let (micros, reminder) = reminder.split_microseconds();
        if let Some(micros) = micros {
            periods.push(TimePeriod::Micros(micros));
        }

        let (nanos, reminder) = reminder.split_nanoseconds();
        if let Some(nanos) = nanos {
            periods.push(TimePeriod::Nanos(nanos));
        }

        debug_assert!(reminder.is_zero());

        if periods.is_empty() {
            periods.push(TimePeriod::Seconds(0));
        }

        periods
    }

    /// Split this `HumanTime` into number of whole years and the reminder
    fn split_years(self) -> (Option<u64>, Self) {
        let years = self.duration.as_secs() / S_YEAR;
        let reminder = self.duration - Duration::new(years * S_YEAR, 0);
        Self::normalize_split(years, reminder)
    }

    /// Split this `HumanTime` into number of whole months and the reminder
    fn split_months(self) -> (Option<u64>, Self) {
        let months = self.duration.as_secs() / S_MONTH;
        let reminder = self.duration - Duration::new(months * Self::DAYS_IN_MONTH, 0);
        Self::normalize_split(months, reminder)
    }

    /// Split this `HumanTime` into number of whole weeks and the reminder
    fn split_weeks(self) -> (Option<u64>, Self) {
        let weeks = self.duration.as_secs() / S_WEEK;
        let reminder = self.duration - Duration::new(weeks * S_WEEK, 0);
        Self::normalize_split(weeks, reminder)
    }

    /// Split this `HumanTime` into number of whole days and the reminder
    fn split_days(self) -> (Option<u64>, Self) {
        let days = self.duration.as_secs() / S_DAY;
        let reminder = self.duration - Duration::new(days * S_DAY, 0);
        Self::normalize_split(days, reminder)
    }

    /// Split this `HumanTime` into number of whole hours and the reminder
    fn split_hours(self) -> (Option<u64>, Self) {
        let hours = self.duration.as_secs() / S_HOUR;
        let reminder = self.duration - Duration::new(hours * S_HOUR, 0);
        Self::normalize_split(hours, reminder)
    }

    /// Split this `HumanTime` into number of whole minutes and the reminder
    fn split_minutes(self) -> (Option<u64>, Self) {
        let minutes = self.duration.as_secs() / S_MINUTE;
        let reminder = self.duration - Duration::new(minutes * S_MINUTE, 0);
        Self::normalize_split(minutes, reminder)
    }

    /// Split this `HumanTime` into number of whole seconds and the reminder
    fn split_seconds(self) -> (Option<u64>, Self) {
        let seconds = self.duration.as_secs();
        let reminder = self.duration - Duration::new(seconds, 0);
        Self::normalize_split(seconds, reminder)
    }

    /// Split this `HumanTime` into number of whole milliseconds and the reminder
    fn split_milliseconds(self) -> (Option<u64>, Self) {
        let millis = self.duration.as_millis();
        // We can safely convert u128 to u64, because we got it from the same value
        let reminder = self.duration - Duration::from_millis(millis.try_into().unwrap());
        Self::normalize_split(millis.try_into().unwrap(), reminder)
    }

    /// Split this `HumanTime` into number of whole seconds and the reminder
    fn split_microseconds(self) -> (Option<u64>, Self) {
        let micros = self.duration.as_micros();
        let reminder = self.duration - Duration::from_micros(micros.try_into().unwrap());
        Self::normalize_split(micros.try_into().unwrap(), reminder)
    }

    /// Split this `HumanTime` into number of whole seconds and the reminder
    fn split_nanoseconds(self) -> (Option<u64>, Self) {
        let nanos = self.duration.as_nanos();
        let reminder = self.duration - Duration::from_nanos(nanos.try_into().unwrap());
        Self::normalize_split(nanos.try_into().unwrap(), reminder)
    }

    fn normalize_split(wholes: u64, reminder: Duration) -> (Option<u64>, Self) {
        let whole = match wholes == 0 {
            true => None,
            false => Some(wholes),
        };

        (
            whole,
            Self {
                duration: reminder,
                is_positive: true,
            },
        )
    }

    /// Check if `HumanTime` duration is zero
    pub fn is_zero(self) -> bool {
        self.duration.is_zero()
    }

    /// Return a string represenation for a given `Accuracy`
    fn locale_en(&self, accuracy: Accuracy) -> String {
        let tense = self.tense(accuracy);
        self.to_text_en(accuracy, tense)
    }

    /// Return duration as seconds, can be negative
    fn as_secs(&self) -> i64 {
        if self.is_positive {
            self.duration.as_secs() as i64
        } else {
            -(self.duration.as_secs() as i64)
        }
    }
}

/// Instantiate `HumanTime` from different time metrics
impl HumanTime {
    /// Instantiate `HumanTime` for given seconds
    pub fn from_seconds(seconds: i64) -> HumanTime {
        HumanTime::from(seconds)
    }

    /// Instantiate `HumanTime` for given minutes
    pub fn from_minutes(minutes: i64) -> HumanTime {
        HumanTime::from(minutes * S_MINUTE as i64)
    }

    /// Instantiate `HumanTime` for given hours
    pub fn from_hours(hours: i64) -> HumanTime {
        HumanTime::from(hours * S_HOUR as i64)
    }

    /// Instantiate `HumanTime` for given days
    pub fn from_days(days: i64) -> HumanTime {
        HumanTime::from(days * S_DAY as i64)
    }

    /// Instantiate `HumanTime` for given weeks
    pub fn from_weeks(weeks: i64) -> HumanTime {
        HumanTime::from(weeks * S_WEEK as i64)
    }

    /// Instantiate `HumanTime` for given months
    pub fn from_months(months: i64) -> HumanTime {
        HumanTime::from(months * S_MONTH as i64)
    }

    /// Instantiate `HumanTime` for given years
    pub fn from_years(years: i64) -> HumanTime {
        HumanTime::from(years * S_YEAR as i64)
    }
}

impl fmt::Display for HumanTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let accuracy = if f.alternate() {
            Accuracy::Precise
        } else {
            Accuracy::Rough
        };

        f.pad(&self.locale_en(accuracy))
    }
}

impl From<Duration> for HumanTime {
    /// Create `HumanTime` from `Duration`
    fn from(duration: Duration) -> Self {
        Self {
            duration,
            is_positive: true,
        }
    }
}

impl Add for HumanTime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        HumanTime::from(self.as_secs() + rhs.as_secs())
    }
}

impl Sub for HumanTime {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        HumanTime::from(self.as_secs() - rhs.as_secs())
    }
}

impl From<SystemTime> for HumanTime {
    fn from(st: SystemTime) -> Self {
        match st.duration_since(SystemTime::now()) {
            Ok(duration) => HumanTime::from(-(duration.as_secs() as i64)),
            Err(err) => HumanTime::from(-(err.duration().as_secs() as i64)),
        }
    }
}

impl From<i64> for HumanTime {
    /// Performs conversion from `i64` to `HumanTime`, from seconds.
    fn from(duration_in_sec: i64) -> Self {
        Self {
            duration: Duration::from_secs(duration_in_sec.unsigned_abs()),
            is_positive: duration_in_sec >= 0,
        }
    }
}

#[cfg(feature = "time")]
impl Into<OffsetDateTime> for HumanTime {
    fn into(self) -> OffsetDateTime {
        if self.is_positive {
            OffsetDateTime::UNIX_EPOCH + self.duration
        } else {
            OffsetDateTime::UNIX_EPOCH - self.duration
        }
    }
}

/// Display `Duration` as human readable time
pub trait Humanize {
    fn humanize(&self) -> String;
}

impl Humanize for Duration {
    fn humanize(&self) -> String {
        format!("{}", HumanTime::from(*self))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use std::time::SystemTime;

    #[cfg(feature = "time")]
    #[test]
    fn test_into_offset_date_time() {
        let dt: OffsetDateTime = HumanTime::from(SystemTime::UNIX_EPOCH).into();
        let ht = HumanTime::from(SystemTime::now());

        // Left is actual unix timestamp, right is duration till timestamp therefore negate
        assert_eq!(dt.unix_timestamp(), -ht.to_unix_timestamp())
    }

    #[test]
    fn test_duration_from_system_time() {
        let ht = HumanTime::from(SystemTime::now());
        assert_eq!("now", format!("{}", ht))
    }

    #[test]
    fn test_duration_from_system_time_since_epoch() {
        let ht = HumanTime::from(SystemTime::UNIX_EPOCH);
        // Well this will work for one year
        assert_eq!("51 years ago", format!("{}", ht))
    }

    #[test]
    fn test_add_human_time() {
        let ht1 = HumanTime::from_seconds(30);
        let ht2 = HumanTime::from_seconds(30);

        let result = ht1 + ht2;
        assert_eq!(result.duration.as_secs(), 60);
        assert!(result.is_positive);
    }

    #[test]
    fn test_add_human_time_neg() {
        let ht1 = HumanTime::from_seconds(30);
        let ht2 = HumanTime::from_seconds(-40);

        let result = ht1 + ht2;
        assert_eq!(result.duration.as_secs(), 10);
        assert!(!result.is_positive);
    }

    #[test]
    fn test_sub_human_time() {
        let ht1 = HumanTime::from_seconds(30);
        let ht2 = HumanTime::from_seconds(30);

        let result = ht1 - ht2;
        assert_eq!(result.duration.as_secs(), 0);
        assert!(result.is_positive);
    }

    #[test]
    fn test_sub_human_time_neg() {
        let ht1 = HumanTime::from_seconds(30);
        let ht2 = HumanTime::from_seconds(-40);

        let result = ht1 + ht2;
        assert_eq!(result.duration.as_secs(), 10);
        assert!(!result.is_positive);
    }
}
