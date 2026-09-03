import Link from "next/link";
import { getEvents, getPosts } from "@/lib/content";
import { formatDate, isUpcoming } from "@/lib/format";
import { DONATE_URL } from "@/lib/constants";
import { weeklyMeetingJsonLd } from "@/lib/schema";

export default function Home() {
  const events = getEvents().filter(isUpcoming).slice(0, 3);
  const posts = getPosts().slice(0, 3);
  const weeklyJsonLd = weeklyMeetingJsonLd();

  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(weeklyJsonLd) }}
      />
      <img
        src="/banner-placeholder.svg"
        alt="Alki Dharma Community banner"
        className="mb-0 h-auto w-full border-b border-zinc-200 dark:border-zinc-800"
      />
      <div className="page-container">
      <div className="mb-8 flex justify-end">
        <Link
          href={DONATE_URL}
          className="primary-cta"
        >
          Donate
        </Link>
      </div>
      <section className="relative">
        <h1 className="mb-4 page-title">
          Alki Dharma Community
        </h1>
        <p className="max-w-xl text-lg body-text">
          A Buddhist meditation group in West Seattle - join us weekly for meditation, reading, and discussion. 
          We welcome everyone regardless of faith or identity,
          and all experience levels are appreciated! 
        </p>
        <img
          src="/intro-placeholder.svg"
          alt="Alki Dharma Community meditation"
          className="mt-6 h-auto w-full max-w-xl rounded-xl border border-zinc-200 shadow-sm dark:border-zinc-800"
        />
        <div className="mt-8 flex flex-wrap gap-4">
          <Link
            href="/about"
            className="secondary-cta"
          >
            About us
          </Link>
          <Link
            href="/resources"
            className="secondary-cta"
          >
            Meditation resources
          </Link>
          </div>
      </section>

            {/* <section>
        <div className="mt-8 flex gap-4">
          <Link
            href="/about"
            className="primary-cta"
          >
            About us
          </Link>
          <Link
            href="https://secure.myvanco.com/L-ZXXH/campaign/C-16CDA"
            className="secondary-cta"
          >
            Donate
          </Link>
        </div>
      </section> */}

      </div>

      <div className="page-container">
      <section className="mb-14">
        <div className="mb-4 flex items-center justify-between">
          <h3 className="section-title">
            Special Events
          </h3>
          <Link
            href="/events"
            className="back-link"
          >
            View all
          </Link>
        </div>
        {events.length > 0 ? (
          <ul className="list-divided">
            {events.map((event) => (
              <li key={event.slug} className="py-4">
                <Link href={`/events/${event.slug}`} className="group">
                  <span className="meta-label">
                    {formatDate(event.start)} · {event.location}
                  </span>
                  <span className="list-title">
                    {event.title}
                  </span>
                </Link>
              </li>
            ))}
          </ul>
        ) : (
          <p className="muted-text">
            No upcoming events scheduled yet.
          </p>
        )}
      </section>

      <section>
        <div className="mb-4 flex items-center justify-between">
          <h3 className="section-title">
            Latest news
          </h3>
          <Link
            href="/blog"
            className="back-link"
          >
            View all
          </Link>
        </div>
        {posts.length > 0 ? (
          <ul className="list-divided">
            {posts.map((post) => (
              <li key={post.slug} className="py-4">
                <Link href={`/blog/${post.slug}`} className="group">
                  <span className="meta-label">
                    {formatDate(post.date)}
                  </span>
                  <span className="list-title">
                    {post.title}
                  </span>
                  {post.excerpt && (
                    <span className="list-excerpt">
                      {post.excerpt}
                    </span>
                  )}
                </Link>
              </li>
            ))}
          </ul>) :
          (
            <p className="muted-text">
              No blog posts yet.
            </p>
          )}
      </section>
      </div>

      <div className="mt-12 w-full bg-meeting text-white">
        <section className="mx-auto flex w-full max-w-3xl flex-col items-center gap-8 px-4 py-12 sm:flex-row">
          <img
            src="/monday-placeholder.svg"
            alt="Monday meeting"
            className="h-56 w-56 shrink-0 rounded-xl border border-white/20 object-cover sm:h-64 sm:w-64"
          />
          <div className="min-w-0 flex-1">
            <h2 className="mb-4 page-title text-white">
              Monday Meeting
            </h2>
            <p className="text-lg text-zinc-100">
              We meet every Monday night <br />
              Doors open around 6:45 PM <br />
              Meeting is held from 7-8:30 PM. <br />
            </p>
            <h2 className="pt-2 section-title text-white">
              Location
            </h2>
            <p className="text-lg text-zinc-100">
              Alki United Church of Christ <br />
              6115 SW Hinds St, Seattle, WA 98116
            </p>
            <div className="mt-4">
              <a
                href="https://www.google.com/maps/search/?api=1&query=Alki+United+Church+of+Christ+6115+SW+Hinds+St+Seattle+WA+98116"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 rounded-md border border-white/30 bg-white/10 px-4 py-2 text-sm font-medium text-white transition hover:bg-white/20"
              >
                View on Google Maps
              </a>
            </div>
            <div className="mt-4 space-y-2 text-sm text-zinc-100">
              <p>
                We ask that you please try to arrive on time, as we tend to close the doors
                once we are seated for meditation.
              </p>
              <p>
                Donations are appreciated but not required.
              </p>
            </div>
          </div>
        </section>
      </div>

      <section className="w-full border-t border-zinc-200 pt-6 dark:border-zinc-800">
        <div className="mx-auto w-full max-w-3xl px-4">
          <p className="text-center text-xs tracking-wide muted-text">
            This site is currently under construction. Please don&apos;t be too put off if it looks a little drab right now. I&apos;m doing my best over here.
          </p>
        </div>
      </section>
    </>
  );
}
