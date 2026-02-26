pub struct Time {
    pub hours: u8,
    pub minutes: u8,
    pub seconds: u8,
}

impl Time {
    pub fn add_second(&self) -> Self {
        let mut hours = self.hours;
        let mut minutes = self.minutes;
        let mut seconds = self.seconds;

        seconds += 1;
        if seconds > 59 {
            seconds = 0;
            minutes += 1;
            if minutes > 59 {
                minutes = 0;
                hours += 1;
                if hours > 23 {
                    hours = 0;
                }
            }
        }

        Time {
            hours,
            minutes,
            seconds,
        }
    }
}
