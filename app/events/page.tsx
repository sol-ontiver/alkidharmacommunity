import type { Metadata } from "next";
import Link from "next/link";
import { getEvents } from "@/lib/content";
import { formatDate, isUpcoming } from "@/lib/format";

export const metadata: Metadata = {
  title: "Events",
  description:
    "Upcoming meditation and dharma events at the Alki Dharma Community in West Seattle, Seattle, WA.",
  keywords: [
    "meditation events seattle",
    "buddhist events west seattle",
    "dharma talks seattle",
    "meditation retreat seattle",
  ],
  openGraph: {
    title: "Events | Alki Dharma Community",
    description:
      "Upcoming special events at the Alki Dharma Community in West Seattle.",
  },
};

export default function EventsPage() {
  const all = getEvents();
  const upcoming = all.filter(isUpcoming);
  const past = all.filter((e) => !isUpcoming(e));

  return (
    <div className="page-container">
      <h1 className="mb-2 page-title">
        Events and Notices
      </h1>
      <p className="mb-10 muted-text">
        Special event announcements can be found here.
      </p>

      <h2 className="mb-4 section-title">
        Upcoming
      </h2>
      {upcoming.length > 0 ? (
        <ul className="list-divided">
          {upcoming.map((event) => (
            <li key={event.slug} className="py-5">
              <Link href={`/events/${event.slug}`} className="group block">
                <span className="meta-label">
                  {formatDate(event.start)} · {event.location}
                </span>
                <span className="list-title text-lg">
                  {event.title}
                </span>
                {event.excerpt && (
                  <span className="list-excerpt">
                    {event.excerpt}
                  </span>
                )}
              </Link>
            </li>
          ))}
        </ul>
      ) : (
        <p className="mb-10 muted-text">
          No upcoming events scheduled yet. Check back soon.
        </p>
      )}

      {past.length > 0 && (
        <>
          <h2 className="mb-4 mt-12 section-title">
            Past events
          </h2>
          <ul className="list-divided">
            {past.map((event) => (
              <li key={event.slug} className="py-4">
                <Link
                  href={`/events/${event.slug}`}
                  className="nav-link group block"
                >
                  <span className="block text-xs uppercase tracking-wide">
                    {formatDate(event.start)}
                  </span>
                  <span className="mt-1 block font-medium">{event.title}</span>
                </Link>
              </li>
            ))}
          </ul>
        </>
      )}
    </div>
  );
}
