
mod periods;

use periods::Period;

fn main() {

    let p = Period::one_day_from_str("2023-07-14", None);

    println!("Created period is {}.", p);
}
