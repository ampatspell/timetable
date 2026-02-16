import Koa from 'koa';
import Router from '@koa/router';
import { fetchWeather } from './weather.js';
import { loadTimetable } from './timetable/index.js';
import { TZDate } from '@date-fns/tz';
import { setTimeout } from 'node:timers/promises';

const router = new Router();

router.get('/', async (ctx) => {
  const weather = await fetchWeather();
  const timetable =  await loadTimetable({
    routeId: 'riga_tram_1',
    stopId: '3123',
    directionId: '1',
    now: new TZDate(new Date(), 'Europe/Riga')
  });

  const lines = [
    weather.temperature.value,
    weather.temperature.description?.long ?? '',
    weather.wind.speed,
    weather.wind.direction,
    ...(timetable.map(entry => {
      return [entry.time, entry.delay];
    })).flat(),
    '',
    '---',
    '',
    JSON.stringify({ weather, timetable }, null, 2),
  ].join('\n');

  ctx.body = lines;
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
