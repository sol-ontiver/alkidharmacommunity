import type { Metadata } from "next";
import Link from "next/link";
import { notFound } from "next/navigation";
import Markdown from "@/components/markdown";
import { getPost, getPosts } from "@/lib/content";
import { formatDate } from "@/lib/format";

export function generateStaticParams() {
  const posts = getPosts();
  if (posts.length === 0) return [{ slug: "_empty" }];
  return posts.map((post) => ({ slug: post.slug }));
}

export async function generateMetadata({
  params,
}: {
  params: Promise<{ slug: string }>;
}): Promise<Metadata> {
  const { slug } = await params;
  const post = getPost(slug);
  if (!post) return {};
  return {
    title: post.title,
    description: post.excerpt,
    keywords: [...post.tags, "meditation", "dharma", "west seattle"],
    openGraph: {
      title: post.title,
      description: post.excerpt || undefined,
      type: "article",
    },
  };
}

export default async function PostPage({
  params,
}: {
  params: Promise<{ slug: string }>;
}) {
  const { slug } = await params;
  const post = getPost(slug);
  if (!post) notFound();

  return (
    <article className="page-container">
      <Link
        href="/blog"
        className="back-link"
      >
        ← All posts
      </Link>
      <h1 className="mt-4 mb-3 page-title">
        {post.title}
      </h1>
      <p className="mb-10 text-sm muted-text">
        {formatDate(post.date)}
      </p>
      <Markdown content={post.content} />
    </article>
  );
}