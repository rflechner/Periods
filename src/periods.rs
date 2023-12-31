use std::fmt;
use std::ops::Add;
use std::iter::{Map};
use chrono::{NaiveDate, NaiveTime, NaiveDateTime, Duration, Datelike, Month, Months};

#[derive(Copy, Clone)]
pub struct Period {
  pub start: NaiveDateTime,
  pub end: NaiveDateTime,
}

impl fmt::Display for Period {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{};{}[", self.start, self.end)
  }
}

impl Period {

  pub fn new(start: NaiveDateTime, end: NaiveDateTime) -> Result<Self, String> {
    if end <= start {
        Err("Start is after end.".to_string())
    } else {
        Ok(Self { start, end })
    }
  }

  /// Create a one day length period from a string.
  pub fn one_day_from_str(s: &str, fmt: Option<&str>) -> Self {
    let start = NaiveDate::parse_from_str(s, fmt.unwrap_or("%Y-%m-%d")).unwrap().and_time(NaiveTime::MIN);
    let end = start.add(Duration::days(1));

    Self { start, end }
  }

  /// Create a period from strings.
  pub fn from_strings(start: &str, end: &str, fmt: Option<&str>) -> Result<Self, String> {
    let start = NaiveDate::parse_from_str(start, fmt.unwrap_or("%Y-%m-%d")).unwrap().and_time(NaiveTime::MIN);
    let end = NaiveDate::parse_from_str(end, fmt.unwrap_or("%Y-%m-%d")).unwrap().and_time(NaiveTime::MIN);

    Self::new(start, end)
  }

  /// Compute period duration.
  pub fn duration(&self) -> chrono::Duration {
      self.end.signed_duration_since(self.start)
  }

  /// Split a period from a duration into sub periods
  pub fn split_in_periods(&self, f: fn(NaiveDateTime, i32) -> NaiveDateTime, start: Option<NaiveDateTime>, end: Option<NaiveDateTime>) -> impl Iterator<Item = Period> {
    let start = start.unwrap_or(self.start);
    let end = end.unwrap_or(self.end);
    
    (1..).map(move |i| {
      let next = f(start, i);
      let previous = f(start, i-1);
      Period::new(previous, next).unwrap()
    }).take_while(move |period| period.start < end)
  }

  /// split period to one day duration periods.
  pub fn get_all_days(&self) -> impl Iterator<Item = Period> { 
    self.split_in_periods(|start, i| start + Duration::days(i.into()), None, None)
  }

  /// split period to one hour duration periods.
  pub fn get_all_hours(&self) -> impl Iterator<Item = Period> { 
    self.split_in_periods(|start, i| start + Duration::hours(i.into()), None, None)
  }

  /// split period to one week duration periods.
  pub fn get_all_weeks(&self) -> impl Iterator<Item = Period> { 
    self.split_in_periods(|start, i| start + Duration::weeks(i.into()), None, None)
  }

  /// split period to one month duration periods.
  pub fn get_all_months(&self) -> impl Iterator<Item = Period> {
    let end = 
      if self.end.day() > 1 {
        NaiveDate::from_ymd_opt(self.end.year(), self.end.month(), 1).unwrap().and_time(NaiveTime::MIN).add(Months::new(1))
      } else {
        NaiveDate::from_ymd_opt(self.end.year(), self.end.month(), 1).unwrap().and_time(NaiveTime::MIN)
      };

      let start = NaiveDate::from_ymd_opt(self.start.year(), self.start.month(), 1).unwrap().and_time(NaiveTime::MIN);

      self.split_in_periods(|start, i| start.add(Months::new(i.try_into().unwrap())), Some(start), Some(end))
  }

  /// check if two periods intersect.
  pub fn intersect(&self, other: &Period) -> bool {
    self.start < other.end && self.end > other.start
  }

}


#[test]
fn periods_shoud_not_intersect() {
  let p1 = Period::one_day_from_str("2023-07-14", None);
  let p2 = Period::one_day_from_str("2023-07-15", None);
  let p3 = Period::one_day_from_str("2023-08-15", None);

  assert!(!p1.intersect(&p2));
  assert!(!p1.intersect(&p3));
  assert!(!p2.intersect(&p1));
  assert!(!p3.intersect(&p2));
}

#[test]
fn periods_shoud_intersect() {
  let p1 = Period::from_strings("2023-07-14", "2023-07-20", None).unwrap();
  let p2 = Period::from_strings("2023-07-16", "2023-07-18", None).unwrap();
  let p3 = Period::from_strings("2023-07-10", "2023-07-16", None).unwrap();

  assert!(p1.intersect(&p2));
  assert!(p1.intersect(&p3));
}

#[test]
fn duration_should_be_10() {
    let start = NaiveDateTime::parse_from_str("2023-07-11 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let end = NaiveDateTime::parse_from_str("2023-07-21 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

    let period = Period::new(start, end).expect("Cannot create period");
    let days: Vec<Period> = period.get_all_days().collect();

    assert_eq!(days.len(), 10);
}

#[test]
fn period_should_be_one_day() {
    let period = Period::one_day_from_str("2023-07-14", None);
    let expected_start = NaiveDateTime::parse_from_str("2023-07-14 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    let expected_end = NaiveDateTime::parse_from_str("2023-07-15 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();

    assert_eq!(period.start, expected_start);
    assert_eq!(period.end, expected_end);
}

#[test]
fn test_get_all_days() {
    let start_datetime = NaiveDate::from_ymd_opt(2023, 7, 11).unwrap().and_time(NaiveTime::MIN);
    let end_datetime = NaiveDate::from_ymd_opt(2023, 7, 15).unwrap().and_time(NaiveTime::MIN);

    let period = Period::new(start_datetime, end_datetime).expect("Cannot create period");
    let days: Vec<Period> = period.get_all_days().collect();
    let view: Vec<String> = days.iter().map(|p| p.to_string()).collect();

    assert_eq!(days.len(), 4);

    let expected_dates = vec![
        NaiveDate::from_ymd_opt(2023, 7, 11).unwrap(),
        NaiveDate::from_ymd_opt(2023, 7, 12).unwrap(),
        NaiveDate::from_ymd_opt(2023, 7, 13).unwrap(),
        NaiveDate::from_ymd_opt(2023, 7, 14).unwrap(),
        NaiveDate::from_ymd_opt(2023, 7, 15).unwrap(),
    ];

    for (i, day) in days.iter().enumerate() {
        assert_eq!(day.start.date(), expected_dates[i]);
        assert_eq!(day.end.date(), expected_dates[i] + Duration::days(1));
    }
}

#[test]
fn test_get_all_hours() {
    let period = Period::one_day_from_str("2023-07-14", None);
    let hours: Vec<Period> = period.get_all_hours().collect();

    assert_eq!(hours.len(), 24);

    let expected_start_times = (0..24).map(|hour| NaiveTime::from_hms_opt(hour, 0, 0).unwrap()).collect::<Vec<_>>();

    for (i, hour) in hours.iter().enumerate() {
        assert_eq!(hour.start.time(), expected_start_times[i]);
        let duration = hour.duration();
        assert_eq!(duration, Duration::hours(1));
    }
}

#[test]
fn test_get_all_weeks() {
    let start_datetime = NaiveDate::from_ymd_opt(2023, 7, 1).unwrap().and_time(NaiveTime::MIN);
    let end_datetime = NaiveDate::from_ymd_opt(2023, 9, 01).unwrap().and_time(NaiveTime::MIN);

    let period = Period::new(start_datetime, end_datetime).expect("Cannot create period");
    let weeks: Vec<Period> = period.get_all_weeks().collect();
    let view: Vec<String> = weeks.iter().map(|p| p.to_string()).collect();

    assert_eq!(weeks.len(), 9);

    assert_eq!(weeks[0].start.date(), NaiveDate::from_ymd_opt(2023, 7, 1).unwrap());
    assert_eq!(weeks[0].end.date(), NaiveDate::from_ymd_opt(2023, 7, 8).unwrap());

    assert_eq!(weeks[1].start.date(), NaiveDate::from_ymd_opt(2023, 7, 8).unwrap());
    assert_eq!(weeks[1].end.date(), NaiveDate::from_ymd_opt(2023, 7, 15).unwrap());

    assert_eq!(weeks[2].start.date(), NaiveDate::from_ymd_opt(2023, 7, 15).unwrap());
    assert_eq!(weeks[2].end.date(), NaiveDate::from_ymd_opt(2023, 7, 22).unwrap());

    assert_eq!(weeks[3].start.date(), NaiveDate::from_ymd_opt(2023, 7, 22).unwrap());
    assert_eq!(weeks[3].end.date(), NaiveDate::from_ymd_opt(2023, 7, 29).unwrap());

    assert_eq!(weeks[4].start.date(), NaiveDate::from_ymd_opt(2023, 7, 29).unwrap());
    assert_eq!(weeks[4].end.date(), NaiveDate::from_ymd_opt(2023, 8, 5).unwrap());

    assert_eq!(weeks[5].start.date(), NaiveDate::from_ymd_opt(2023, 8, 5).unwrap());
    assert_eq!(weeks[5].end.date(), NaiveDate::from_ymd_opt(2023, 8, 12).unwrap());

    assert_eq!(weeks[6].start.date(), NaiveDate::from_ymd_opt(2023, 8, 12).unwrap());
    assert_eq!(weeks[6].end.date(), NaiveDate::from_ymd_opt(2023, 8, 19).unwrap());

    assert_eq!(weeks[7].start.date(), NaiveDate::from_ymd_opt(2023, 8, 19).unwrap());
    assert_eq!(weeks[7].end.date(), NaiveDate::from_ymd_opt(2023, 8, 26).unwrap());

    // the last days are contained by this week.
    assert_eq!(weeks[8].start.date(), NaiveDate::from_ymd_opt(2023, 8, 26).unwrap());
    assert_eq!(weeks[8].end.date(), NaiveDate::from_ymd_opt(2023, 9, 2).unwrap());
}

#[test]
fn test_get_all_months() {
    let start_datetime = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap().and_time(NaiveTime::MIN);
    let end_datetime = NaiveDate::from_ymd_opt(2024, 1, 01).unwrap().and_time(NaiveTime::MIN);

    let period = Period::new(start_datetime, end_datetime).expect("Cannot create period");
    let months: Vec<Period> = period.get_all_months().collect();

    let view: Vec<String> = months.iter().map(|p| p.to_string()).collect();

    assert_eq!(months.len(), 12);

    for i in 0..11 {
      let sm = (i+1).try_into().unwrap();
      let em = sm+1;

      assert_eq!(months[i].start.date(), NaiveDate::from_ymd_opt(2023, sm, 1).unwrap());
      assert_eq!(months[i].end.date(), NaiveDate::from_ymd_opt(2023, em, 1).unwrap());
    }

    assert_eq!(months[11].start.date(), NaiveDate::from_ymd_opt(2023, 12, 1).unwrap());
    assert_eq!(months[11].end.date(), NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
}

