use no_std_strings::str128;
use ui::payload::{Payload, Temperature, Tram, Weather, Wind};

pub fn parse(body: &str) -> Payload {
    let mut iter = body.split('\n').into_iter();

    let temperature = {
        let value = iter.next().unwrap().parse::<f32>().unwrap();
        let description = str128::from(iter.next().unwrap());
        Temperature { value, description }
    };

    let wind = {
        let speed = iter.next().unwrap().parse::<f32>().unwrap();
        let direction = iter.next().unwrap().parse::<u16>().unwrap();
        Wind { speed, direction }
    };

    let weather = Weather { temperature, wind };

    let trams = {
        let mut parse = || {
            let time = str128::from(iter.next().unwrap());
            let adjustment = iter.next().unwrap().parse::<i64>().unwrap();
            Tram { time, adjustment }
        };

        (parse(), parse())
    };

    Payload {
        weather,
        trams: trams.into(),
    }
}
