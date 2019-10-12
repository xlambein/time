/// An offset from UTC.
///
/// Guaranteed to store values up to ±24 hours. Any values outside this range
/// may have incidental support that can change at any time without notice. If
/// you need support outside this range, please file an issue with your use
/// case.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeZone {
    /// The number of seconds offset from UTC. Positive is east, negative is
    /// west.
    seconds: i32,
}

impl TimeZone {
    /// A `TimeZone` that is UTC.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::UTC, TimeZone::seconds(0));
    /// ```
    pub const UTC: Self = Self::seconds(0);

    /// Create a `TimeZone` representing an easterly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::east_hours(1).as_hours(), 1);
    /// assert_eq!(TimeZone::east_hours(2).as_minutes(), 120);
    /// ```
    pub const fn east_hours(hours: u8) -> Self {
        Self::hours(hours as i8)
    }

    /// Create a `TimeZone` representing a westerly offset by the number of
    /// hours provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::west_hours(1).as_hours(), -1);
    /// assert_eq!(TimeZone::west_hours(2).as_minutes(), -120);
    /// ```
    pub const fn west_hours(hours: u8) -> Self {
        Self::hours(-(hours as i8))
    }

    /// Create a `TimeZone` representing an offset by the number of hours
    /// provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::hours(1).as_hours(), 1);
    /// assert_eq!(TimeZone::hours(-1).as_hours(), -1);
    /// assert_eq!(TimeZone::hours(2).as_minutes(), 120);
    /// assert_eq!(TimeZone::hours(-2).as_minutes(), -120);
    /// ```
    pub const fn hours(hours: i8) -> Self {
        Self::seconds(hours as i32 * 3_600)
    }

    /// Create a `TimeZone` representing an easterly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::east_minutes(60).as_hours(), 1);
    /// assert_eq!(TimeZone::east_minutes(30).as_minutes(), 30);
    /// ```
    pub const fn east_minutes(minutes: u16) -> Self {
        Self::minutes(minutes as i16)
    }

    /// Create a `TimeZone` representing a westerly offset by the number of
    /// minutes provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::west_minutes(60).as_hours(), -1);
    /// assert_eq!(TimeZone::west_minutes(30).as_minutes(), -30);
    /// ```
    pub const fn west_minutes(minutes: u16) -> Self {
        Self::minutes(-(minutes as i16))
    }

    /// Create a `TimeZone` representing a offset by the number of minutes
    /// provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::minutes(60).as_hours(), 1);
    /// assert_eq!(TimeZone::minutes(-60).as_hours(), -1);
    /// assert_eq!(TimeZone::minutes(30).as_minutes(), 30);
    /// assert_eq!(TimeZone::minutes(-30).as_minutes(), -30);
    /// ```
    pub const fn minutes(minutes: i16) -> Self {
        Self::seconds(minutes as i32 * 60)
    }

    /// Create a `TimeZone` representing an easterly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::east_seconds(3_600).as_hours(), 1);
    /// assert_eq!(TimeZone::east_seconds(1_800).as_minutes(), 30);
    /// ```
    pub const fn east_seconds(seconds: u32) -> Self {
        Self::seconds(seconds as i32)
    }

    /// Create a `TimeZone` representing a westerly offset by the number of
    /// seconds provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::west_seconds(3_600).as_hours(), -1);
    /// assert_eq!(TimeZone::west_seconds(1_800).as_minutes(), -30);
    /// ```
    pub const fn west_seconds(seconds: u32) -> Self {
        Self::seconds(-(seconds as i32))
    }

    /// Create a `TimeZone` representing an offset by the number of seconds
    /// provided.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::seconds(3_600).as_hours(), 1);
    /// assert_eq!(TimeZone::seconds(-3_600).as_hours(), -1);
    /// assert_eq!(TimeZone::seconds(1_800).as_minutes(), 30);
    /// assert_eq!(TimeZone::seconds(-1_800).as_minutes(), -30);
    /// ```
    pub const fn seconds(seconds: i32) -> Self {
        Self { seconds }
    }

    /// Get the number of seconds from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::UTC.as_seconds(), 0);
    /// assert_eq!(TimeZone::hours(12).as_seconds(), 43_200);
    /// assert_eq!(TimeZone::hours(-12).as_seconds(), -43_200);
    /// assert_eq!(TimeZone::hours(24).as_seconds(), 86_400);
    /// assert_eq!(TimeZone::hours(-24).as_seconds(), -86_400);
    /// ```
    pub const fn as_seconds(self) -> i32 {
        self.seconds
    }

    /// Get the number of minutes from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::UTC.as_minutes(), 0);
    /// assert_eq!(TimeZone::hours(12).as_minutes(), 720);
    /// assert_eq!(TimeZone::hours(-12).as_minutes(), -720);
    /// assert_eq!(TimeZone::hours(24).as_minutes(), 1_440);
    /// assert_eq!(TimeZone::hours(-24).as_minutes(), -1_440);
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_minutes(self) -> i16 {
        (self.as_seconds() / 60) as i16
    }

    /// Get the number of hours from UTC the value is. Positive is east,
    /// negative is west.
    ///
    /// ```rust
    /// # use time::TimeZone;
    /// assert_eq!(TimeZone::UTC.as_hours(), 0);
    /// assert_eq!(TimeZone::hours(12).as_hours(), 12);
    /// assert_eq!(TimeZone::hours(-12).as_hours(), -12);
    /// assert_eq!(TimeZone::hours(24).as_hours(), 24);
    /// assert_eq!(TimeZone::hours(-24).as_hours(), -24);
    /// ```
    #[allow(clippy::cast_possible_truncation)]
    pub const fn as_hours(self) -> i8 {
        (self.as_seconds() / 3_600) as i8
    }
}
