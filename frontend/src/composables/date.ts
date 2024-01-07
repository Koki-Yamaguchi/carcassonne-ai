export function useFormatDate(d: Date) {
  d.setTime(d.getTime() + 1000 * 60 * 60 * 9);
  return d
    .toLocaleDateString("ja-JP", {
      year: "numeric",
      month: "2-digit",
      day: "2-digit",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
    })
    .split("/")
    .join("-");
}
