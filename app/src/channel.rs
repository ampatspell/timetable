use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use ui::payload::Payload;

pub enum Messages {
    Update { payload: Payload },
    Ping {},
}

pub static CHANNEL: Channel<CriticalSectionRawMutex, Messages, 10> = Channel::new();
