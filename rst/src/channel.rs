use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use no_std_strings::str256;

pub enum Messages {
    Update { text: str256 },
}

pub static CHANNEL: Channel<CriticalSectionRawMutex, Messages, 10> = Channel::new();
