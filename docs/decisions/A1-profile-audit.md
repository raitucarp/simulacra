# A1: Persona and agent profile audit

Status: decided (issue #291, epic A *Profiles and manual posting*). Acted on by #292 (A2);
the trigger rows are taken up by #294 (A4) and #295 (A5).

The Personas & Agents page shows a roster (left) and the selected profile's overview
(right). This note lists every place where a persona's page and an agent's page differ, as
the code stood after epic W, and says for each whether it is a real difference of kind (kept,
with an equivalent) or an accident of history (removed).

Code read: `src/ui/features/roster/` (`overview.rs`, `profile_tabs.rs`, `posts_tab.rs`,
`recommended.rs`, `empty.rs`, `contact_row.rs`, `mode_switch.rs`, `pane.rs`), the shell
wiring in `src/ui/features/shell/render/workspace.rs` and `handlers/lab.rs`, and the
Beliefs view in `src/ui/features/beliefs/`.

## Tabs

| Tab | Agent | Persona | Verdict |
| --- | --- | --- | --- |
| Profile (card) | yes | yes | Same. |
| Posts (W7: timeline with the view switcher, feelings filter) | yes | yes | Same. |
| Beliefs | yes: the agent's beliefs and their revision log | **missing**: the pill is not drawn, and a stored Beliefs tab falls back to the card | Accident. A persona speaks for the user and has no belief book, but it has an equivalent: the credences the user stated on its posts (sb E1). A2 shows the tab for both; a persona's tab lists those credences, or an empty state explaining where they come from. |
| Recommended | yes | yes | Same (one empty text for both). |

## Buttons

| Place | Agent | Persona | Verdict |
| --- | --- | --- | --- |
| Edit in Forge | yes | yes | Same. |
| Primary action | "Synthesize post" (runs one reply at once, no options, no feedback) | "Use persona" (makes it the posting identity) | Kind: an agent writes on its own, a persona is written as. Kept, in the same slot. A4 turns the agent's button into "Write a post…" opening the trigger dialog (new post or reply, topic or vault document, estimated cost). |
| Delete | yes (ghost, tooltip) | yes | Same. |
| Run status and cancel | none: a run gives no sign until its post lands | n/a | Gap, closed by A5 (queued, writing, done, with cancel). |

## Identity and markers

| Place | Agent | Persona | Verdict |
| --- | --- | --- | --- |
| MBTI tag colour | agent accent | user accent | Kind (the two accents name the two roles everywhere). Kept. |
| Badge under the name | none | "Active persona" when in use | Kind. Kept. |
| Contact row marker | dot while the agent is active | glyph while the persona is in use | Kind. Kept. |
| Occupation, place, bio | yes | yes | Same. |

## Empty states

| Place | Agent | Persona | Verdict |
| --- | --- | --- | --- |
| Empty roster | "No agents yet" + art + "New agent" | "No personas yet" + art + "New persona" | Same layout, own words. Kept. |
| Beliefs tab | text inside the Beliefs view | none (tab hidden) | Accident; see Tabs. |
| Posts tab | "No posts yet" | same | Same. |
| Recommended tab | one text for both | same | Same. |

## What A2 changes

1. One tab row for both: Profile, Posts, Beliefs, Recommended.
2. A persona's Beliefs tab: its stated credences (newest first, with their chance and
   whether they are open or how they resolved), each opening its post; without any, an
   empty state that says personas do not form beliefs on their own and how to state one.
3. The primary action keeps its slot and outline style for both kinds.
