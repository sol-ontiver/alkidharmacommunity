import type { Metadata } from "next";
import Link from "next/link";
import { getEvents } from "@/lib/content";
import { formatDate, isUpcoming } from "@/lib/format";

export const metadata: Metadata = {
  title: "Events",
  description: "Upcoming special events at the Alki Dharma Community.",
};

export default function EventsPage() {
  const all = getEvents();
  const upcoming = all.filter(isUpcoming);
  const past = all.filter((e) => !isUpcoming(e));

  return (
    <div className="mx-auto w-full max-w-3xl px-4 py-16">
      <h1 className="mb-2 text-4xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-50">
        Events and Notices
      </h1>
      <p className="mb-10 text-zinc-600 dark:text-zinc-400">
        Special event announcements can be found here.
      </p>

      <h2 className="mb-4 text-xl font-semibold text-zinc-900 dark:text-zinc-50">
        Upcoming
      </h2>
      {upcoming.length > 0 ? (
        <ul className="divide-y divide-zinc-200 dark:divide-zinc-800">
          {upcoming.map((event) => (
            <li key={event.slug} className="py-5">
              <Link href={`/events/${event.slug}`} className="group block">
                <span className="block text-xs uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
                  {formatDate(event.start)} · {event.location}
                </span>
                <span className="mt-1 block text-lg font-medium text-zinc-900 group-hover:underline dark:text-zinc-50">
                  {event.title}
                </span>
                {event.excerpt && (
                  <span className="mt-1 block text-sm text-zinc-600 dark:text-zinc-400">
                    {event.excerpt}
                  </span>
                )}
              </Link>
            </li>
          ))}
        </ul>
      ) : (
        <p className="mb-10 text-zinc-500 dark:text-zinc-400">
          No upcoming events scheduled yet. Check back soon.
        </p>
      )}

      {past.length > 0 && (
        <>
          <h2 className="mb-4 mt-12 text-xl font-semibold text-zinc-900 dark:text-zinc-50">
            Past events
          </h2>
          <ul className="divide-y divide-zinc-200 dark:divide-zinc-800">
            {past.map((event) => (
              <li key={event.slug} className="py-4">
                <Link
                  href={`/events/${event.slug}`}
                  className="group block text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
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
