import type { BaseLayoutProps } from 'fumadocs-ui/layouts/shared';
import { withBasePath } from './base-path';

export const repoUrl = 'https://github.com/raitucarp/simulacra';
export const releasesUrl = `${repoUrl}/releases`;

export function baseOptions(): BaseLayoutProps {
  return {
    nav: {
      title: (
        <span className="flex items-center gap-2 font-semibold tracking-tight">
          {/* eslint-disable-next-line @next/next/no-img-element */}
          <img src={withBasePath('/logo.png')} alt="" width={22} height={22} className="rounded-md" />
          Simulacra
        </span>
      ),
    },
    githubUrl: repoUrl,
    links: [
      { text: 'Guide', url: '/docs', active: 'nested-url' },
      { text: 'Download', url: releasesUrl, external: true },
    ],
  };
}
