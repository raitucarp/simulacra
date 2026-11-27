import defaultMdxComponents from 'fumadocs-ui/mdx';
import { Step, Steps } from 'fumadocs-ui/components/steps';
import { Tab, Tabs } from 'fumadocs-ui/components/tabs';
import { Accordion, Accordions } from 'fumadocs-ui/components/accordion';
import { File, Files, Folder } from 'fumadocs-ui/components/files';
import type { MDXComponents } from 'mdx/types';
import { Screenshot, ScreenshotGrid } from '@/components/screenshot';
import { Kbd, Shortcut } from '@/components/kbd';
import { UiLabel } from '@/components/ui-label';

export function getMDXComponents(components?: MDXComponents): MDXComponents {
  return {
    ...defaultMdxComponents,
    Step,
    Steps,
    Tab,
    Tabs,
    Accordion,
    Accordions,
    File,
    Files,
    Folder,
    Screenshot,
    ScreenshotGrid,
    Kbd,
    Shortcut,
    UiLabel,
    ...components,
  };
}

export const useMDXComponents = getMDXComponents;
