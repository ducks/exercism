extern crate chrono;
use chrono::*;

pub fn after(startdate: DateTime<UTC>) -> DateTime<UTC> {
  startdate + Duration::seconds(1_000_000_000)
}
