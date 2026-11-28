"""Adds the demo content and settings to a freshly seeded Simulacra database (app closed).

    python docs/capture/demo/seed.py <path to simulacra.db>

- posts.json: a few months of posts, replies and checklists, dated relative to today;
- the AI engine and the embedding engine point at the local stub (stub-ai.mjs) with a
  placeholder key, so AI features run offline;
- the welcome is reset so it shows on the next start.
"""
import json
import sqlite3
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path

HERE = Path(__file__).resolve().parent
STUB = "http://127.0.0.1:8765/v1"
PLACEHOLDER_KEY = "local-stub-not-a-secret"


def stamp(when: datetime) -> str:
    return when.astimezone(timezone.utc).isoformat(timespec="microseconds")


def seed_posts(db: sqlite3.Connection) -> None:
    posts = json.loads((HERE / "posts.json").read_text(encoding="utf-8"))
    now = datetime.now().astimezone()
    today = now.replace(minute=12, second=0, microsecond=0)
    for post in posts:
        if post["hour"] < 0:
            when = now + timedelta(hours=post["hour"])
        else:
            when = (today - timedelta(days=post["days"])).replace(hour=post["hour"], minute=(len(post["text"]) * 7) % 60)
        db.execute(
            "INSERT OR REPLACE INTO posts (id, network_id, author_id, author_json, content, created_at, likes, reply_to_id)"
            " VALUES (?, ?, ?, '{}', ?, ?, ?, ?)",
            (post["id"], post["net"], post["author"], post["text"], stamp(when), post.get("likes", 0), post.get("reply")),
        )
    db.execute(
        "UPDATE posts SET replies_count = (SELECT COUNT(*) FROM posts AS r WHERE r.reply_to_id = posts.id)"
    )


def set_setting(db: sqlite3.Connection, key: str, value: str) -> None:
    db.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", (key, value))


def merge_pref(db: sqlite3.Connection, key: str, **changes) -> None:
    row = db.execute("SELECT value FROM settings WHERE key = ?", (key,)).fetchone()
    value = json.loads(row[0]) if row else {}
    value.update(changes)
    set_setting(db, key, json.dumps(value))


def seed_engines(db: sqlite3.Connection) -> None:
    set_setting(db, "llm_provider", "custom")
    set_setting(db, "llm_model", "stub-thinker")
    set_setting(db, "llm_endpoint", f"{STUB}/chat/completions")
    set_setting(db, "llm_api_key", PLACEHOLDER_KEY)
    credential = {
        "id": "cred_localstub",
        "kind": "custom",
        "label": "Local engine",
        "secret": PLACEHOLDER_KEY,
        "endpoint": f"{STUB}/chat/completions",
    }
    set_setting(db, "llm.credentials", json.dumps([credential]))
    merge_pref(db, "preferences.embeddings", provider="custom", model="stub-embed", endpoint=STUB, dimensions=256)


def main() -> None:
    db = sqlite3.connect(sys.argv[1])
    with db:
        seed_posts(db)
        seed_engines(db)
        set_setting(db, "active_network_id", "net_philosophy")
        set_setting(db, "onboarding.done", "false")
        seed_beliefs(db)
        seed_practice(db, Path(sys.argv[1]).resolve().parent)
    db.close()
    print("demo content added")


# --- appended: agent beliefs with a revision history ---------------------------------------
# The stub engine answers belief updates with prose, so no belief ever forms through the UI in
# the demo profile. Sokrates of Alopece gets a few beliefs with their history (formed, then
# revised on later days) taken from his posts and threads, for the Beliefs tab captures. Simone
# Weil and Baruch de Spinoza keep none (their empty Beliefs tabs are captured too).

def label_of(text: str) -> str:
    flat = " ".join(text.split())
    return flat if len(flat) <= 60 else flat[:60] + "…"


def utc(when: datetime) -> str:
    return when.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def seed_beliefs(db: sqlite3.Connection) -> None:
    agent = "agent_socrates"
    now = datetime.now(timezone.utc).replace(second=0, microsecond=0)

    def content(post_id: str) -> str:
        row = db.execute("SELECT content FROM posts WHERE id = ?", (post_id,)).fetchone()
        return row[0] if row else ""

    def source(origin: str, post_id: str, label_from: str) -> dict:
        return {"origin": origin, "id": post_id, "label": label_of(content(label_from))}

    # (id, proposition, source, [(days ago, credence, reason)], pinned): oldest change first.
    beliefs = [
        ("demo_belief_mirror", "A reflection can reproduce a thing without sharing in what it is.",
         source("thread", "post_1", "post_1"),
         [(12, 70, "The mirror that shows the sun without its warmth seems to settle it."),
          (4, 60, "Arya asks whether predicting symbols is already a way of grasping them; the line is less sharp than I said.")], False),
        ("demo_belief_use", "Using a word well is not the same as knowing what it means.",
         source("post", "demo_p05", "demo_p05"),
         [(97, 80, "A lyre player can play well and still not say what music is."),
          (95, 74, "Arya's reply: know-how may be a knowing of its own, even if it cannot be said.")], False),
        ("demo_belief_virtue", "Knowing what a virtue is does not make a person virtuous.",
         source("post", "demo_p24", "demo_p24"),
         [(30, 55, "Those who define courage best are rarely the bravest."),
          (26, 68, "Arya adds that virtue is a trained habit, which supports it.")], False),
        ("demo_belief_survive", "No one can tell in advance which beliefs would survive reasons not yet met.",
         source("reply", "demo_p31", "demo_p30"),
         [(8, 82, "Only meeting the reasons shows which beliefs survive them.")], False),
        ("demo_belief_recollect", "Learning is, at least in part, a kind of recollection.",
         source("reply", "demo_p12", "demo_p11"),
         [(75, 55, "Meno's slave boy found the diagonal without being told it.")], False),
    ]
    for belief_id, proposition, src, changes, pinned in beliefs:
        old = None
        for days, credence, reason in changes:
            at = utc(now - timedelta(days=days, hours=3))
            revision = {"belief": belief_id, "agent": agent, "proposition": proposition, "old": old, "new": credence,
                        "cause": "evidence", "reason": reason, "source": src, "at": at}
            db.execute("INSERT INTO belief_revisions (belief_id, agent_id, at, record_json) VALUES (?, ?, ?, ?)",
                       (belief_id, agent, at, json.dumps(revision)))
            old = credence
        created = utc(now - timedelta(days=changes[0][0], hours=3))
        updated = utc(now - timedelta(days=changes[-1][0], hours=3))
        record = {"id": belief_id, "agent": agent, "proposition": proposition, "credence": changes[-1][1], "source": src,
                  "created_at": created, "updated_at": updated, "pinned": pinned}
        db.execute("INSERT OR REPLACE INTO agent_beliefs (id, agent_id, updated_at, record_json) VALUES (?, ?, ?, ?)",
                   (belief_id, agent, updated, json.dumps(record)))


# --- appended: practice history (Journal, Review tutor, Arena) ---------------------------------
# What the practice captures need but the UI cannot make on the day of a run, or the stub engine
# cannot produce (it answers in prose, never the JSON the tutor summary, the debate judge and the
# argument map read):
# - two weeks of mood check-ins, journal entries and focus sessions on past journal days;
# - time capsules: one ready (sealed yesterday for a day, with an image), one sealed for a year,
#   one opened earlier with a reply;
# - two Socratic tutor dialogues: one waiting for an answer, one finished with its summary;
# - two debates (one judged, with its verdict and claim map; one stopped after three of six
#   turns) and a thought experiment (the trolley problem) answered by two agents.
# Journal days follow the app's 04:00 rollover.

def write_png(path: Path, width: int, height: int) -> None:
    """A small evening-sky picture (standard library only)."""
    import struct
    import zlib

    rows = []
    for y in range(height):
        t = y / height
        row = bytearray([0])
        for x in range(width):
            r, g, b = int(46 + 170 * t), int(58 + 92 * t), int(120 - 30 * t)
            dx, dy = x - width * 0.7, y - height * 0.32
            if dx * dx + dy * dy < (height * 0.13) ** 2:
                r, g, b = 250, 222, 150
            if y > height * (0.78 - 0.12 * abs(((x / width) * 3) % 2 - 1)):
                r, g, b = 28, 36, 54
            row += bytes((r, g, b))
        rows.append(bytes(row))

    def chunk(kind: bytes, data: bytes) -> bytes:
        return struct.pack(">I", len(data)) + kind + data + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF)

    png = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", width, height, 8, 2, 0, 0, 0))
    png += chunk(b"IDAT", zlib.compress(b"".join(rows), 9)) + chunk(b"IEND", b"")
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(png)


def seed_practice(db: sqlite3.Connection, profile: Path) -> None:
    local_now = datetime.now().astimezone()
    tz = local_now.tzinfo
    jday = (local_now - timedelta(hours=4)).date()

    def at(days_ago: int, hour: int, minute: int = 0) -> datetime:
        d = jday - timedelta(days=days_ago)
        return datetime(d.year, d.month, d.day, hour, minute, tzinfo=tz)

    def z(when: datetime) -> str:
        return when.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")

    def sort_key(when: datetime) -> str:
        return when.astimezone(timezone.utc).isoformat()

    def day_of(days_ago: int) -> str:
        return (jday - timedelta(days=days_ago)).isoformat()

    def journal_row(table: str, rid: str, day: str, sort_at: str, record: dict) -> None:
        db.execute(f"INSERT OR REPLACE INTO {table} (id, day, sort_at, record_json) VALUES (?, ?, ?, ?)",
                   (rid, day, sort_at, json.dumps(record)))

    # Journal entries: a morning page and an evening review yesterday (today's comes from the UI).
    entries = [
        ("demo_entry_morning", 1, 7, 30, "morning",
         "## Today's intention\nRead Part I slowly, one proposition at a time.\n\n## Three priorities\n- Definitions 1 to 8\n"
         "- Questions for the reading group\n- A walk before lunch\n\n## I'm grateful for\nA quiet morning and the first frost.\n"),
        ("demo_entry_evening", 1, 21, 40, "evening",
         "## What went well\nThe group took the mirror question seriously.\n\n## What I learnt\nSpinoza's 'in itself' is a claim about "
         "explanation, not about size.\n\n## What surprised me\nSokrates agreed too quickly; I should ask why.\n\n"
         "## Tomorrow: if … then I will …\nIf I reach for my phone, then I will read one proposition instead.\n"),
    ]
    for rid, days_ago, hour, minute, template, body in entries:
        when = at(days_ago, hour, minute)
        journal_row("journal_entries", rid, day_of(days_ago), sort_key(when),
                    {"id": rid, "day": day_of(days_ago), "created_at": z(when), "updated_at": z(when), "body": body,
                     "prompt": None, "template": template})

    # Mood check-ins over two weeks: brighter on days with writing or a focus session.
    checkins = [
        (14, 8, 10, 0.55, 0.30, 4, 7.5, ["reading"], "Slept well"),
        (13, 20, 5, -0.30, -0.40, 2, 6.0, ["work"], ""),
        (12, 9, 0, 0.10, 0.15, 3, 7.0, [], ""),
        (11, 13, 30, -0.45, 0.50, 2, 5.5, ["deadline"], "Too many tabs open"),
        (10, 8, 45, 0.20, -0.20, 3, 7.0, ["walk"], ""),
        (9, 21, 15, 0.65, 0.35, 4, 8.0, ["reading", "friends"], "Good talk after the seminar"),
        (8, 7, 50, -0.20, -0.55, 2, 5.0, ["poor sleep"], ""),
        (7, 12, 20, 0.30, 0.10, 3, 7.0, [], ""),
        (6, 9, 5, 0.70, 0.45, 5, 8.0, ["writing"], "Wrote first, then email"),
        (5, 10, 40, 0.45, 0.25, 4, 7.5, ["focus"], ""),
        (4, 18, 30, -0.10, -0.30, 2, 6.5, ["errands"], ""),
        (3, 8, 15, 0.60, 0.40, 4, 7.5, ["writing", "walk"], ""),
        (3, 21, 0, 0.35, -0.45, 3, None, ["reading"], "Calm evening"),
        (2, 13, 10, 0.05, 0.20, 3, 6.5, [], ""),
        (1, 8, 0, 0.55, 0.30, 4, 7.0, ["reading"], ""),
        (1, 22, 10, 0.40, -0.50, 3, None, [], "Tired but content"),
    ]
    for n, (days_ago, hour, minute, valence, arousal, energy, sleep, tags, note) in enumerate(checkins):
        when = at(days_ago, hour, minute)
        rid = f"demo_checkin_{n:02d}"
        journal_row("mood_checkins", rid, day_of(days_ago), when.isoformat(),
                    {"id": rid, "at": when.isoformat(), "day": day_of(days_ago), "affect": {"valence": valence, "arousal": arousal},
                     "energy": energy, "sleep_hours": sleep, "tags": tags, "note": note, "wants_support": False})

    # Focus sessions: one on today's review (done), one on the Ethics (partly), one not done.
    sessions = [
        ("demo_focus_review", 1, 9, 0, "Clear today's review", 25, 22, {"kind": "review"}, 0, "done", "Every item, one slip"),
        ("demo_focus_ethics", 3, 14, 0, "Read Ethics I, propositions 1 to 8", 50, 46, {"kind": "document", "id": "doc_ethics"}, 1,
         "partly", "Stopped at P6"),
        ("demo_focus_draft", 5, 10, 0, "Draft the semiotics post", 25, 18, None, 3, "not_done", "Kept checking messages"),
    ]
    for rid, days_ago, hour, minute, intention, planned, focused, link, distractions, outcome, reflection in sessions:
        start = at(days_ago, hour, minute)
        journal_row("focus_sessions", rid, day_of(days_ago), sort_key(start),
                    {"id": rid, "day": day_of(days_ago), "started_at": z(start), "ended_at": z(start + timedelta(minutes=focused)),
                     "intention": intention, "planned_minutes": planned, "focused_minutes": focused, "link": link,
                     "distractions": distractions, "outcome": outcome, "reflection": reflection})

    # Time capsules. The image path is relative to the profile folder (the app's working folder
    # in captures): demo/build.ps1 keeps the profile's media folder and run.ps1 copies it back.
    image = "media/attachments/demo-capsule-sky.png"
    write_png(profile / image, 480, 320)
    capsules = [
        ("demo_capsule_ready", at(1, 21, 50), 1, 0,
         "Tomorrow-you: did the reading group like the mirror question? Look at this sky before you answer.",
         [{"path": image, "size": {"width": 480, "height": 320}}], None, None),
        ("demo_capsule_year", at(20, 22, 5), 20, -345,
         "A year from now: are you still reading one proposition a morning? Which book replaced the Ethics?", [], None, None),
        ("demo_capsule_opened", at(45, 20, 30), 45, 15,
         "Will you have finished the Tractatus by now, or only the ladder?", [], at(15, 9, 10),
         "Finished it, slower than I feared. The ladder was the point."),
    ]
    for rid, written, written_ago, opens_ago, body, attachments, opened, reply in capsules:
        opens_on = day_of(opens_ago)
        journal_row("time_capsules", rid, opens_on, sort_key(written),
                    {"id": rid, "written_at": z(written), "written_on": day_of(written_ago), "opens_on": opens_on, "body": body,
                     "attachments": attachments, "opened_at": z(opened) if opened else None, "reply": reply, "announced": True})

    # Socratic tutor dialogues on Spinoza's posts.
    def post_text(post_id: str) -> str:
        row = db.execute("SELECT content FROM posts WHERE id = ?", (post_id,)).fetchone()
        return row[0] if row else ""

    def question(text: str, level: str, move: str) -> dict:
        return {"text": text, "level": level, "move": move}

    def assessed(reached, verdict: str, right: list, wrong: list) -> dict:
        return {"reached": reached, "verdict": verdict, "right": right, "misconceptions": wrong}

    tutor = [
        ("tutor_demo_open", "post_3", at(1, 20, 40), [
            {"question": question("Before anything else: what do you take 'substance' to mean in this post?", "remember", "elicit"),
             "answer": "Something that exists by itself and does not need anything else to be thought of.",
             "assessment": assessed("remember", "solid", ["You gave Spinoza's own test: in itself and conceived through itself."], [])},
            {"question": question("Put it in your own words: why could there be only one such substance?", "understand", "explain"),
             "answer": None, "assessment": None},
        ], False, None),
        ("tutor_demo_done", "demo_p08", at(2, 19, 15), [
            {"question": question("What do you think Spinoza means by 'the order and connection of ideas'?", "remember", "elicit"),
             "answer": "The way ideas follow from each other, like steps in a proof.",
             "assessment": assessed("remember", "solid", ["Ideas follow from ideas, as in a demonstration."], [])},
            {"question": question("Explain to a friend how one order can be 'seen twice'.", "understand", "explain"),
             "answer": "A river and the thought of the river are one event, seen as body and as mind.",
             "assessment": assessed("understand", "solid", ["You used the thread's river example well."], [])},
            {"question": question("If thought and body are one order, where does error come from?", "apply", "apply"),
             "answer": "From a false idea that does not match the thing.",
             "assessment": assessed(None, "missing", [], ["For Spinoza error is a privation, not a separate false idea."])},
        ], True, {"shown": ["You can state the parallelism in your own words.", "You connected it to the river example from the thread."],
                  "missing": ["Why error is a privation rather than a false idea of its own.",
                              "How one order can be seen twice without two substances."],
                  "review": ["What does Spinoza mean when he calls error a privation?",
                             "How can the order of ideas and the order of things be one order?"],
                  "mastery": "understand"}),
    ]
    for sid, post_id, when, exchanges, closing, summary in tutor:
        text = post_text(post_id)
        record = {"id": sid, "network_id": "net_philosophy", "source": {"kind": "post", "id": post_id}, "title": label_of(text),
                  "material": text, "exchanges": exchanges, "closing": closing, "summary": summary, "items_created": 0,
                  "created_at": z(when - timedelta(minutes=12)), "updated_at": z(when)}
        db.execute("INSERT OR REPLACE INTO learning_tutor_sessions (id, network_id, updated_at, record_json) VALUES (?, ?, ?, ?)",
                   (sid, "net_philosophy", sort_key(when), json.dumps(record)))

    # Debates: Sokrates for, Spinoza against, Simone Weil moderating, two rounds (Oxford).
    def slot(rnd: int, speaker: str, side: str, role: str) -> dict:
        return {"round": rnd, "speaker": speaker, "side": side, "role": role, "addressee": None}

    schedule = [slot(0, "agent_simone", "open", "moderation"), slot(1, "agent_socrates", "for", "opening"),
                slot(1, "agent_spinoza", "against", "opening"), slot(2, "agent_socrates", "for", "closing"),
                slot(2, "agent_spinoza", "against", "closing"), slot(2, "agent_simone", "open", "moderation")]

    def debate(did: str, motion: str, texts: list, start: datetime, verdict, claims) -> None:
        turns = [{"slot": schedule[i], "text": t, "at": z(start + timedelta(minutes=2 * i))} for i, t in enumerate(texts)]
        updated = start + timedelta(minutes=2 * len(texts) + (3 if verdict else 0))
        arg_map = None
        if claims:
            ref = lambda i: f"{did}#turn-{i}"
            arg_map = {"root": ref(0), "nodes": [{"id": f"n{k + 1}", "role": "claim", "text": text, "source": ref(turn)}
                                               for k, (turn, text) in enumerate(claims[0])],
                       "edges": [{"from": a, "to": b, "kind": "attack"} for a, b in claims[1]]}
        record = {"id": did, "setup": {"motion": motion, "source_post": None,
                                       "debaters": [{"agent": "agent_socrates", "side": "for"}, {"agent": "agent_spinoza", "side": "against"}],
                                       "format": "oxford", "rounds": 2, "moderator": "agent_simone", "judge": "global_model"},
                  "turns": turns, "verdict": verdict, "map": arg_map, "published": None,
                  "created_at": z(start), "updated_at": z(updated)}
        db.execute("INSERT OR REPLACE INTO arena_debates (id, updated_at, record_json) VALUES (?, ?, ?)",
                   (did, sort_key(updated), json.dumps(record)))

    debate("demodebatejudged", "Understanding is nothing more than very good prediction.", [
        "Welcome. The motion: understanding is nothing more than very good prediction. Sokrates speaks for it, Baruch against. Please answer each other, not the room.",
        "Tell me what else understanding could be. If someone predicts every answer a thing will give, every way it will move, what is missing? Name it, and I will grant the motion falls.",
        "What is missing is the cause. A mirror predicts the sun's position perfectly and knows nothing of fire. To understand is to know a thing through its causes, not to anticipate its surface.",
        "Then let us test the mirror: ask it why the sun rises. It cannot answer, so it does not predict well after all. Prediction that survives every question already contains the causes.",
        "A table of every answer is still a table. It repeats what follows without grasping why it must follow. Adequate ideas explain; they do not merely agree with the outcome.",
        "You both agree that understanding must survive questions. You differ on whether surviving them is the whole of it. That difference is worth keeping.",
    ], at(2, 18, 0), {
        "scores": [{"agent": "agent_socrates", "marks": [7, 8, 9, 8], "note": "Pressed the mirror example until it gave way; answered every point directly."},
                   {"agent": "agent_spinoza", "marks": [7, 8, 6, 7], "note": "A clear account of causes, but the closing restated rather than answered."}],
        "summary": "Both debaters kept to the motion and tested one example hard. Sokrates turned the mirror into a test of prediction; Spinoza held that explanation by causes is more than agreement with outcomes, which left the core question open.",
        "judged_by": "global_model",
    }, ([(1, "Prediction that survives every question leaves nothing missing."),
         (2, "Understanding is knowing a thing through its causes, not anticipating its surface."),
         (3, "A mirror that cannot say why the sun rises does not predict well."),
         (4, "A table of every answer repeats what follows without grasping why it must follow.")],
        [("n1", "n2"), ("n2", "n1"), ("n4", "n3")]))
    debate("demodebatestopped", "A reflection shares in the truth of what it reflects.", [
        "The motion: a reflection shares in the truth of what it reflects. Sokrates for, Baruch against.",
        "Does the bronze mirror lie when it shows the sun? It shows the sun's form truly. Then some truth passes into it.",
        "The form passes, the substance does not. A reflection is an affection of the mirror, conceived through the mirror, not through the sun.",
    ], at(1, 17, 20), None, None)

    # A thought experiment: the trolley problem put to Sokrates and Spinoza.
    answers = [
        ("agent_socrates", 0, "yes", "Five lives weigh more than one, if a lever is all I touch.", "No one is used as a means; the harm is a side effect."),
        ("agent_spinoza", 0, "yes", "Reason counts five as five; I would pull it.", "The good of more people follows from the same reasoning."),
        ("agent_socrates", 1, "undecided", "I would have to ask what I owe the one I love first.", "Duties to friends are not obviously outweighed by numbers."),
        ("agent_spinoza", 1, "yes", "Grief does not change the sum; I would still pull it.", "Love is a passion; it should not decide what reason sees."),
        ("agent_socrates", 2, "no", "To push a man is to use him; I will not make a person a tool.", "Doing harm differs from allowing it."),
        ("agent_spinoza", 2, "undecided", "The outcome is the same, but the act needs a passion I distrust.", "Whoever pushes must overcome his nature; the case is unclear."),
        ("agent_socrates", 3, "no", "No argument would move my hands against someone I love.", "Both the act and the bond weigh against it."),
        ("agent_spinoza", 3, "no", "Here the act and the affect both refuse; I would not push.", "Reason and desire agree, for once."),
    ]
    started = at(1, 16, 0)
    experiment = {"id": "demoexperimenttrolley",
                  "scenario": {"title": "The trolley problem",
                               "premise": "A runaway trolley will kill five people on the track ahead unless it is diverted. Diverting it will kill one other person.",
                               "variables": [{"name": "How you divert it", "values": ["pull a lever", "push a person onto the track"]},
                                             {"name": "The one person", "values": ["a stranger", "someone you love"]}],
                               "question": "Should you divert the trolley?"},
                  "agents": ["agent_socrates", "agent_spinoza"],
                  "answers": [{"agent": a, "cell": c, "stance": s, "verdict": v, "reasoning": r} for a, c, s, v, r in answers],
                  "preset": "trolley", "created_at": z(started), "updated_at": z(started + timedelta(minutes=1))}
    db.execute("INSERT OR REPLACE INTO arena_experiments (id, updated_at, record_json) VALUES (?, ?, ?)",
               ("demoexperimenttrolley", sort_key(started + timedelta(minutes=1)), json.dumps(experiment)))


if __name__ == "__main__":
    main()
