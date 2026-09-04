import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import Markdown from "@/components/markdown";
import DownloadPdf from "@/components/download-pdf";
import { getResource, getResources } from "@/lib/content";

export function generateStaticParams() {
  const resources = getResources();
  if (resources.length === 0) return [{ slug: "_empty" }];
  return resources.map((resource) => ({ slug: resource.slug }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ slug: string }>;
}): Promise<Metadata> {
  const { slug } = await params;
  const resource = getResource(slug);
  if (!resource) return {};
  return {
    title: resource.title,
    description: resource.description,
    keywords: [
      resource.title,
      "meditation",
      "dharma",
      "buddhist practice",
      "west seattle",
    ],
    openGraph: {
      title: resource.title,
      description: resource.description || undefined,
      type: "article",
    },
  };
}

export default async function ResourcePage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const resource = getResource(slug);
  if (!resource) notFound();

  return (
    <article className="page-container print-area">
      <div className="flex items-center justify-between">
        <Link
          href="/resources"
          className="back-link"
        >
          ← All resources
        </Link>
        <DownloadPdf />
      </div>
      <div className="mt-8 overflow-hidden rounded-2xl border border-zinc-200 bg-white shadow-sm dark:border-zinc-800 dark:bg-zinc-900/60">
        <div className="px-8 py-8 sm:px-10">
          <h1 className="mb-8 page-title">
            {resource.title}
          </h1>
          <Markdown content={resource.content} />
        </div>
      </div>
    </article>
  );
}
