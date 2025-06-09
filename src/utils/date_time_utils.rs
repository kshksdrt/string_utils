use chrono::{DateTime, Local};

pub(crate) fn format_duration(seconds: u32) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let remaining_seconds = seconds % 60;

    let mut parts = Vec::new();

    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if remaining_seconds > 0 {
        parts.push(format!("{}s", remaining_seconds));
    }

    if parts.is_empty() {
        "0s".to_string()
    } else {
        parts.join(" ")
    }
}

pub(crate) fn format_duration_with_timestamp(seconds: u32) -> String {
    let duration_str = format_duration(seconds);
    let now: DateTime<Local> = Local::now();
    let future_time = now + chrono::Duration::seconds(seconds as i64);
    let iso_timestamp = future_time.format("%Y-%m-%dT%H:%M:%S").to_string();
    let human_timestamp = future_time.format("%H:%M:%S %d/%m/%Y").to_string();

    format!("{} ({} - {})", duration_str, human_timestamp, iso_timestamp)
}
