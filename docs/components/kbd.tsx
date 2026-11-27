import type { ReactNode } from 'react';

/** One key cap. */
export function Kbd({ children }: { children: ReactNode }) {
  return (
    <kbd className="inline-flex min-w-[1.6em] items-center justify-center rounded-md border border-b-2 bg-fd-card px-1.5 py-px font-mono text-[0.78em] font-medium leading-normal text-fd-foreground">
      {children}
    </kbd>
  );
}

/**
 * A chord such as `Ctrl+Shift+P`. On macOS Simulacra uses ⌘ where Windows and Linux use Ctrl;
 * write the Windows form and set `mac` to show both.
 */
export function Shortcut({ keys, mac = true }: { keys: string; mac?: boolean }) {
  const parts = keys.split('+');
  const usesCtrl = parts[0] === 'Ctrl';
  const render = (list: string[]) => (
    <span className="inline-flex items-center gap-0.5 align-baseline">
      {list.map((key, at) => (
        <Kbd key={`${key}-${at}`}>{key}</Kbd>
      ))}
    </span>
  );
  if (!mac || !usesCtrl) return render(parts);
  return (
    <span className="inline-flex flex-wrap items-center gap-1 align-baseline">
      {render(parts)}
      <span className="text-[0.8em] text-fd-muted-foreground">/</span>
      {render(['⌘', ...parts.slice(1)])}
    </span>
  );
}
