import type { ReactNode } from 'react';

/** A label exactly as it appears in the app (a button, menu item or field). */
export function UiLabel({ children }: { children: ReactNode }) {
  return (
    <span className="rounded-[5px] border border-fd-border/70 bg-fd-secondary/60 px-1.5 py-px text-[0.88em] font-medium whitespace-nowrap text-fd-foreground">
      {children}
    </span>
  );
}
