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
          We do not follow any specific tradition. The structure of our meetings is as follows:
        </p>
        <ul className="list-disc space-y-2 pl-6">
          <li>Introductions and welcome</li>
          <li>Mindfulness meditation (20 minutes)</li>
          <li>Selected Reading</li>
          <li>Discussion</li>
          <li>Closing, selection of volunteer for next week's reading</li>
          <li>Closing Meditation (5 minutes)</li>
        </ul>
        <h2 className="pt-2 text-xl font-semibold text-zinc-900 dark:text-zinc-50">
          Donations
        </h2>
        <p>
          Please direct all donations to our gracious hosts at{" "}
          <a
            href="https://www.alkiucc.org/"
            target="_blank"
            rel="noopener noreferrer"
            className="underline decoration-zinc-300 underline-offset-2 hover:decoration-zinc-500 dark:decoration-zinc-700"
          >
            Alki UCC
          </a>. Follow the
          link and click "Donate", then select "Alki Dharma Meditation Group"


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
