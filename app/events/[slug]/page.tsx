import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import Markdown from "@/components/markdown";
import { getEvent, getEvents } from "@/lib/content";
import { formatDate } from "@/lib/format";
import { eventJsonLd } from "@/lib/schema";

export function generateStaticParams() {
  const events = getEvents();
  if (events.length === 0) return [{ slug: "_empty" }];
  return events.map((event) => ({ slug: event.slug }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ slug: string }>;
}): Promise<Metadata> {
  const { slug } = await params;
  const event = getEvent(slug);
  if (!event) return {};
  return {
    title: event.title,
    description: event.excerpt,
    keywords: [
      "meditation event",
      "buddhist event seattle",
      "dharma",
      event.location,
    ].filter(Boolean),
    openGraph: {
      title: event.title,
      description: event.excerpt || undefined,
    },
  };
}

export default async function EventPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const event = getEvent(slug);
  if (!event) notFound();
  const eventLd = eventJsonLd({
    title: event.title,
    start: event.start,
    end: event.end,
    time: event.time,
    location: event.location || "Alki United Church of Christ",
  });

  return (
    <article className="page-container">
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(eventLd) }}
      />
      <Link
        href="/events"
        className="back-link"
      >
        ← All events
      </Link>
      <h1 className="mt-4 mb-3 page-title">
        {event.title}
      </h1>
      <div className="mb-10 text-sm muted-text">
        {formatDate(event.start)}
        {event.location ? ` · ${event.location}` : ""}
        {event.time ? ` · ${event.time}` : ""}
      </div>
      <Markdown content={event.content} />
    </article>
  );
}
