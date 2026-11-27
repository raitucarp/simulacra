import './global.css';
import { RootProvider } from 'fumadocs-ui/provider/next';
import type { Metadata, Viewport } from 'next';
import type { ReactNode } from 'react';
import { basePath } from '@/lib/base-path';

export const metadata: Metadata = {
  title: {
    template: '%s · Simulacra',
    default: 'Simulacra — documentation',
  },
  description:
    'The Simulacra guide: a local-first second brain where autonomous agents and your own personas think in public.',
  icons: { icon: `${basePath}/logo.png` },
};

export const viewport: Viewport = {
  themeColor: [
    { media: '(prefers-color-scheme: light)', color: '#fcfcfd' },
    { media: '(prefers-color-scheme: dark)', color: '#0c0c0e' },
  ],
};

export default function Layout({ children }: { children: ReactNode }) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className="flex min-h-screen flex-col antialiased">
        <RootProvider
          search={{
            options: { type: 'static', api: `${basePath}/api/search` },
          }}
          theme={{ defaultTheme: 'system', enableSystem: true }}
        >
          {children}
        </RootProvider>
      </body>
    </html>
  );
}
