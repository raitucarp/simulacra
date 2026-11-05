# AGENTS.md - Core Project Guidelines for Simulacra

## 1. Absolute Rule: No Hardcoding & No Magic Strings
- **Zero Magic Strings & Magic Numbers**: Colors, font sizes, line heights, spacing, border widths, border radii, opacities, elevation levels, and key names must be strongly-typed enums, constants, or token fields.
- **Dynamic Theming via Theme Tokens**: All visual parameters (colors, typography scales, spacing tokens, radii, elevation, opacity, dialectic callout styles) must belong to the structured design token system (`ThemeTokens`).
- **Persistence to Local SQLite Vault**: All theme configurations and custom preferences must be serialized and persisted into the local SQLite database (`simulacra.db` via `settings` table), loaded upon startup, and synced on change.
- **No Direct Color Inlining in UI Components**: Never inline raw hex codes (`rgb(...)` / `rgba(...)`) or ad-hoc pixel values into organisms or molecules. Always reference `theme.<token>`.

## 2. Design System Foundations (Contemporary SoTA)
- **Colors**: Layered elevation surfaces (App background `#000000`, Card `#08080a`, Elevated Hover `#111115`, Hairline Border `#1e1e22`, Brand Violet `#8b5cf6`, Dialectic Gold `#eab308`, Emerald Active `#10b981`).
- **Typography Scale**:
  - Author Display Name: `font-semibold`, `text_sm` (14px), `text_primary`
  - Author Handle: `font-normal`, `text_xs` (12-13px), `text_muted`
  - Timestamp: `font-normal`, `text_xs` (12px), `text_muted` with middle dot `·`
  - Body Prose: `font-normal`, `text_sm` (14px), line height 1.45, `text_primary`
  - Epistemic / Dialectic Quotes: Indented callout card with `border_l_2` in accent color, tinted surface, distinct typographic lead.
  - Wikilink Pill: Interactive pill with preview text, no hash noise.
- **Spacing**: 4px base grid (`spacing_xs` = 4px, `spacing_sm` = 8px, `spacing_md` = 12px, `spacing_lg` = 16px, `spacing_xl` = 24px).
- **Radii**: Tokenized radii (`radius_sm` = 4px, `radius_md` = 8px, `radius_lg` = 12px, `radius_full` = 9999px).

## 3. Epistemic Architecture (Simulacra Core Purpose)
- Local-first epistemic dialectic simulator with autonomous agents (Socrates-GPT & Spinoza-Node).
- Continuous threadlines connecting parent posts to replies.
- Two-column split: Main Dialectic Stream (left) + Deep Focus Inspector (right).
