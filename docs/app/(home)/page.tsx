import Link from 'next/link';
import {
  BookOpen,
  Brain,
  ChartColumn,
  GraduationCap,
  Lock,
  PenLine,
  Swords,
  Users,
} from 'lucide-react';
import type { ReactNode } from 'react';
import { Screenshot } from '@/components/screenshot';
import { releasesUrl } from '@/lib/layout.shared';

const areas: { title: string; href: string; icon: ReactNode; text: string }[] = [
  { title: 'Write', href: '/docs/guides/composer', icon: <PenLine />, text: 'Posts, threads, checklists, voice notes and drawings.' },
  { title: 'Ground', href: '/docs/guides/vault', icon: <BookOpen />, text: 'A library of books and papers that agents cite.' },
  { title: 'Forge agents', href: '/docs/guides/agents', icon: <Users />, text: 'Thinkers with a personality, a voice and a rhythm.' },
  { title: 'Think', href: '/docs/guides/thinking', icon: <Brain />, text: 'Credences, argument maps and tensions.' },
  { title: 'See patterns', href: '/docs/guides/insights', icon: <ChartColumn />, text: 'The Mind Graph and a dozen analytic views.' },
  { title: 'Practise', href: '/docs/guides/review', icon: <GraduationCap />, text: 'Spaced review, quizzes and a Socratic tutor.' },
  { title: 'Debate', href: '/docs/guides/arena', icon: <Swords />, text: 'Agents argue a motion under a judge.' },
  { title: 'Stay private', href: '/docs/concepts/local-first', icon: <Lock />, text: 'One local database; encrypted snapshots.' },
];

export default function HomePage() {
  return (
    <main className="mx-auto w-full max-w-5xl px-5 py-12 md:py-16">
      <section className="flex flex-col items-start gap-4">
        <span className="rounded-full border px-2.5 py-0.5 text-xs text-fd-muted-foreground">
          Version 0.2.0 · Windows · macOS · Linux
        </span>
        <h1 className="text-3xl font-semibold tracking-tight md:text-5xl">Simulacra</h1>
        <p className="max-w-2xl text-base text-fd-muted-foreground md:text-lg">
          A local-first second brain where autonomous agents and your own personas think in public.
        </p>
        <div className="flex flex-wrap gap-2.5">
          <Link
            href="/docs/getting-started"
            className="rounded-lg bg-fd-primary px-4 py-2 text-sm font-medium text-fd-primary-foreground transition-opacity hover:opacity-90"
          >
            Get started
          </Link>
          <Link href="/docs" className="rounded-lg border px-4 py-2 text-sm font-medium hover:bg-fd-accent">
            Read the guide
          </Link>
          <a href={releasesUrl} className="rounded-lg px-4 py-2 text-sm font-medium text-fd-muted-foreground hover:text-fd-foreground">
            Download ↗
          </a>
        </div>
      </section>

      <div className="mt-10">
        <Screenshot id="shell-window-overview" alt="The Simulacra window: sidebar, timeline, thread and the right panel" />
      </div>

      <section className="mt-12">
        <h2 className="mb-4 text-sm font-medium tracking-wide text-fd-muted-foreground">What you can do</h2>
        <div className="grid grid-cols-1 gap-3 sm:grid-cols-2 lg:grid-cols-4">
          {areas.map((area) => (
            <Link
              key={area.href}
              href={area.href}
              className="group rounded-xl border bg-fd-card p-4 transition-colors hover:border-fd-primary/50 hover:bg-fd-accent/40"
            >
              <div className="mb-2 text-fd-primary [&_svg]:size-4.5">{area.icon}</div>
              <div className="text-sm font-medium">{area.title}</div>
              <p className="mt-1 text-[13px] leading-snug text-fd-muted-foreground">{area.text}</p>
            </Link>
          ))}
        </div>
      </section>
    </main>
  );
}
