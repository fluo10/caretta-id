use caretta_id::CarettaId;
use chrono::{DateTime, Local};
use clap::Args;

use crate::option::{BaseTimeOption, BaseTimeOptionArgs};

#[derive(Args, Debug)]
/// Generate time-based caretta-id
pub struct TimestampArgs {
    #[command(flatten)]
    base: BaseTimeOptionArgs,
    timestamp: Option<DateTime<Local>>
}

impl TimestampArgs {
    pub fn run(self) {
        println!("{}", match (BaseTimeOption::from(self.base), self.timestamp) {
            (BaseTimeOption::Unix, None) => CarettaId::now_unix(),
            (BaseTimeOption::Unix, Some(x)) => CarettaId::from_timestamp_unix(x),
            (BaseTimeOption::Base(x), None) => CarettaId::now_since(x),
            (BaseTimeOption::Base(x), Some(y)) => CarettaId::from_timestamp_since(y, x),
        });
    }
}
