import Link from "next/link";
import { getEvents, getPosts } from "@/lib/content";
import { formatDate, isUpcoming } from "@/lib/format";

export default function Home() {
  const events = getEvents().filter(isUpcoming).slice(0, 3);
  const posts = getPosts().slice(0, 3);

  return (
    <div className="mx-auto w-full max-w-3xl px-4 py-16">
      <section className="mb-16">
        <h1 className="mb-4 text-4xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-50">
          Alki Dharma Community
        </h1>
        <p className="max-w-xl text-lg leading-8 text-zinc-600 dark:text-zinc-400">
          A Buddhist meditation group in West Seattle
        </p>
        <div className="mt-8 flex gap-4">
          <Link
            href="/events"
            className="rounded-full bg-zinc-900 px-5 py-2.5 text-sm font-medium text-white hover:bg-zinc-700 dark:bg-zinc-50 dark:text-zinc-900 dark:hover:bg-zinc-200"
          >
            Upcoming events
          </Link>
          <Link
            href="/about"
            className="rounded-full border border-zinc-300 px-5 py-2.5 text-sm font-medium text-zinc-700 hover:border-transparent hover:bg-zinc-100 dark:border-zinc-700 dark:text-zinc-200 dark:hover:bg-zinc-800"
          >
            About us
          </Link>
        </div>
      </section>

      <section className="mb-14">
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-xl font-semibold text-zinc-900 dark:text-zinc-50">
            Upcoming events
          </h2>
          <Link
            href="/events"
            className="text-sm text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
          >
            View all
          </Link>
        </div>
        {events.length > 0 ? (
          <ul className="divide-y divide-zinc-200 dark:divide-zinc-800">
            {events.map((event) => (
              <li key={event.slug} className="py-4">
                <Link href={`/events/${event.slug}`} className="group">
                  <span className="block text-xs uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
                    {formatDate(event.start)} · {event.location}
                  </span>
                  <span className="mt-1 block font-medium text-zinc-900 group-hover:underline dark:text-zinc-50">
                    {event.title}
                  </span>
                </Link>
              </li>
            ))}
          </ul>
        ) : (
          <p className="text-zinc-500 dark:text-zinc-400">
            No upcoming events scheduled yet.
          </p>
        )}
      </section>

      <section>
        <div className="mb-4 flex items-center justify-between">
          <h2 className="text-xl font-semibold text-zinc-900 dark:text-zinc-50">
            Latest news
          </h2>
          <Link
            href="/blog"
            className="text-sm text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
          >
            View all
          </Link>
        </div>
        <ul className="divide-y divide-zinc-200 dark:divide-zinc-800">
          {posts.map((post) => (
            <li key={post.slug} className="py-4">
              <Link href={`/blog/${post.slug}`} className="group">
                <span className="block text-xs uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
                  {formatDate(post.date)}
                </span>
                <span className="mt-1 block font-medium text-zinc-900 group-hover:underline dark:text-zinc-50">
                  {post.title}
                </span>
                {post.excerpt && (
                  <span className="mt-1 block text-sm text-zinc-600 dark:text-zinc-400">
                    {post.excerpt}
                  </span>
                )}
              </Link>
            </li>
          ))}
        </ul>
      </section>
    </div>
  );
}