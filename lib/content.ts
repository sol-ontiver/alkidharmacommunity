import fs from "node:fs";
import path from "node:path";
import matter from "gray-matter";

const contentRoot = path.join(process.cwd(), "content");

export type Post = {
  slug: string;
  title: string;
  date: string;
  excerpt: string;
  tags: string[];
  content: string;
};

export type CommunityEvent = {
  slug: string;
  title: string;
  recurring?: string;
  start: string;
  end?: string;
  location: string;
  excerpt: string;
  content: string;
};

function readMarkdownFiles<T>(dir: string, map: (data: Record<string, unknown>, slug: string, content: string) => T): T[] {
  const fullPath = path.join(contentRoot, dir);
  if (!fs.existsSync(fullPath)) return [];

  return fs
    .readdirSync(fullPath)
    .filter((file) => file.endsWith(".md"))
    .map((file) => {
      const raw = fs.readFileSync(path.join(fullPath, file), "utf8");
      const { data, content } = matter(raw);
      const slug = file.replace(/\.md$/, "");
      return map({ ...data }, slug, content.trim());
    });
}

export function getPosts(): Post[] {
  return readMarkdownFiles("posts", (data, slug, content) => ({
    slug,
    title: String(data.title ?? slug),
    date: String(data.date ?? ""),
    excerpt: String(data.excerpt ?? ""),
    tags: Array.isArray(data.tags) ? data.tags.map(String) : [],
    content,
  })).sort((a, b) => b.date.localeCompare(a.date));
}

export function getPost(slug: string): Post | undefined {
  return getPosts().find((post) => post.slug === slug);
}

export function getEvents(): CommunityEvent[] {
  return readMarkdownFiles("events", (data, slug, content) => ({
    slug,
    title: String(data.title ?? slug),
    recurring: String(data.recurring ?? slug),
    start: String(data.start ?? ""),
    end: data.end ? String(data.end) : undefined,
    location: String(data.location ?? ""),
    excerpt: String(data.excerpt ?? ""),
    content,
  })).sort((a, b) => a.start.localeCompare(b.start));
}

export function getEvent(slug: string): CommunityEvent | undefined {
  return getEvents().find((event) => event.slug === slug);
}