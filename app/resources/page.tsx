import type { Metadata } from "next";
import Link from "next/link";
import { getResources } from "@/lib/content";

export const metadata: Metadata = {
  title: "Meditation Resources",
  description:
    "Meditation and dharma resources from the Alki Dharma Community in West Seattle — readings, practices, and guidance for mindfulness.",
  keywords: [
    "meditation resources",
    "buddhist readings",
    "dharma resources seattle",
    "meditation practice guides",
  ],
  openGraph: {
    title: "Meditation Resources | Alki Dharma Community",
    description:
      "Meditation and dharma resources from the Alki Dharma Community in West Seattle.",
  },
};

export default function ResourcesPage() {
  const resources = getResources();

  return (
    <div className="page-container">
      <h1 className="mb-8 page-title">
        Resources
      </h1>
      {resources.length > 0 ? (
        <div className="overflow-hidden rounded-2xl border border-zinc-200 bg-white shadow-sm dark:border-zinc-800 dark:bg-zinc-900/60">
          <ul className="divide-y divide-zinc-200 dark:divide-zinc-800">
            {resources.map((resource) => (
              <li
                key={resource.slug}
                className="px-6 py-5 transition-colors hover:bg-zinc-50 dark:hover:bg-zinc-800/50"
              >
                <div className="flex items-baseline justify-between gap-4">
                  <Link
                    href={`/resources/${resource.slug}`}
                    className="list-title"
                  >
                    {resource.title}
                  </Link>
                  {resource.description && (
                    <span className="text-sm text-zinc-600 dark:text-zinc-400 text-right">
                      {resource.description}
                    </span>
                  )}
                </div>
              </li>
            ))}
          </ul>
        </div>
      ) : (
        <p className="muted-text">
          No resources available yet.
        </p>
      )}
    </div>
  );
}
