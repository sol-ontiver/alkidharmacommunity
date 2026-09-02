import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import Markdown from "@/components/markdown";
import { getEvent, getEvents } from "@/lib/content";
import { formatDate } from "@/lib/format";

export function generateStaticParams() {
  return getEvents().map((event) => ({ slug: event.slug }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ slug: string }>;
}): Promise<Metadata> {
  const { slug } = await params;
  const event = getEvent(slug);
  if (!event) return {};
  return { title: event.title };
}

export default async function EventPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const event = getEvent(slug);
  if (!event) notFound();

  return (
    <article className="mx-auto w-full max-w-3xl px-4 py-16">
      <Link
        href="/events"
        className="text-sm text-zinc-500 hover:text-zinc-900 dark:text-zinc-400 dark:hover:text-zinc-100"
      >
        ← All events
      </Link>
      <h1 className="mt-4 mb-3 text-4xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-50">
        {event.title}
      </h1>
      <p className="mb-10 text-sm text-zinc-500 dark:text-zinc-400">
        {formatDate(event.start)}
        {event.location ? ` · ${event.location}` : ""}
      </p>
      <Markdown content={event.content} />
    </article>
  );
}