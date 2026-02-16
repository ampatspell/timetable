import { TZDate } from "@date-fns/tz";
import { loadStaticTimetable } from "./static.js";
import { LoadedRealtime, loadRealtime } from "./realtime.js";
import { loadParsedStaticData } from "./data.js";

const extractRealtime = ({ updates, tripId }: { updates: LoadedRealtime; tripId: string }) => {
  const update = updates.find(pb => pb.tripUpdate?.trip.tripId === tripId)?.tripUpdate;
  if(update) {
    let time = update.stopTimeUpdate?.find(u => !!u);
    if(time) {
      const delay = time.departure?.delay;
      if(delay !== 0) {
        return {
          update,
          time,
        };
      }
    }
  }
}

export const loadTimetable = async ({ routeId, stopId, directionId, now }: { routeId: string; stopId: string; directionId: string; now: TZDate; }) => {
  const loaded = await loadParsedStaticData();
  const [timetable, updates] = await Promise.all([
    loadStaticTimetable({ loaded, routeId, stopId, directionId, now }),
    loadRealtime(),
  ]);

  const realtime = timetable.map(entry => {
    const tripId = entry.trip.trip_id;
    const update = extractRealtime({ updates, tripId });
    return {
      ...entry,
      update,
    };
  });

  // Positive Integer: The vehicle is late by that many seconds.

  const first = realtime.slice(0, 2);

  return first.map(el => {
    const time = el.formatted;
    const delay = el.update?.time.departure?.delay ?? 0;
    return { time, delay };
  });
}
