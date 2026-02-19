use defmt::{Formatter, write};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use no_std_strings::str128;

pub enum Messages {
    Update { payload: Payload },
}

pub static CHANNEL: Channel<CriticalSectionRawMutex, Messages, 10> = Channel::new();

pub struct Temperature {
    pub value: f32,
    pub description: str128,
}

pub struct Wind {
    pub speed: f32,
    pub direction: u16,
}

pub struct Tram {
    pub time: str128,
    pub adjustment: i64,
}

pub struct Weather {
    pub temperature: Temperature,
    pub wind: Wind,
}

pub struct Payload {
    pub weather: Weather,
    pub trams: [Tram; 2],
}

impl defmt::Format for Payload {
    fn format(&self, fmt: Formatter) {
        write!(fmt, "Payload({}, {})", self.weather, self.trams);
    }
}

impl defmt::Format for Weather {
    fn format(&self, fmt: Formatter) {
        write!(fmt, "Weather({}, {})", self.temperature, self.wind);
    }
}

impl defmt::Format for Temperature {
    fn format(&self, fmt: Formatter) {
        write!(
            fmt,
            "Temperature({}, {})",
            self.value,
            self.description.as_str()
        );
    }
}

impl defmt::Format for Wind {
    fn format(&self, fmt: Formatter) {
        write!(fmt, "Wind({}, {})", self.speed, self.direction);
    }
}

impl defmt::Format for Tram {
    fn format(&self, fmt: Formatter) {
        write!(fmt, "Tram({}, {})", self.time.as_str(), self.adjustment);
    }
}
