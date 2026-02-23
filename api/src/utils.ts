import { TZDate } from "@date-fns/tz";
import { differenceInMinutes, differenceInSeconds, format } from "date-fns";

export type SortDescriptor<T> = {
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  value: (object: T) => any;
  direction?: 'asc' | 'desc';
};

export type SortDescriptors<T> = SortDescriptor<T> | SortDescriptor<T>[];

export function sortedBy<T>(arr: T[], descriptors: SortDescriptors<T>): T[] {
  if (!Array.isArray(descriptors)) {
    descriptors = [descriptors];
  }
  let sorted = [...arr];
  for (const descriptor of descriptors) {
    sorted = sorted.sort((a, b) => {
      const av = descriptor.value(a);
      const bv = descriptor.value(b);
      if (av === bv) {
        return 0;
      }
      if (descriptor.direction === 'desc') {
        return av < bv ? 1 : -1;
      } else {
        return av < bv ? -1 : 1;
      }
    });
  }
  return sorted;
}

export function isTruthy<T>(value?: T | undefined | null | false): value is T {
  return !!value;
}

export const asString = (arg: string | string[] | undefined) => {
  if(Array.isArray(arg)) {
    return arg[0];
  }
  return arg;
}

export const formatDate = (date: TZDate) => {
  return format(date, 'yyyy-MM-dd');
}

export const formatTime = (date: TZDate | undefined) => {
  if(date) {
    return format(date, 'HH:mm:ss');
  }
}

export const parse = (date: string | undefined) => {
  if(date) {
    return new TZDate(date).withTimeZone('Europe/Riga');
  }
}

export const replaceDate = (a: TZDate | undefined, b: TZDate | undefined) => {
  if(a && b) {
    let ret = new TZDate(a);
    ret.setHours(b.getHours());
    ret.setMinutes(b.getMinutes());
    ret.setSeconds(b.getSeconds());
    ret.setMilliseconds(b.getMilliseconds());
    return ret;
  }
}

export const formatSeconds = (seconds: number) => {
    let sign = seconds < 0 ? '-' : '';
    seconds = Math.abs(seconds);
    if(seconds === 0) {
      return '';
    }
    if(seconds < 60) {
      return `${sign}${seconds}s`;
    }
    let minutes = Math.floor(seconds / 60);
    let remainder = seconds - (minutes * 60);
    return [`${sign}${minutes}m`, remainder > 0 && `${remainder}s`].filter(isTruthy).join('');
}

export const formatDiff = (a: TZDate | undefined, b: TZDate | undefined) => {
  if(a && b) {
    return formatSeconds(differenceInSeconds(a, b));
  }
}

export const createNow = () => new TZDate(new Date()).withTimeZone('Europe/Riga');
