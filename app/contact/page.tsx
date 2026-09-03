import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Contact",
};

export default function Contact() {
  return (
    <div className="mx-auto w-full max-w-3xl px-4 py-16">
      <h1 className="mb-6 text-4xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-50">
        Contact us
      </h1>
      <p className="mb-8 max-w-xl leading-8 text-zinc-600 dark:text-zinc-400">
        Please feel free to reach out if you would like more info, and we will get back to you
        as soon as possible.
      </p>

      <div className="mb-10 space-y-6 text-zinc-700 dark:text-zinc-300">
        <div>
          <h2 className="text-sm font-medium uppercase tracking-wide text-zinc-500 dark:text-zinc-400">
            Email
          </h2>
          <a
            href="mailto:alkidharmacommunity@gmail.com"
            className="mt-1 block text-zinc-900 underline underline-offset-2 hover:decoration-zinc-400 dark:text-zinc-50"
          >
            alkidharmacommunity@gmail.com
          </a>
        </div>
      </div>
    </div>
  );
}