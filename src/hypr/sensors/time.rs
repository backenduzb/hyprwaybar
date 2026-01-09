use chrono::{Local, Timelike};

pub fn get_time_string() -> String {
    let now = Local::now();

    let mut hour = now.hour();
    let minute = now.minute();

    let day = if hour >= 12 { "PM" } else { "AM" };

    hour = hour % 12;
    if hour == 0 {
        hour = 12;
    }

    format!("{:02}:{:02} {}", hour, minute, day)
}