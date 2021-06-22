export default function metaEvents(
  events: readonly string[]
): Record<string, unknown> {
  return events
    .map((event) => `on${event[0].toUpperCase()}${event.slice(1)}`)
    .reduce((acc, cur) => {
      acc[cur] = {};
      return acc;
    }, {} as Record<string, unknown>);
}
