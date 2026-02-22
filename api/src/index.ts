import Koa from 'koa';
import Router from '@koa/router';
import { fetchWeather } from './weather.js';
import { loadTimetable } from './timetable/index.js';
import { TZDate } from '@date-fns/tz';
import { setTimeout } from 'node:timers/promises';
import dedent from 'dedent';

const router = new Router();

router.get('/', async (ctx) => {
  const weather = await fetchWeather();
  const timetable =  await loadTimetable({
    routeId: 'riga_tram_1',
    stopId: '3123',
    directionId: '1',
    now: new TZDate(new Date(), 'Europe/Riga')
  });


  let t = (idx: number) => {
    let value = timetable[idx];
    if(value) {
      let d = value.delay ? `${value.delay}s` : undefined;
      return [value.time, d].filter(Boolean).join(' ');
    }
    return '';
  }

  let now = new TZDate(new Date(), 'Europe/Riga');
  let y = now.getFullYear();
  let mo = now.getMonth();
  let d = now.getDate();
  let h = now.getHours();
  let m = now.getMinutes();
  let s = now.getSeconds();

  let lines = dedent`
    ${y}
    ${mo}
    ${d}
    ${h}
    ${m}
    ${s}
    clock
    01:04:42

    cloud-snow
    ${weather.temperature.value}
    ${weather.temperature.description?.short ?? ''}
    sun
    01

    sunrise
    06:39:10

    sunset
    03:11:45

    bus-stop
    ${t(0)}
    ${t(1)}
  `;

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

