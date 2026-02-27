use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use no_std_strings::{str12, str32};
use ui::payload::BlockPayload;

use crate::{network::data::Weather, time::Time};

pub enum Network {
    Tick,
    Time { time: Time },
    Weather { weather: Weather },
    Timetable { timetable: [str32; 2] },
}

pub static NETWORK_CHANNEL: Channel<CriticalSectionRawMutex, Network, 3> = Channel::new();

pub enum Visual {
    Time { time: str12 },
    Weather { blocks: [BlockPayload; 4] },
    Timetable { block: BlockPayload },
}

pub static VISUAL_CHANNEL: Channel<CriticalSectionRawMutex, Visual, 3> = Channel::new();
