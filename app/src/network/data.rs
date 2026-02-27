use no_std_strings::{str12, str32};

pub struct Weather {
    pub icon: str12,
    pub temperature: str32,
    pub description: str32,
    pub uv: str32,
    pub sunrise: str32,
    pub sunset: str32,
}
