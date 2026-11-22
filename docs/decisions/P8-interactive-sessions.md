# P8: persistent interactive sessions (deferred)

**Status:** spike written up, not built (issue #282, marked *Later*).

## The idea

Today each Run is one program, one fresh folder, one end (P2). A session would keep a runtime
alive between runs, so a variable set in one block is still there in the next (a notebook), or
so the user can type into a running program (a REPL).

## Design, if it is built

- **One session per post and runtime**, opened by the first Run of a block of that runtime and
  shown in the output panel as "Session · Python 3.12" with End.
- **Same runner, longer life.** The session's process runs in the same bounded tree (job object
  / process group), cleared environment and private folder as a run. The folder lives as long as
  the session. The time limit becomes two limits: per evaluation (as today) and an idle limit
  after which the session is ended (for example 10 minutes), plus an overall cap.
- **A protocol per runtime, not a terminal.** Each block is sent to the runtime's REPL over stdin
  with a sentinel the runtime echoes after it (`print("\x00done:<n>")` in Python, `console.log`
  in Node, `io.write` in Lua), so the panel knows where one block's output ends. Go and PHP have
  no usable REPL and would stay one-shot. No pseudo-terminal: a PTY adds a terminal emulator to
  the attack surface and gives nothing the panel can show.
- **Stdin stays closed to the user** at first: blocks are the only input, so nothing typed into
  the panel reaches a program by accident.
- **Confirm per block, not per session.** Code from agents is still shown and confirmed block by
  block; a session that an agent's block joins is marked as such.
- **Saved runs** keep working per block; the session itself is never saved (its state is the
  program's memory) and ends when the app closes.

## Why it is deferred

- **Safety.** A long-lived process with the user's rights outlives the moment the user looked at
  the code: the confirm no longer bounds what runs or for how long, and every runtime's REPL
  differs in how it can be driven and stopped. The one-shot runner (P2–P7) is reviewed; a session
  needs its own review (idle limits, sentinel spoofing by the code itself, output interleaving).
- **Value against cost.** Posts are at most 280 characters; a block rarely needs the state of
  another. The per-runtime protocols (Python, Node, Deno, Lua, Ruby) would be most of the work
  and most of the risk, for a feature no post today depends on.
- **Platform gaps.** The open hardening items of P7 (the Windows job race, Unix `setsid` escapes,
  no Unix rlimits) matter more for a process that lives for minutes than for one that lives for
  seconds; they should be closed first.

## When to revisit

After the P7 deferred items are fixed, and if posts start chaining code blocks (a thread that
builds up a computation). The first step would be Python only, one session per post, with the
idle limit and the sentinel protocol above, behind its own switch in Settings › Code that is off
by default.
