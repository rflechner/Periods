
mod periods;

use periods::Period;

fn main() {

    let p = Period::one_day_from_str("2023-07-14", None);
    println!("Created period is {}.", p);

    let big_period_1 = Period::from_strings("2023-07-14", "2025-04-10", None).unwrap();
    println!("Months in period {} :", p);

    for month in big_period_1.get_all_months() {
        println!("- {} :", month);
    }

}
