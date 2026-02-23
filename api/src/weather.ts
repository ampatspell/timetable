import { TZDate } from "@date-fns/tz";
import { createNow, formatDiff, formatTime, parse, formatDate, resetDate } from "./utils.js";
import { wmoForCode } from "./wmo.js";

export const fetchWeather = async (lat: string, lng: string) => {
  let props = {
    latitude: lat,
    longitude: lng,
    current_weather: true,
    wind_speed_unit: 'kmh',
    daily: ['uv_index_max', 'uv_index_clear_sky_max', 'sunrise', 'sunset'],
    past_days: 1
  }

  let query = Object.keys(props).reduce<string[]>((arr, key) => {
    let value = props[key as keyof typeof props];
    (Array.isArray(value) ? value : [value]).map(item => String(item)).forEach((value) => {
      arr.push(`${key}=${value}`);
    });
    return arr;
  }, []).join('&');

  const res = await fetch(`https://api.open-meteo.com/v1/forecast?${query}`);
  const json = await res.json();
  const current = json.current_weather;
  const daily = json.daily;

console.log(json);

  let now = formatDate(createNow());
  let index = daily.time.indexOf(now);

  let temperature = {
    value: current.temperature,
    description: wmoForCode(current.weathercode as number),
  };

  let uv = {
    max: daily.uv_index_max[index],
    clearSkyMax: daily.uv_index_clear_sky_max[index],
  };

  let createSunrise = () => {
    let date = parse(daily.sunrise[index]);
    let yesterday = parse(daily.sunrise[index-1]);
    return {
      time: formatTime(date),
      diff: formatDiff(resetDate(date), resetDate(yesterday)),
    };
  }

  let sunrise = createSunrise();

  let createSunset = () => {
    let date = parse(daily.sunset[index]);
    let yesterday = parse(daily.sunset[index - 1]);
    return {
      time: formatTime(date),
      diff: formatDiff(resetDate(date), resetDate(yesterday))
    }
  }

  let sunset = createSunset();

  return {
    temperature,
    uv,
    sunrise,
    sunset
  };
}
