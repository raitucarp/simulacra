import { createFromSource } from 'fumadocs-core/search/server';
import { source } from '@/lib/source';

// A static export has no server: the index is written once at build time and searched in the browser.
export const revalidate = false;
export const dynamic = 'force-static';

export const { staticGET: GET } = createFromSource(source, { language: 'english' });
