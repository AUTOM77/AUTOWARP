use chrono::{DateTime, FixedOffset, Local, SecondsFormat};

pub fn get_tos() -> String {
    let local_time: DateTime<Local> = Local::now();
    let offset = FixedOffset::west_opt(2 * 3600).expect("Invalid offset");
    let date_time = local_time.with_timezone(&offset);
    let formatted = date_time.to_rfc3339_opts(SecondsFormat::Millis, true);

    formatted
}

