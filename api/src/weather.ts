import { wmoForCode } from "./wmo.js";

export const fetchWeather = async () => {
  let lat = 56.95;
  let lng = 24.11;

  let props = {
    latitude: lat,
    longitude: lng,
    current_weather: true,
    wind_speed_unit: 'kmh',
    daily: ['uv_index_max', 'uv_index_clear_sky_max', 'sunrise', 'sunset']
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
  console.log(json);
  return {
    temperature: {
      value: current.temperature,
      description: wmoForCode(current.weathercode as number),
    },
    wind:  {
      speed: current.windspeed,
      direction: current.winddirection,
    }
  };
}
