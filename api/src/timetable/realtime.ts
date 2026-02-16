import GtfsRealtimeBindings from "gtfs-realtime-bindings";

const url = 'https://saraksti.rigassatiksme.lv/gtfs_realtime.pb';

export const loadRealtime = async () => {

  // const loadUpdates = async () => {
  //   const url = 'https://saraksti.rigassatiksme.lv/trip_updates.pb';
  //   let res = await fetch(url);
  //   let buff = await res.arrayBuffer();
  //   return GtfsRealtimeBindings.transit_realtime.FeedMessage.decode(new Uint8Array(buff));
  // }

  // const loadPositions = async () => {
  //   const url = 'https://saraksti.rigassatiksme.lv/vehicle_positions.pb';
  //   let res = await fetch(url);
  //   let buff = await res.arrayBuffer();
  //   return GtfsRealtimeBindings.transit_realtime.FeedMessage.decode(new Uint8Array(buff));
  // }

  const res = await fetch(url);
  const buff = await res.arrayBuffer();
  return GtfsRealtimeBindings.transit_realtime.FeedMessage.decode(new Uint8Array(buff)).entity;
}

export type LoadedRealtime = Awaited<ReturnType<typeof loadRealtime>>;
