import { parse as _parse } from "csv-parse";
import { readFile } from "node:fs/promises";
import { join } from "node:path";

// last-modified: Sat, 07 Feb 2026 00:27:57 GMT
// https://saraksti.rigassatiksme.lv/gtfs.zip

const root = join(import.meta.dirname, '..', '..', 'data');

const parse = async (name: string) => {
  const filename = join(root, `${name}.txt`);
  const buffer = await readFile(filename);
  return new Promise((resolve, reject) => {
    _parse(buffer, { relaxColumnCount: true, columns: true, bom: true }, (err, arr) => {
      if(err) {
        console.log(filename);
        return reject(err);
      }
      resolve(arr);
    });
  });
}

const _loadParsed = async () => {
  const [
    // agency,
    // attributions,
    calendarDates,
    calendar,
    // routes,
    // shapes,
    stopTimes,
    stops,
    trips,
  ] = await Promise.all([
    // parse('agency'),
    // parse('attributions'),
    parse('calendar_dates'),
    parse('calendar'),
    // parse('routes'),
    // parse('shapes'),
    parse('stop_times'),
    parse('stops'),
    parse('trips')
  ]);

  type Strings<T extends string> = { [key in T]: string }[];

  return {
    // agency,
    // attributions,
    calendarDates,
    calendar,
    // routes,
    // shapes,
    stopTimes,
    stops,
    trips
  } as {
    // agency: Strings<'agency_id' | 'agency_name' | 'agency_url' | 'agency_timezone' | 'agency_phone' | 'agency_lang'>,
    // attributions: Strings<'route_id' | 'organization_name' | 'is_operator'>,
    calendarDates: Strings<'service_id' | 'date' | 'exception_type'>
    calendar: Strings<'service_id' | 'monday' | 'tuesday' | 'wednesday' | 'thursday' | 'friday' | 'saturday' | 'sunday' | 'start_date' | 'end_date'>,
    // routes: Strings<'route_id' | 'route_short_name' | 'route_long_name' | 'route_desc' | 'route_type' | 'route_url' | 'route_color' | 'route_text_color' | 'route_sort_order'>,
    // shapes: Strings<'shape_id' |  'shape_pt_lat' | 'shape_pt_lon' | 'shape_pt_sequence' | 'shape_dist_traveled'>,
    stopTimes: Strings<'trip_id' | 'arrival_time' | 'departure_time' | 'stop_id' | 'stop_sequence' | 'timepoint' | 'pickup_type' | 'drop_off_type'>,
    stops: Strings<'stop_id' | 'stop_code' | 'stop_name' | 'stop_desc' | 'stop_lat' | 'stop_lon' | 'stop_url' | 'location_type' | 'parent_station'>,
    trips: Strings<'route_id' | 'service_id' | 'trip_id' | 'trip_headsign' | 'direction_id' | 'block_id' | 'shape_id' | 'wheelchair_accessible' | 'direction_name' | 'non_revenue_trip'>,
  };
}

export type ParsedStaticData = Awaited<ReturnType<typeof _loadParsed>>;

let promise: Promise<ParsedStaticData>;

export const loadParsedStaticData = () => {
  if(!promise) {
    promise = _loadParsed();
  }
  return promise;
}
