import { wmoForCode } from "./wmo.js";

export const fetchWeather = async () => {
  let lat = 56.95;
  let lng = 24.11;
  const res = await fetch(`https://api.open-meteo.com/v1/forecast?latitude=${lat}&longitude=${lng}&current_weather=true&wind_speed_unit=kmh`);
  const json = await res.json();
  const current = json.current_weather;
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
