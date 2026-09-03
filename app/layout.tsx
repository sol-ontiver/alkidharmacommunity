import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import Link from "next/link";
import { SITE_URL, ORGANIZATION } from "@/lib/constants";
import "./globals.css";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  metadataBase: new URL(SITE_URL),
  title: {
    default: "Alki Dharma Community | Buddhist Meditation in West Seattle",
    template: "%s | Alki Dharma Community",
  },
  description:
    "A Buddhist meditation group in West Seattle (Alki, Seattle, WA). Join us weekly for mindfulness meditation, dharma reading, and discussion. All are welcome regardless of faith or experience.",
  keywords: [
    "buddhism",
    "buddhist",
    "meditation",
    "mindfulness",
    "dharma",
    "sangha",
    "spiritual community",
    "spirituality",
    "west seattle",
    "alki",
    "seattle washington",
    "meditation group seattle",
    "buddhist meditation west seattle",
  ],
  openGraph: {
    type: "website",
    locale: "en_US",
    url: SITE_URL,
    siteName: "Alki Dharma Community",
    title: "Alki Dharma Community | Buddhist Meditation in West Seattle",
    description:
      "A welcoming Buddhist meditation group in West Seattle. Join us weekly for meditation, dharma reading, and discussion.",
  },
  robots: {
    index: true,
    follow: true,
  },
  alternates: {
    canonical: "/",
  },
};

const jsonLd = {
  "@context": "https://schema.org",
  "@type": "Organization",
  name: ORGANIZATION.name,
  url: SITE_URL,
  description: ORGANIZATION.description,
  address: {
    "@type": "PostalAddress",
    streetAddress: ORGANIZATION.streetAddress,
    addressLocality: "Seattle",
    addressRegion: "WA",
    postalCode: ORGANIZATION.postalCode,
    addressCountry: "US",
  },
  areaServed: ["Seattle", "West Seattle", "Alki", "Puget Sound"],
  knowsAbout: [
    "Buddhism",
    "Meditation",
    "Mindfulness",
    "Dharma",
    "Spirituality",
  ],
};

const navLinks = [
  { href: "/", label: "Home" },
  { href: "/about", label: "About" },
  { href: "/resources", label: "Resources" },
  { href: "/events", label: "Events" },
  { href: "/blog", label: "Blog" },
  { href: "/contact", label: "Contact" },
];

export default function RootLayout({ children }: LayoutProps<"/">) {
  return (
    <html
      lang="en"
      className={`${geistSans.variable} ${geistMono.variable} h-full antialiased`}
    >
      <body className="min-h-full flex flex-col">
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{ __html: JSON.stringify(jsonLd) }}
        />
        <header className="border-b border-zinc-200 dark:border-zinc-800">
          <nav className="mx-auto flex w-full max-w-3xl items-center justify-between gap-4 px-4 py-4">
            <Link
              href="/"
              className="font-semibold tracking-tight text-foreground"
            >
              Alki Dharma
            </Link>
            <ul className="flex items-center gap-4 text-sm">
              {navLinks.map((link) => (
                <li key={link.href}>
                  <Link
                    href={link.href}
                    className="nav-link"
                  >
                    {link.label}
                  </Link>
                </li>
              ))}
            </ul>
          </nav>
        </header>
        <main className="flex-1 w-full">{children}</main>
        <footer className="border-t border-zinc-200 py-8 dark:border-zinc-800">
          <div className="mx-auto w-full max-w-3xl px-4 text-sm muted-text">
            Alki Dharma Community · West Seattle
          </div>
        </footer>
      </body>
    </html>
  );
}