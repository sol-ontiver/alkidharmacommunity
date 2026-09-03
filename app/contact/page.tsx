import type { Metadata } from "next";

export const metadata: Metadata = {
  title: "Contact",
  description:
    "Contact the Alki Dharma Community, a Buddhist meditation group in West Seattle, to join our weekly sangha.",
  keywords: [
    "contact alki dharma",
    "join meditation group west seattle",
    "west seattle buddhist community",
  ],
  openGraph: {
    title: "Contact | Alki Dharma Community",
    description:
      "Get in touch with the Alki Dharma Community in West Seattle.",
  },
};

export default function Contact() {
  return (
    <div className="page-container">
      <h1 className="mb-6 page-title">
        Contact us
      </h1>
      <p className="mb-8 max-w-xl body-text">
        Please feel free to reach out if you would like more info, and we will get back to you
        as soon as possible.
      </p>

      <div className="mb-10 space-y-6 muted-text">
        <div>
          <h2 className="text-sm font-medium uppercase tracking-wide">
            Email
          </h2>
          <a
            href="mailto:alkidharmacommunity@gmail.com"
            className="list-title"
          >
            alkidharmacommunity@gmail.com
          </a>
        </div>
      </div>
    </div>
  );
}