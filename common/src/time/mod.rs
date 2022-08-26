use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

pub fn get_unix_timestamp() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn unix_timestamp_to_datetime(timestamp: u64) -> DateTime {
    let z = ((timestamp as i64) / 86400) + 719468;
    let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
    let doe = (z - era * 146097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;

    let day = (doy - (153 * mp + 2) / 5 + 1) as u8;
    let month = (if mp < 10 { mp + 3 } else { mp - 9 }) as u8;
    let year = (y + (if month <= 2 { 1 } else { 0 })) as u16;
    let hour = ((timestamp / 3600) % 24) as u8;
    let minute = ((timestamp / 60) % 60) as u8;
    let second = (timestamp % 60) as u8;

    DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
    }
}
