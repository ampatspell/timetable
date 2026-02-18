import { addDays, differenceInSeconds, format, getDay, parse, startOfDay } from "date-fns";
import { TZDate } from "@date-fns/tz";
import { sortedBy } from "../utils.js";
import { ParsedStaticData } from "./data.js";

// https://saraksti.rigassatiksme.lv/gtfs.zip

const weekdays = ['sunday', 'monday', 'tuesday',  'wednesday', 'thursday', 'friday', 'saturday'] as const;

const weekdayFromDate = (date: Date) => {
  const index = getDay(date);
  return weekdays[index];
}

const timetableFromStaticData = ({ filtered, now } : { filtered: FilteredStaticData, now: TZDate }) => {
  const weekday = weekdayFromDate(now);
  const tomorrow = startOfDay(addDays(now, 1));

  const today = sortedBy(filtered
    .filter(s => s.service[weekday] === '1')
    .map(s => {
      let [hours, minutes, seconds] = s.time!.departure_time.split(':').map(s => parseInt(s));
      let ref;
      if(hours > 23) {
        hours = hours - 24;
        ref = tomorrow;
      } else {
        ref = now;
      }

      const date = parse(`${hours}:${minutes}:${seconds}`, 'HH:mm:ss', ref);
      const formatted = format(date, 'HH:mm:ss');
      return {
        ...s,
        date,
        formatted,
      }
    }), { value: (s) => s.date, direction: 'asc'});

  return today.filter((s) => differenceInSeconds(s.date, now) > 0);
}

const filterStaticData = ({ loaded, routeId, stopId, directionId }: { loaded: ParsedStaticData, routeId: string; stopId: string; directionId: string }) => {
  const trips = loaded.trips.filter(trip => trip.route_id === routeId && trip.direction_id === directionId);
  const stopTimes = loaded.stopTimes.filter(time => time.stop_id === stopId);

  return trips.map(trip => {
    const time = stopTimes.find(s => s.trip_id === trip.trip_id);
    const service = loaded.calendar.find(calendar => calendar.service_id === trip.service_id)!;

    // dates → exception_type
    // 1 (Added): Service is added for this date.
    // 2 (Removed): Service is removed for this date.
    const dates = loaded.calendarDates.filter(date => date.service_id === service.service_id);
    return {
      trip,
      time,
      service,
      dates,
    };
  }).filter(trip => trip.time !== undefined);
}

type FilteredStaticData = ReturnType<typeof filterStaticData>;

// TODO: console.log(s.dates.map(d => d.exception_type));
export const loadStaticTimetable = async ({ loaded, routeId, stopId, directionId, now }: { loaded: ParsedStaticData, routeId: string; stopId: string; directionId: string; now: TZDate; }) => {
  const filtered = filterStaticData({ loaded, routeId, stopId, directionId })
  return timetableFromStaticData({ filtered, now });
}
