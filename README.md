# Alki Dharma Community

Website for the Alki Dharma Community

Built with Next.js 16 (App Router), React, TypeScript, Tailwind CSS, and Markdown content. Produces a fully **static site** (`npm run build` → `out/`) that can be hosted on any web host that serves HTML/CSS/JS.

## Development

```bash
npm run dev      # start the dev server at http://localhost:3000
npm run build    # lint + typecheck + static export into ./out
npm run lint     # run ESLint
```

Upload the contents of `out/` to your web host to publish. Re-run `npm run build` after every content change.

## Editing content

Content lives in Markdown files with YAML frontmatter — no CMS required, just edit and rebuild.

| Content   | Folder             | Frontmatter fields                                   |
| --------- | ------------------ | ---------------------------------------------------- |
| Blog posts | `content/posts/`  | `title`, `date`, `excerpt`, `tags[]`                 |
| Events    | `content/events/` | `title`, `start` (ISO date), `end`?, `location`, `excerpt` |

Example post:

```md
---
title: "A note on the summer sesshin"
date: "2026-08-01"
excerpt: "Details for this year's summer retreat."
tags: ["sesshin"]
---

Body text in Markdown.
```

The filename (minus `.md`) becomes the page slug. Dates are ISO strings; events sort by `start` and upcoming events come first.

## Pages

- `/` — landing page with latest events and posts
- `/about`, `/contact` — static info pages (edit `app/about/page.tsx`, `app/contact/page.tsx`)
- `/events` and `/events/[slug]` — event list and details
- `/blog` and `/blog/[slug]` — post list and details