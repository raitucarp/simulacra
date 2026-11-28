import type { ReactNode } from 'react';
import { ImageZoom } from 'fumadocs-ui/components/image-zoom';
import { withBasePath } from '@/lib/base-path';
import manifest from './screenshots.json';

/** One captured image: its size and whether a light-theme capture exists beside the dark one. */
interface ShotInfo {
  width: number;
  height: number;
  light?: boolean;
}

const shots = manifest as Record<string, ShotInfo>;

type Size = 'full' | 'md' | 'sm';

const sizeClass: Record<Size, string> = {
  full: 'w-full',
  md: 'w-full max-w-[560px]',
  sm: 'w-full max-w-[360px]',
};

interface ScreenshotProps {
  /** The capture id (`capture/shots/*.json`), served from `/screenshots/<id>.webp`. */
  id: string;
  /** What the image shows, for screen readers and when it cannot load. */
  alt: string;
  caption?: ReactNode;
  /** `full` for whole windows, `md` for panels and dialogs, `sm` for single controls. */
  size?: Size;
}

/** A framed, zoomable capture of the app, with a light-theme twin when one was captured. */
export function Screenshot({ id, alt, caption, size }: ScreenshotProps) {
  const info = shots[id];
  const resolved: Size = size ?? (info && info.width < 700 ? (info.width < 420 ? 'sm' : 'md') : 'full');

  return (
    <figure className={`not-prose my-6 ${sizeClass[resolved]}`}>
      <div className="overflow-hidden rounded-xl border bg-fd-muted shadow-sm ring-1 ring-black/5 dark:ring-white/5">
        {info ? (
          <>
            <ImageZoom
              src={withBasePath(`/screenshots/${id}.webp`)}
              alt={alt}
              width={info.width}
              height={info.height}
              className={`block h-auto w-full ${info.light ? 'hidden dark:block' : ''}`}
            />
            {info.light ? (
              <ImageZoom
                src={withBasePath(`/screenshots/light/${id}.webp`)}
                alt={alt}
                width={info.width}
                height={info.height}
                className="block h-auto w-full dark:hidden"
              />
            ) : null}
          </>
        ) : (
          <div className="flex aspect-[16/10] items-center justify-center p-6 text-center text-sm text-fd-muted-foreground">
            {alt}
          </div>
        )}
      </div>
      {caption ? (
        <figcaption className="mt-2 text-[13px] leading-snug text-fd-muted-foreground">{caption}</figcaption>
      ) : null}
    </figure>
  );
}

/** Two or three small captures side by side (stacked on phones). */
export function ScreenshotGrid({ children, cols = 2 }: { children: ReactNode; cols?: 2 | 3 }) {
  return (
    <div className={`not-prose my-6 grid grid-cols-1 gap-4 ${cols === 3 ? 'sm:grid-cols-3' : 'sm:grid-cols-2'} [&>figure]:my-0`}>
      {children}
    </div>
  );
}
