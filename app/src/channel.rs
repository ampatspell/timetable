use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use ui::payload::{Payload, TimeData};

pub enum Messages {
    Ping {},
    Time { time: TimeData },
}

pub static CHANNEL: Channel<CriticalSectionRawMutex, Messages, 10> = Channel::new();
