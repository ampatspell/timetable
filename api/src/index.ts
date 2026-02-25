import Koa from 'koa';
import Router from '@koa/router';
import { fetchWeather } from './weather.js';
import { loadTimetable } from './timetable/index.js';
import dedent from 'dedent';
import { asString, createNow, formatSeconds, formatTime } from './utils.js';
import { TZDate, tzOffset } from '@date-fns/tz';

const router = new Router();

router.get('/weather', async (ctx) => {
  // lat=56.95570916409245&lng=24.12422103404933
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
        let delay = item.delay;
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

router.get('/font', async (ctx) => {
  let fontSize = asString(ctx.query['font-size']) ?? '20';
  ctx.headers['content-type'] = 'text/html';

  let numbers = '×0123456789';
  let lowercase = 'abcdefghijklmnopqrstuvwxyzāčēģīķļņšūž';
  let uppercase = lowercase.toUpperCase();
  let special = '°+-';
  let glyphs = [...numbers, ...lowercase, ...uppercase, ...special].join('');

  ctx.body = dedent`
    <!doctype html>
    <html lang="en">
    <head>
      <link rel="preconnect" href="https://fonts.googleapis.com">
      <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
      <link href="https://fonts.googleapis.com/css2?family=Ubuntu+Mono:ital,wght@0,400;0,700;1,400;1,700&display=swap" rel="stylesheet">

      <style>
        html, body {
          margin: 0;
          background: #fff;
          color: #000;
          font-family: 'Ubuntu Mono';
          font-size: ${fontSize}px;
        }

        body {
          display: flex;
          flex-direction: column;
          gap: 10px;
          padding: 25px;
        }

        .measure {
          display: flex;
          flex-direction: row;
          width: min-content;
        }

        .content {
          display: flex;
          flex-direction: row;
        }
      </style>
      </head>
      <body>
        <div class="content">
          <div class="row"></div>
        </div>
        <div class="measure">M</div>
        <script>
          window.addEventListener('DOMContentLoaded', () => {
            let body = document.body;
            let row = "${glyphs}";
            body.querySelector('.row').textContent = row;

            let done = document.createElement('div');
            done.className = "done";
            body.append(done);
          });
        </script>
      </body>
    </html>
  `;
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
