import type { Metadata } from "next";
import { DONATE_URL } from "@/lib/constants";

export const metadata: Metadata = {
  title: "About",
  description:
    "About the Alki Dharma Community, a Buddhist meditation group in West Seattle welcoming all levels of practice for meditation, dharma reading, and discussion.",
  keywords: [
    "about alki dharma",
    "west seattle meditation group",
    "buddhist sangha seattle",
    "what is a sangha",
  ],
  openGraph: {
    title: "About | Alki Dharma Community",
    description:
      "Learn about the Alki Dharma Community, a Buddhist meditation group in West Seattle.",
  },
};

export default function About() {
  return (
    <div className="page-container">
      <img
        src="/banner-about.svg"
        alt="Alki Dharma Community banner"
        className="mb-8 h-auto w-full max-w-3xl rounded-2xl border border-zinc-200 shadow-sm dark:border-zinc-800"
      />
      <h1 className="mb-6 page-title">
        About our Sangha
      </h1>
      <div className="space-y-6 body-text">
        <p>
          The Alki Dharma Community is a Buddhist meditation group open to anyone, at any
          skill level. We meet weekly to practice meditation, hear selected readings, and
          discuss the Dharma with one another.
        </p>
        <h2 className="pt-2 section-title">
          Our Mission
        </h2>
        <p>
          Our group gathers together in the spirit of awareness and reflection,
          To celebrate the Dharma and build local community,
          We practice meditation, hear selected readings, and invite sharing on how this
          practice is meaningful in each of our lives.
        </p>
        <h2 className="pt-2 section-title">
          Meeting Format
        </h2>
        <p>
          The structure of our meetings is as follows:
        </p>
        <ul className="list-disc space-y-2 pl-6">
          <li>Introductions and welcome</li>
          <li>Mindfulness meditation (20 minutes)</li>
          <li>Selected Reading</li>
          <li>Discussion</li>
          <li>Closing, selection of volunteer for next week&apos;s reading</li>
          <li>Closing Meditation (5 minutes)</li>
        </ul>
        <p>
          Cushions, blankets, and chairs are provided, or you are welcome to bring your own.
        </p>
        <h2 className="pt-2 section-title">
          How to join
        </h2>
        <p>
          Just show up! Nothing is required to join, and you are welcome to come as frequently (or infrequently)
          as you like.
        </p>
        <h2 className="pt-2 section-title">
          Donations
        </h2>
        <p>
          All donations go directly to supporting our
          group&apos;s activities and to helping pay for the space we use to gather. 
          For Online donations, please follow{" "}
          <a
            href={DONATE_URL}
            className="inline-link"
            target="_blank"
            rel="noopener noreferrer"
          >
            this link
          </a>{" "}
          to support our gracious hosts at Alki UCC.
        </p>
        <h2 className="pt-2 section-title">
          Mailing List
        </h2>
        <p>
          We have a mailing list where we will ocassionally send out schedule updates,
          holiday schedules, etc. If you would like to be updated on the group&apos;s activities, 
          feel free to sign up on the{" "}
          <a
            href="/contact"
            className="inline-link"
          >
            contact page
          </a>
          .
        </p>
      </div>
    </div>
  );
}
