use std::{
    ops::SubAssign,
    thread,
    time::{Duration, Instant},
};

use chrono::{NaiveDate, Utc};

pub fn test_stdtime() {
    let dur1 = Duration::from_secs(15);
    println!("{}", dur1.as_millis());

    let dur2 = Duration::from_millis(15500);

    let dur3 = dur1.checked_sub(dur2);

    match dur3 {
        Some(d) => println!("Left time: {}", d.as_millis()),
        None => println!("Left time: None"),
    }

    let now = Instant::now();

    thread::sleep(Duration::from_millis(200));

    println!("{}", now.elapsed().as_millis())
}

pub fn test_chrono() {
    let now_utc = Utc::now();

    println!("Non formated for machines: {}", now_utc.to_string());
    println!(
        "Formated in a readable way: {}",
        now_utc.format("%d/%m/%Y %H:%M")
    );

    let date1 = NaiveDate::from_isoywd_opt(2004, 1, chrono::Weekday::Sun);

    let unwrapped_date = date1.unwrap();

    println!("Day of the year is: {}", unwrapped_date.format("%j"));

    unwrapped_date
        .iter_days()
        .take(4)
        .for_each(|d| println!("{}", d.format("%j")));

    let date2 = NaiveDate::from_yo_opt(2004, 366);

    println!("{}", date2.unwrap().format("%A %B %D"));
}
