
use crate::periods::{self, Period, PeriodValue};

pub struct Timeline<I> {
  values: Box<dyn Iterator<Item = PeriodValue<I>>>,
}

impl<I> Timeline<I> {
  pub fn new(values: impl Iterator<Item = PeriodValue<I>> + 'static) -> Self {
      Timeline {
          values: Box::new(values),
      }
  }
}

impl<I> Iterator for Timeline<I> {
  type Item = PeriodValue<I>;

  fn next(&mut self) -> Option<Self::Item> {
      self.values.next()
  }
}

pub fn merge_contiguous_periods_values<T>(
  periods: impl Iterator<Item = PeriodValue<T>>,
) -> impl Iterator<Item = PeriodValue<T>>
where
  T: std::cmp::PartialEq + Clone,
{
  let mut iter = periods.peekable();

  std::iter::from_fn(move || {
      if let Some(current) = iter.next() {
          let mut merged_period = current.period.clone();
          let mut merged_value = current.value.clone();

          while let Some(next) = iter.peek() {
              if merged_period.end == next.period.start && merged_value == next.value {
                  merged_period.end = next.period.end;
                  iter.next();
              } else if merged_period.end > next.period.start {
                  merged_period.end = next.period.start;
                  break;
              } else {
                  break;
              }
          }

          Some(PeriodValue {
              period: merged_period,
              value: merged_value,
          })
      } else {
          None
      }
  })
}
