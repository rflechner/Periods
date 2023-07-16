mod periods;
mod timeline;

use chrono::{NaiveDate, Months, DateTime, NaiveTime};
use periods::{Period, PeriodValue};
use timeline::{Timeline};

fn main() {

    // let p = Period::one_day_from_str("2023-07-14", None);
    // println!("Created period is {}.", p);

    // let big_period_1 = Period::from_strings("2023-07-14", "2023-09-20", None).unwrap();
    // println!("days in period {} :", p);

    // for day in big_period_1.get_all_days() {
    //     println!("- {}", day);
    // }

    // let periods_collection = periods_vec![
    //     Period::from_strings("2023-07-14", "2023-09-20", None).unwrap(),
    //     Period::from_strings("2023-07-20", "2023-09-24", None).unwrap(),
    //     Period::from_strings("2023-07-24", "2023-09-26", None).unwrap()
    // ];

    let merged_periods = periods::merge_contiguous_periods(vec![
        Period::from_strings("2023-07-14", "2023-07-20", None).unwrap(),
        Period::from_strings("2023-07-20", "2023-07-24", None).unwrap(),
        Period::from_strings("2023-07-24", "2023-07-26", None).unwrap(),
        Period::from_strings("2023-07-28", "2023-08-10", None).unwrap(),
    ].into_iter());
    
    println!("merged periods:");
    for p in merged_periods {
        println!("- {}", p);
    }

    
    let chip1_price_variation = vec![
        one_day_value!("2023-07-14", 23),
        one_day_value!("2023-07-15", 25),
        period_days_value!("2023-07-16", "2023-07-20", 25),
        period_days_value!("2023-07-21", "2023-07-28", 29),
        period_days_value!("2023-07-28", "2023-08-12", 30),
        period_days_value!("2023-08-12", "2023-08-31", 30),
        period_days_value!("2023-09-10", "2023-09-30", 30),
    ];

    println!("merged chip1_price_variation:");
    for p in timeline::merge_contiguous_periods_values(chip1_price_variation.into_iter()) {
        println!("- {}", p);
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
