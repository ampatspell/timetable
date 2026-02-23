import Koa from 'koa';
import Router from '@koa/router';
import { fetchWeather } from './weather.js';
import { loadTimetable } from './timetable/index.js';
import dedent from 'dedent';
import { asString, createNow, formatSeconds } from './utils.js';

const router = new Router();

router.get('/weather', async (ctx) => {
  let lat = asString(ctx.query['lat']);
  let lng = asString(ctx.query['lng']);
  if(lat && lng) {
    const weather = await fetchWeather(lat, lng);

    ctx.body = dedent`
      cloud-snow
      ${weather.temperature.value}
      ${weather.temperature.description?.short ?? ''}
      ${weather.uv.max}
      ${weather.uv.clearSkyMax}
      ${weather.sunrise.time}
      ${weather.sunrise.diff}
      ${weather.sunset.time}
      ${weather.sunset.diff}
    `;
  } else {
    ctx.body = dedent`
      cloud-snow
      No lat/lng
    `;
  }
});

router.get('/now', async (ctx) => {
  let now = createNow();
  let year = now.getFullYear();
  let month = now.getMonth();
  let date = now.getDate();
  let hours = now.getHours();
  let minutes = now.getMinutes();
  let seconds = now.getSeconds();
  let millis = now.getMilliseconds();
  ctx.body = dedent`
    ${year}
    ${month}
    ${date}
    ${hours}
    ${minutes}
    ${seconds}
    ${millis}
  `;
});

router.get('/timetable', async (ctx) => {
  // route=riga_tram_1&stop=3123&direction=1
  let routeId = asString(ctx.query['route']);
  let stopId = asString(ctx.query['stop']);
  let directionId = asString(ctx.query['direction']);
  if(routeId && stopId && directionId) {
    const timetable = await loadTimetable({
        routeId,
        stopId,
        directionId,
        now: createNow()
      });
      ctx.body = timetable.map(item => {
        let time = item.time;
        let delay = item.delay;;
        return `${time} ${formatSeconds(delay)}`;
      }).join('\n');
  } else {
    ctx.body = dedent`
      cloud-snow
      No route/stop/direction
    `
  }
});

router.get('/health', async (ctx) => {
  ctx.body = { ok: true };
});

const app = new Koa();
app.use(router.routes()).use(router.allowedMethods());
app.listen(3000);

// (async () => {
//   const t =  await loadTimetable({
//     routeId: 'riga_tram_1',
//     stopId: '3123',
//     directionId: '1',
//     now: new TZDate(new Date(), 'Europe/Riga')
//   });
//   console.log(t);
// })()

