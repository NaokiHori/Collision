export function getNumber(
  keyword: string,
  defval: number,
  minval: number,
  maxval: number,
): number {
  // check if URL param is given
  const urlParams = new URLSearchParams(window.location.search);
  // if not given, use default value
  let val: number = defval;
  if (urlParams.has(keyword)) {
    // if given, use after sanitised
    let tmp: number | null = Number(urlParams.get(keyword));
    if (tmp) {
      tmp = tmp < minval ? minval : tmp;
      tmp = maxval < tmp ? maxval : tmp;
      val = tmp;
    }
  }
  return val;
}
