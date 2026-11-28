# M1: menu architecture (post menu, context menus, every dropdown)

Status: decided (WS3 of the feature wave). Built on the shared menu model in
`src/ui/menus/`; the inventory test in `src/ui/features/post/controls/post_menu/tests/`
proves every action of the old post menu is still reachable.

## Need

Every menu was a hand-built kit `PopupMenu` with its own helper, two tick conventions
(the kit's tick, or an icon swapped for a check) and no right-click menu anywhere. The
post "…" menu reached about 70 actions three levels deep and mixed unrelated concerns:

- **Organize** held lifecycle (Archive, timers), a destructive timer (Delete
  automatically), task state, the garden, quote maintenance, series, privacy, edit history
  and collections.
- **Think it over** held up to 19 items: corrections, transcripts, distilling, the lens
  fan, translation, words to review, terms, three kinds of "question", changes of mind, the
  margin reply and the what-if fork.
- Three different "question" concepts lived in two submenus: Learn › Add question (a
  review card), Make it an open question (the post's state) and Ask the room (asking the
  agents).

## Card sort

The inventory's leaves were sorted by the question a reader has in mind when they open
the menu. The groups that came out, in the order a reader meets them:

| # | Section | The reader is thinking | What it holds |
| - | ------- | ---------------------- | ------------- |
| 1 | Respond | "I want to answer this" | Reply, Quote, Reply in the margin… |
| 2 | Think with AI | "Help me think about it" | the 21 actions by group (Understand ▸, Challenge ▸, Explore ▸), Find similar, Fork a what-if… |
| 3 | Study | "I want to learn or examine it" | review cards and words to review; the post as an open question, asking the room, answering a question; terms and confidence |
| 4 | Organize | "Where should it live?" | bookmark, collections, series, garden, entry state, re-pinning a quote, distilling to the vault; archive and its timer; delete on a timer |
| 5 | View | "Show it another way" | Show in ‹language› / Show original, Lens fan…, Transcribe voice note |
| 6 | Share & copy | "Take it elsewhere" | copy text, link, embed; who can read it |
| 7 | Edit & history | "Change it, or see how it changed" | Edit, History (N), Mark as corrected… or taking a mark back, Changed my mind… |
| 8 | Visibility & lifecycle | "Who sees it, how long it lives" | who can read it (in Share & copy), Archive and Archive automatically (in Organize) |
| 9 | Danger zone | "End it" | Retract…, Delete automatically ▸, Delete — always last in its menu, always red |

Sections 8 and 9 are folded into the submenus where readers look for them, to keep the top
level near eight entries: *who can read this* sits with sharing (as in a document's Share
dialog), *archive* with organizing (as in a mail client), and each destructive item closes
the submenu of its concept in a "Danger zone" section (Retract… under Edit & history,
Delete automatically ▸ under Organize). Delete itself stays at the top level, last and red.

The three question concepts are merged into **Study › Questions**: making the post an open
question (and marking it answered, reopening it, or making it a thought again), asking the
room, and "This answers my question". The review card that a question was confused with
now says what it is: **Study › Review › Add a review question…** / **Add a review
answer…**. Translation, the lens fan and the transcript form **View**; the margin reply
joins **Respond**; "Send words to review…" joins **Study › Review**.

## Rules

- At most about eight top-level entries: someone else's post shows eight; a post the user
  wrote, with every feature on, shows ten (the hard limit, `rules::MAX_TOP_LEVEL`).
- A submenu holds at most ten entries, or labelled sections of at most ten each
  (`rules::MAX_SUBMENU`); at most three levels (`rules::MAX_DEPTH`). A picker whose whole
  top level is labelled sections (the view menu: Layouts, Lenses, Collections) may list more
  than the top-level budget the same way.
- No concept twice: an action id lives in exactly one menu.
- No duplicate labels within a menu.
- One tick convention: the kit's `checked`, drawn on the right (`check_side(Side::Right)`),
  so a ticked item keeps its own icon. Every menu renders through `render_menu`, which sets
  it.
- Destructive entries are red (label and icon; a submenu's icon) and sit in the last
  section of their menu, after every other entry of it. Every confirmation dialog stays
  (Delete, Retract…, deleting a document, a profile, a journal entry, a review item).
- Items that do not apply are left out, as before. The model can also show an item greyed
  out with a reason (`MenuItem::disabled_because`) where hiding it would confuse.
- Labels come from locale keys; the structure tests check every key in all ten locales.
- Stored keys never change (`PostView`, `CommandId` wire keys); new menu labels get new keys.

## The shared model (`src/ui/menus/`)

`MenuSpec` (sections, each optionally labelled) of `MenuEntry` (`MenuItem` or `Submenu`).
An item has a stable id, a `MenuText` label (text plus the locale key it came from), an
icon, an optional `Action` (its shortcut is shown; registry commands use `RunCommand`), a
tick, a disabled reason, destructive styling, a muted detail line and a click handler.
Builders add only what applies (the visibility predicates). `render_menu` is the one
renderer to the kit's `PopupMenu`; `context_menu_of` hands a spec to
`ContextMenuExt::context_menu`; `OpenMenu` hosts a menu the shell opens from state (the
network menu, which a command also opens). `rules::violations` and `rules::label_keys`
check a spec; `MenuSpec::path_of` / `items` / `menus` walk it.

## Menus now

### Post menu ("…" and right-click on a card: timeline, inspector root and replies)

```
Reply
Quote
Reply in the margin…                        (not on a locked post)
─────
Think with AI ▸
    Understand ▸   Search a concept · Explain for… · Summarise the thread · Map to concepts · Translate · Map the argument · Check atomicity
    Challenge ▸    Steelman · Find strawmen · Find the hidden premise · Counterexamples · Spot cognitive biases · Rate claim confidence ·
                   Find the crux · Find tensions with this post · Debate this
    Explore ▸      Socratic questions · Analogies · Make it falsifiable · Continue the thought · Run a thought experiment on this
    ─────
    Find similar
    Fork a what-if…                         (not locked)
Study ▸
    [Review]       Add a review question… · Add a review answer…  |  Edit review · Stop learning (in review)
                   Learn this thread's connections
                   Send words to review…   (once translated)
    [Questions]    Make it an open question | Mark answered · Not a question any more | Reopen the question   (yours)
                   Ask the room             (yours, once)
                   This answers my question (a reply to your open question)
    [Terms and confidence]
                   Define a term… · Stop defining “…”   (yours)
                   Set a credence…          (yours, none yet)
Organize ▸
    [Keep and sort]  Bookmark / Remove bookmark
                     Add to collection ▸  ‹each collection, ticked when it holds the post› · New collection…
                     Add to a series ▸ ‹each series› · New series…  |  Leave “…”   (yours)
                     Plant in the garden  |  Garden: ‹stage› ▸ Grow to … · Back to … · Return to the stream   (yours)
                     Entry ▸ ‹each allowed move›   (your task or event)
                     Re-pin the quote to its source   (yours, drifted)
                     Distill thread to vault · Distill to vault with a summary
    [Lifecycle]      Archive / Unarchive · Archive automatically ▸ ‹ticked choice›   (yours)
    [Danger zone]    Delete automatically ▸ Never ✓ · In an hour · In a day · In a week · In 30 days   (yours, red)
View ▸
    Show in ‹language› / Show original      (not locked)
    Lens fan…                               (not locked)
    Transcribe voice note                   (yours, untranscribed)
─────
Share & copy ▸
    Copy text · Copy link · Copy embed
    ─────
    Who can read this ▸ Everyone ✓ · Only me · ‹each agent›   (yours)
Edit & history ▸
    Edit (yours) · History (N) (once edited) · Mark as corrected… / Take back the … (yours, not locked)
    Changed my mind… · Not a change of mind (yours, not locked)
    [Danger zone]  Retract…   (yours, not locked, red)
─────
Delete                                      (yours, red, asks first)
```

### Other menus

```
New (sidebar)      [Write] New thread
                   [Capture] New note · New voice note · New drawing
                   [Import] Import file
                   [Create] New agent · More ▸ New journal entry · Log mood · New time capsule ·
                            New debate · New thought experiment · New connection map · Create network
                   (every entry a registry command, with its shortcut)
View (inspector, profile)
                   [Layouts] Show as cards · an outline · an argument map · a rapid log · raw text ·
                             a calendar · a future log · a board
                   [Lenses] Show as what-ifs side by side · lens fans · claims to recheck ·
                            open questions · changes of mind
                   [Collections] Show as a garden · series · a glossary        (active view ticked)
References         [Refers to] ‹quotes, embeds, links› (or "No quoted or linked thoughts")
                   [Referred to by N thoughts] ‹backlinks›
                   ───── Open conversation (N replies)                            (scrolls)
Composer More      Add to thread ───── Attach link ✓ (while its field is open) · Pick image · Insert hashtag (#)
Vault sort         [Sort by] Relevance · Title · Date added · Date published · Most linked (ticked)
                   [Order] Ascending · Descending (ticked); the button's arrow shows the direction
Palette menus      [Built-in] ‹bundled palettes› [Custom] ‹yours›                  (scrolls)
Settings choices   one option per row, ticked on the right                         (scrolls)
Language           [Select language] ‹ten languages, native names, current ticked›  (keyboard, scrolls)
Network            [Switch network] ‹each network with “N docs · M agents”, active ticked›
                   ───── Create network                                            (keyboard, scrolls)
App menu (☰, title bar; every entry a registry command with its shortcut)
                   File ▸      New thread · New note · New voice note · New drawing · More ▸ (New agent,
                               Create network, New journal entry, New time capsule, New debate,
                               New thought experiment, New connection map) ─ Import file ─
                               Export bundle · Encrypted snapshot · Restore snapshot
                   Post ▸      the commands on the open thought (actions, find similar, learn,
                               bookmark, pin, debate, thought experiment)
                   View ▸      sidebars ─ outline · argument map ─ appearance
                   Go ▸        [Places] the ten screens [Lists] inbox · saved · scheduled · open questions
                               [Switch] network · persona
                   Tools ▸     [Search] search · by meaning · command palette [Thinking] … [Practice] …
                   Insights ▸  [The network] … [Your thinking] …
                   ─────
                   Help ▸      Keyboard shortcuts · Replay welcome · Settings
```

The app menu uses the shared model in a title-bar dropdown on every platform rather than
the kit's `AppMenuBar` (a horizontal bar) or `cx.set_menus` (macOS): one ☰ button keeps
the frameless title bar uncluttered, and the same spec could feed a native macOS menu
later. Its "Edit" menu is named **Post**, since every entry acts on the open thought (text
editing lives in the composer's own right-click menu). There is no Developer menu yet:
there are no developer commands.

### Right-click menus (same model)

| Where | Entries (each the existing handler) |
| ----- | ----------------------------------- |
| Post card (timeline, inspector root and replies) | the post menu above |
| Vault document (shelf card, table row) | Open · Read ─ Edit · Mark as read/unread · History ─ Learn passages · Learn connections · Quote a passage ─ Delete (asks first) |
| Saved shelf chip (Bookmarks, a collection) | Open · Learn this collection |
| Saved shelf row | Open · Edit note (collections) ─ Remove from this shelf |
| Sidebar place (Timeline, Knowledge, …) | Go to ‹place› ─ its jump list of registry commands (e.g. Journal: New journal entry · Log mood · Start focus · New time capsule) |
| Sidebar pinned, recent, source or member row | Open · Pin / Unpin |
| Roster agent or persona | Open · Edit in Forge · Write a post… (agent) / Switch to Persona ─ Remove profile (asks first) |
| Journal entry | Edit · Turn into a post ─ Delete (asks first) |
| Review item | Edit · Suspend / Resume (not retired) ─ Stop learning (asks first) |
| Task (Tasks page) | Mark done / Reopen · Open the post |
| Mind Graph canvas | Open ‹the node under the pointer› ─ Reset view |
| Composer text | Cut · Copy (with a selection) · Paste ─ Select all ─ Format ▸ Bold · Italic · Underline · Code · Spoiler |

### Command palette

Commands are listed under category headings (`CommandCategory`, seeded from the shortcut
sheet's groups): Navigate, Create, Open post, Thinking, Practice, Insights, Settings. With
an empty query the categories follow that order; while searching they follow their best
result. Docs, Developer and Journey are added with their first command. Wire keys are
unchanged.

## Old path → new path (post menu)

Every leaf of the old menu, with its stable id. The inventory test
(`every_action_of_the_old_menu_is_reachable_where_it_was`) checks each one on a card where
the old menu offered it, and prints this mapping with `--nocapture`.

| Old path | New path | Id |
| -------- | -------- | -- |
| Actions › ‹Understand action› | Think with AI › Understand › ‹action› | `post.ai.<key>` |
| Actions › ‹Challenge action› | Think with AI › Challenge › ‹action› | `post.ai.<key>` |
| Actions › ‹Explore action› | Think with AI › Explore › ‹action› | `post.ai.<key>` |
| Reply | Reply | `post.reply` |
| Quote | Quote | `post.quote` |
| Find similar | Think with AI › Find similar | `post.find_similar` |
| Copy › Copy text | Share & copy › Copy text | `post.copy.text` |
| Copy › Copy link | Share & copy › Copy link | `post.copy.link` |
| Copy › Copy embed | Share & copy › Copy embed | `post.copy.embed` |
| Learn › Add question | Study › Add a review question… | `post.learn.add_question` |
| Learn › Add answer | Study › Add a review answer… | `post.learn.add_answer` |
| Learn › Edit review | Study › Edit review | `post.learn.edit` |
| Learn › Stop learning | Study › Stop learning | `post.learn.stop` |
| Learn › Learn this thread's connections | Study › Learn this thread's connections | `post.learn.connections` |
| Set a credence… | Study › Set a credence… | `post.credence.set` |
| Edit | Edit & history › Edit | `post.edit` |
| Bookmark / Remove bookmark | Organize › Bookmark / Remove bookmark | `post.bookmark`, `post.unbookmark` |
| Organize › Archive / Unarchive | Organize › Archive / Unarchive | `post.archive`, `post.unarchive` |
| Organize › Archive automatically › ‹choice› | Organize › Archive automatically › ‹choice› | `post.auto_archive.<key>` |
| Organize › Delete automatically › Never / ‹delay› | Organize › Delete automatically › Never / ‹delay› (Danger zone) | `post.auto_delete.never`, `post.auto_delete.<key>` |
| Organize › Entry › ‹move› | Organize › Entry › ‹move› | `post.entry.<state>` |
| Organize › Plant in the garden | Organize › Plant in the garden | `post.garden.plant` |
| Organize › Garden: ‹stage› › Grow to / Back to / Return to the stream | Organize › Garden: ‹stage› › Grow to / Back to / Return to the stream | `post.garden.grow`, `.cut`, `.return` |
| Organize › Re-pin the quote to its source | Organize › Re-pin the quote to its source | `post.quote.repin` |
| Organize › Add to a series › ‹series› / New series… | Organize › Add to a series › ‹series› / New series… | `post.series.add`, `post.series.new` |
| Organize › Leave “…” | Organize › Leave “…” | `post.series.leave` |
| Organize › Who can read this › Everyone / Only me / ‹agent› | Share & copy › Who can read this › Everyone / Only me / ‹agent› | `post.audience.everyone`, `.only_me`, `.agent` |
| Organize › History (N) | Edit & history › History (N) | `post.history` |
| Organize › Add to collection › ‹collection› / New collection… | Organize › Add to collection › ‹collection› / New collection… | `post.collection.toggle`, `post.collection.new` |
| Think it over › Transcribe voice note | View › Transcribe voice note | `post.voice.transcribe` |
| Think it over › Retract… | Edit & history › Retract… (Danger zone) | `post.correction.retract` |
| Think it over › Mark as corrected… | Edit & history › Mark as corrected… | `post.correction.corrected` |
| Think it over › Take back the retraction / correction | Edit & history › Take back the retraction / correction | `post.correction.withdraw` |
| Think it over › Distill thread to vault | Organize › Distill thread to vault | `post.distill.thread` |
| Think it over › Distill to vault with a summary | Organize › Distill to vault with a summary | `post.distill.summary` |
| Think it over › Lens fan… | View › Lens fan… | `post.lens` |
| Think it over › Show in ‹language› / Show original | View › Show in ‹language› / Show original | `post.translation.show`, `.original` |
| Think it over › Send words to review… | Study › Send words to review… | `post.translation.words` |
| Think it over › This answers my question | Study › This answers my question | `post.question.answers` |
| Think it over › Define a term… / Stop defining “…” | Study › Define a term… / Stop defining “…” | `post.glossary.define`, `.undefine` |
| Think it over › Make it an open question | Study › Make it an open question | `post.question.ask` |
| Think it over › Mark answered / Not a question any more | Study › Mark answered / Not a question any more | `post.question.resolve`, `.unask` |
| Think it over › Reopen the question | Study › Reopen the question | `post.question.reopen` |
| Think it over › Changed my mind… / Not a change of mind | Edit & history › Changed my mind… / Not a change of mind | `post.mind.open`, `.unmark` |
| Think it over › Ask the room | Study › Ask the room | `post.question.ask_room` |
| Think it over › Reply in the margin… | Reply in the margin… | `post.margin` |
| Think it over › Fork a what-if… | Think with AI › Fork a what-if… | `post.branch` |
| Delete | Delete | `post.delete` |

Docs pages that quote "… › Organize › …" or "… › Think it over › …" paths are updated from
this table in the documentation workstream.

## Other changes worth knowing

- **Language and network menus** are kit popup menus now: arrow keys, Enter, Escape and
  scrolling, the current choice ticked. The network menu opens from the switcher or the
  "Switch network" command (`OpenMenu`); the dead `WorkspaceTrigger` is gone.
- **References**: Outgoing and Incoming sections, scrolling past the kit's height; the
  backlinks count is a checked plural (`post.refs_backlinks` joins the count-label list).
- **Composer More**: "Attach link" is ticked while its field is open; "Pick image" is
  sentence case.
- **Vault sort**: an Order section shows and sets the direction; choosing the current key
  again still reverses it, as before.
