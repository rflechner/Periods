
mod periods;

use std::ops::Add;

use chrono::{NaiveDate, Months, DateTime, NaiveTime};
use periods::Period;


fn main() {

    let p = Period::one_day_from_str("2023-07-14", None);
    println!("Created period is {}.", p);

    let big_period_1 = Period::from_strings("2023-07-14", "2023-09-20", None).unwrap();
    println!("days in period {} :", p);

    for day in big_period_1.get_all_days() {
        println!("- {}", day);
    }

    // let one_month = Months::new(1);
    // let start = NaiveDate::from_ymd_opt(2023, 1, 1).unwrap();
    // let end1 = start.add(one_month);
    // let end2 = end1.add(one_month);

    // println!("______________________");
    // println!("{} - {} - {}", start, end1, end2);

    // let stop = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap().and_time(NaiveTime::MIN);

    // let suite = (1..).map(|i| {
    //     let next = start.add(Months::new(i)).and_time(NaiveTime::MIN);
    //     let previous = start.add(Months::new(i-1)).and_time(NaiveTime::MIN);
    //     Period::new(previous, next).unwrap()
    // });
    // //.take_while(|month| month.start < stop);

    // for month in suite.take(100) {
    //     println!("- {}", month);
    // }

}
