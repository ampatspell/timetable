use no_std_strings::{str12, str16};

pub struct Weather {
    pub icon: str12,
    pub temperature: str16,
    pub description: str16,
    pub uv: str16,
    pub sunrise: str16,
    pub sunset: str16,
}
