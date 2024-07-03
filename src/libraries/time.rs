use chrono::{prelude::{DateTime, Utc}, FixedOffset, Local};

pub fn to_str_localtz(time: &DateTime<Utc>) -> String {
    time.with_timezone(&FixedOffset::east_opt(get_timezone())
        .expect("Problem coverting time zone"))
        .format("%b %e, %a | %I:%M%p").to_string()
}

pub fn to_utc(race_date: &str, race_time: &str) -> DateTime<Utc> {
    let time: String = format!("{}T{}", race_date, race_time);
    time.parse::<DateTime<Utc>>().expect("Problem converting time")
}

pub fn utc_now() -> DateTime<Utc> {
    Utc::now()
}

fn get_timezone() -> i32{
    Local::now().offset().local_minus_utc()
}
