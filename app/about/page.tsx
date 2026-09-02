import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "About",
};

export default function About() {
  return (
    <div className="mx-auto w-full max-w-3xl px-4 py-16">
      <h1 className="mb-6 text-4xl font-semibold tracking-tight text-zinc-900 dark:text-zinc-50">
        About our Sangha
      </h1>
      <div className="space-y-6 leading-8 text-zinc-600 dark:text-zinc-400">
        <p>
          The Alki Dharma Community is a Buddhist meditation group open to anyone, at any
          skill level. We meet weekly to practice meditation, hear selected readings, and
          discuss the Dharma with one another.
        </p>
        <p>
          We do not follow any specific tradition. The structure of our meetings are as follows:
          {/* Make this a list */}
          * Introductions and welcome
          * Mindfulness meditation (20 minutes)
          * Selected Reading
          * Discussion
          * Closing, selection of volunteer for next week's reading
          * Closing Meditation (5 minutes)
        </p>
        <h2 className="pt-2 text-xl font-semibold text-zinc-900 dark:text-zinc-50">
          Location
        </h2>
        <p>
          We meet at Alki United Church of Christ
        </p>
        <h2 className="pt-2 text-xl font-semibold text-zinc-900 dark:text-zinc-50">
          Newsletter
        </h2>
        <p>
          We have a mailing list where we will ocassionally send out schedule updates,
          holiday schedules, etc. You can sign up on the{" "}
          <a
            href="/contact"
            className="underline decoration-zinc-300 underline-offset-2 hover:decoration-zinc-500 dark:decoration-zinc-700"
          >
            contact page
          </a>
          .
        </p>
      </div>
    </div>
  );
}