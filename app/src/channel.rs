use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use no_std_strings::str12;

use crate::time::Time;

pub enum Network {
    Tick,
    Time { time: Time },
}

pub static NETWORK_CHANNEL: Channel<CriticalSectionRawMutex, Network, 3> = Channel::new();

pub enum Visual {
    Time { time: str12 },
}

pub static VISUAL_CHANNEL: Channel<CriticalSectionRawMutex, Visual, 3> = Channel::new();
