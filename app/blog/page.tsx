import type { Metadata } from "next";
import Link from "next/link";
import { getPosts } from "@/lib/content";
import { formatDate } from "@/lib/format";

export const metadata: Metadata = {
  title: "Blog",
  description:
    "News, announcements, and reflections from the Alki Dharma Community, a Buddhist meditation group in West Seattle.",
  keywords: [
    "meditation blog seattle",
    "buddhist news seattle",
    "dharma reflections",
    "west seattle sangha",
  ],
  openGraph: {
    title: "Blog | Alki Dharma Community",
    description:
      "News and reflections from the Alki Dharma Community in West Seattle.",
  },
};

export default function BlogPage() {
  const posts = getPosts();

  return (
    <div className="page-container">
      <h1 className="mb-2 page-title">
        Blog
      </h1>
      <p className="mb-10 muted-text">
        News and announcements from the community.
      </p>

      {posts.length > 0 ? (
        <ul className="list-divided">
          {posts.map((post) => (
            <li key={post.slug} className="py-6">
              <Link href={`/blog/${post.slug}`} className="group block">
                <span className="meta-label">
                  {formatDate(post.date)}
                  {post.tags.length > 0
                    ? ` · ${post.tags.join(", ")}`
                    : ""}
                </span>
                <span className="list-title text-xl">
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
        </ul>
      ) : (
        <p className="muted-text">
          No posts yet.
        </p>
      )}
    </div>
  );
}