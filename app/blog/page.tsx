import type { Metadata } from "next";
import Link from "next/link";
import { getPosts } from "@/lib/content";
import { formatDate } from "@/lib/format";

export const metadata: Metadata = {
  title: "Blog",
  description: "News and reflections from the Alki Dharma Community.",
};

export default function BlogPage() {
  const posts = getPosts();

  return (
    <div className="mx-auto w-full max-w-3xl px-4 py-16">
      <h1 className="mb-2 text-4xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-50">
        Blog
      </h1>
      <p className="mb-10 text-zinc-600 dark:text-zinc-400">
        News and announcements from the community.
      </p>

      {posts.length > 0 ? (
        <ul className="divide-y divide-zinc-200 dark:divide-zinc-800">
          {posts.map((post) => (
            <li key={post.slug} className="py-6">
              <Link href={`/blog/${post.slug}`} className="group block">
                <span className="block text-xs uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
                  {formatDate(post.date)}
                  {post.tags.length > 0
                    ? ` · ${post.tags.join(", ")}`
                    : ""}
                </span>
                <span className="mt-1 block text-xl font-medium text-zinc-900 group-hover:underline dark:text-zinc-50">
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
      ) : (
        <p className="text-zinc-500 dark:text-zinc-400">
          No posts yet.
        </p>
      )}
    </div>
  );
}