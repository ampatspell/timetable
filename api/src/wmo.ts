const mapping: Record<string, [string, string]> = {
  "0": [ "Skaidrs", "01"],
  "1": [ "Apmēram skaidrs", "02"],
  "2": [ "Bik mākoņains", "02"],
  "3": [ "Apmācies", "03"],
  "45": [ "Miglains", "50"],
  "48": [ "Sarma, migla", "50"],
  "51": [ "Smidzina", "10"],
  "53": [ "Mēreni smidzina", "09"],
  "55": [ "Ļoti smidzina", "09"],
  "56": [ "Bik salstoši smidzina", "09"],
  "57": [ "Ļoti salstoši smidzina", "09"],
  "61": [ "Bik lietus", "10"],
  "63": [ "Lietus", "09"],
  "65": [ "Riktīgs lietus", "09"],
  "66": [ "Bik salst lietus", "09"],
  "67": [ "Pamatīgs salst lietus", "09"],
  "71": [ "Bik snieg", "13"],
  "73": [ "Snieg", "13"],
  "75": [ "Riktīgi snieg", "13"],
  "77": [ "Sniega graudi", "13"],
  "80": [ "Bik lietusgāzes", "10"],
  "81": [ "Lietusgāzes", "09"],
  "82": [ "Pamatīgas lietusgāzes", "09"],
  "85": [ "Bik sniegs līst", "13"],
  "86": [ "Sniegs līst", "13"],
  "95": [ "Negaiss", "07"],
  "96": [ "Negaiss, bik krusa", "07"],
  "99": [ "Negaiss, krusa", "07"],
};

export const wmoForCode = (code: number) => {
  const key = String(code) as keyof typeof mapping;
  let arr = mapping[key];
  if(arr) {
    return arr[0]!;
  }
}
