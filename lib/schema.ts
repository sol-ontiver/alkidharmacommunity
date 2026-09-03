import { ORGANIZATION, SITE_URL, WEEKLY_MEETING } from "@/lib/constants";

const address = {
  "@type": "PostalAddress",
  streetAddress: ORGANIZATION.streetAddress,
  addressLocality: "Seattle",
  addressRegion: "WA",
  postalCode: ORGANIZATION.postalCode,
  addressCountry: "US",
} as const;

function toIsoDateTime(date: string, time?: string): string {
  if (!time) return new Date(date).toISOString();
  const match = time.match(/^(\d{1,2})(?::(\d{2}))?\s*(AM|PM)?$/i);
  if (!match) return new Date(date).toISOString();
  let hours = parseInt(match[1], 10);
  const minutes = match[2] ? parseInt(match[2], 10) : 0;
  const meridian = (match[3] || "").toUpperCase();
  if (meridian === "PM" && hours < 12) hours += 12;
  if (meridian === "AM" && hours === 12) hours = 0;
  const d = new Date(`${date}T${String(hours).padStart(2, "0")}:${String(minutes).padStart(2, "0")}:00`);
  return d.toISOString();
}

export function eventJsonLd(event: {
  title: string;
  start: string;
  end?: string;
  time?: string;
  location: string;
}): Record<string, unknown> {
  const startIso = toIsoDateTime(event.start, event.time);
  const endIso = event.end ? new Date(event.end).toISOString() : undefined;
  return {
    "@context": "https://schema.org",
    "@type": "Event",
    name: event.title,
    startDate: startIso,
    ...(endIso ? { endDate: endIso } : {}),
    eventStatus: "https://schema.org/EventScheduled",
    eventAttendanceMode: "https://schema.org/OfflineEventAttendanceMode",
    location: {
      "@type": "Place",
      name: event.location,
      address,
    },
  };
}

export function weeklyMeetingJsonLd(): Record<string, unknown> {
  const location = {
    "@type": "Place",
    name: WEEKLY_MEETING.venue,
    address,
  };
  return {
    "@context": "https://schema.org",
    "@type": "Event",
    name: WEEKLY_MEETING.name,
    description: WEEKLY_MEETING.description,
    eventStatus: "https://schema.org/EventScheduled",
    eventAttendanceMode: "https://schema.org/OfflineEventAttendanceMode",
    location,
    url: `${SITE_URL}/`,
    schedule: {
      "@type": "Schedule",
      repeatFrequency: "P1W",
      byDay: "MO",
      startTime: WEEKLY_MEETING.startTime,
      endTime: WEEKLY_MEETING.endTime,
      timeZone: WEEKLY_MEETING.timezone,
    },
  };
}

export function renderJsonLd(data: Record<string, unknown>): string {
  return JSON.stringify(data);
}
