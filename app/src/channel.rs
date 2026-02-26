use chrono::{DateTime, FixedOffset};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use no_std_strings::str8;

pub enum NetworkMessages {
    Tick,
    Time { date_time: DateTime<FixedOffset> },
}

pub static NETWORK_CHANNEL: Channel<CriticalSectionRawMutex, NetworkMessages, 3> = Channel::new();

pub enum UiMessages {
    Time { time: str8 },
}

pub static UI_CHANNEL: Channel<CriticalSectionRawMutex, UiMessages, 3> = Channel::new();
