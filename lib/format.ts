export function formatDate(iso: string): string {
  return new Date(iso).toLocaleDateString("en-US", {
    year: "numeric",
    month: "long",
    day: "numeric",
  });
}

export function isUpcoming(event: { start: string }): boolean {
  return new Date(event.start).getTime() >= Date.now();
}