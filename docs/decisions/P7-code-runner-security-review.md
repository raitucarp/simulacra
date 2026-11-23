# P7: security review of running code blocks (P2–P6)

**Status:** reviewed; five findings fixed in this change, the rest accepted or deferred below
(issue #281).

## What is reviewed

| Part | Where |
|---|---|
| P2 runner: folder, time limit, output cap, cleared environment, process tree | `src/runner/{code,folder,env,process,tree*}.rs` |
| P3 language → command table | `src/domain/code/commands.rs` |
| P4 Settings › Code | `src/domain/preferences/categories/code.rs`, `src/ui/features/settings/panes/runtimes.rs` |
| P5 Run button and confirm | `src/ui/features/post/body/code_run.rs`, `src/ui/features/shell/handlers/code_{run,confirm}.rs` |
| P6 output panel and saved runs | `src/ui/features/post/body/run_panel.rs`, `src/domain/code/run.rs`, `src/state/code/` |
| P1 detection, as it feeds the runner | `src/runner/probe.rs` |

## Threat model

Code in a post may come from the user, from an agent (text a model wrote, which a prompt
injection can steer), or from an imported or captured passage. The runner is **not a sandbox
for the file system or the network**: code runs as the user, with the user's rights, like code
pasted into a terminal. The defences are therefore about *who decides* that code runs, *what
else* is started, and *how much* a run can take:

1. Nothing runs unless the user switched it on (P4) and pressed Run (P5).
2. Code the user did not type is shown in full and runs only after an explicit confirm (P5).
3. Only the runtime's own executable starts, with a fixed command line, never a shell (P2, P3).
4. A run is bounded: fresh folder, cleared environment, time limit, output cap, and every
   process it started is ended with it (P2).
5. Nothing of a locked post's sealed text is run or stored (X2 with P5, P6).

## Checked and holding

- **No shell anywhere.** Programs start through `std::process::Command` with the arguments as
  a list (`runner::code::run_code`, `runner::probe::version_of`). The code is written to a file
  and passed by name; it is never part of a command line. On Windows only `.exe` files are taken
  from the search path, so a `.bat`/`.cmd` (which `CreateProcess` would hand to `cmd.exe`) never
  starts (`find_on_path`, test `executables_are_found_on_the_search_path_and_shell_scripts_never`).
  Shell languages (`sh`, `bash`, `powershell`, `cmd`) map to no runtime (test
  `language_tags_pick_their_runtime`).
- **Detection runs only version probes.** `Runtime::probe` holds the version flag of each runtime
  and nothing else (test `every_runtime_has_a_label_and_a_version_probe`); probes run in the same
  bounded runner (10 s, 4 KiB, cleared environment) and only when the user presses Refresh.
- **Master switch off.** `CodePrefs::default().enabled == false`; a missing or unreadable row reads
  as off; `allows` needs the master switch and the runtime's own switch (tests in
  `domain/preferences/tests/code.rs`). With it off no block gets Run (`code_actions` is `None`).
- **Confirm for code the user did not type.** `needs_confirm` asks for every author that is not
  one of the user's own personas and for every imported or captured post, whoever it is
  attributed to. The dialog shows the whole code (posts are at most 280 characters, so it always
  fits), the author and the runtime, and a warning that it runs with the user's rights. What is
  shown is the request's own copy of the code, and the same copy runs, so an edit between the
  confirm and the run cannot swap it. A quoted or reposted post is scoped to its own id, so Run
  inside a quote of an agent's post still asks.
- **Fresh folder.** Each run gets a new folder (`create_dir`, never reused; mode `0700` on Unix)
  under the system temp folder, removed afterwards with everything in it (test
  `code_runs_in_a_fresh_folder_that_is_removed_afterwards`).
- **Cleared environment.** `env_clear()`, then `PATH` (the runtime's own folder only), home and temp
  pointed at the run's folder, and on Windows `SystemRoot`, `SystemDrive`, `windir`; plus the
  runtime's fixed extras (`NO_COLOR`, `PYTHON*` encoding, `DENO_DIR`, `GO*`). No key, token, proxy
  or home of the user reaches the code (test `the_environment_holds_only_what_a_runtime_needs`).
- **Time limit and output cap.** The watch loop stops the run at the timeout (1–300 s, default 10)
  and when output passes 64 KiB; readers keep at most the cap in memory (tests
  `a_run_past_its_time_is_stopped_with_everything_it_started`, `output_past_the_cap_stops_the_run`).
  Stdin is empty. No console window opens on Windows.
- **Whole process tree.** Windows: a job object with `KILL_ON_JOB_CLOSE`, no breakaway, at most
  64 processes and 2 GiB committed memory; the job is ended on timeout, cancel, cap *and* after a
  normal exit, so nothing a run started outlives it. Unix: the program leads a new process group,
  killed with `SIGKILL` in the same cases. The kill-tree test starts a grandchild that holds the
  output pipe; the run only returns quickly if it was ended too.
- **Fail closed.** If the program cannot be put in its job, it is killed and the run fails.
- **Locked posts.** The unlocked view of a locked post renders without Run; `save_code_run`
  refuses a locked post (test `a_locked_post_never_keeps_a_run`); the search and agents never
  read `extras.code_runs` (they read `content`, the placeholder).
- **Saved output.** At most 8 Ki characters per block, plain text (no markup is interpreted),
  stored in the post's extras, which agents' context and the search index never read.
- **Exports.** An export never carries `preferences.code` or the detection cache (test
  `an_export_never_carries_the_code_switches`), so an imported file cannot switch running on.

## Findings fixed here

| # | Finding | Fix |
|---|---|---|
| F1 | The runtime's path came from the cached detection (`code.runtimes`) and was trusted as is. A restored vault or snapshot from elsewhere, or an edited row, could name any program, which Run would then start. | `runner::trust::is_runtime_executable`: before every run the path must be absolute, an executable file, named like one of its runtime's probe executables (`python3`, `lua5.4`…, `.exe` only on Windows). `spec_for` refuses otherwise (test `only_an_executable_named_like_its_runtime_runs`). |
| F2 | The runtime cache kept between runs (Go build cache, Deno modules) lived in a fixed folder under the temp folder. On a multi-user Unix system another user could create it first and plant cache entries. | `runner::trust::private_cache`: the cache is made `0700`, and used only when this user owns it and its parent and no one else may write them; otherwise the run caches inside its own folder. |
| F3 | Deno fetched remote modules on import and read a `deno.json` from any folder above the run's. | `deno run --no-prompt --cached-only --no-config`: no network module fetches, no config from above; Deno still grants the code no permission at all. |
| F4 | `go run` read a `go.mod`/`go.work` from any folder above the run's folder (on Unix the shared `/tmp`), which can change the build. | `GO111MODULE=off`, `GOWORK=off` (GOPATH mode for the single file), with `GOTOOLCHAIN=local` and `GOPROXY=off` as before. |
| F5 | A confirm could be accepted after code running was switched off while it was open. | `start_code_run` checks the switches again before anything starts. |

## Accepted or deferred

- **No file-system or network sandbox** (by design, see the threat model). Deno is the exception:
  it runs with no permissions. Node's permission model (`--permission`) was not used because it is
  missing or experimental on the Node versions users have; a later change could enable it when the
  detected version supports it.
- **Windows job race.** The program joins its job right after it starts, not before, since
  `std::process` cannot start a process suspended or with a job attribute. A program that starts a
  child within that first moment, before its runtime has even read the code file, would leave it
  outside the job. Accepted: every runtime in the table reads and compiles its file first.
  Fix later: start suspended (`CREATE_SUSPENDED`) and resume after joining.
- **Unix escapes and limits.** A program can leave its process group (`setsid`) and outlive the
  run; there are no `rlimit`s on Unix. Fix later: a cgroup per run on Linux, or
  `PR_SET_CHILD_SUBREAPER` with a reaper, and `RLIMIT_AS`/`RLIMIT_NPROC` set in `pre_exec`.
- **A restored vault is trusted like the user's own.** A snapshot restore brings back the
  preferences (including the master switch) and posts attributed to the user's personas, whose
  code runs without a confirm. F1 limits what can start to real runtimes; the code itself is the
  user's restored data.
- **Interpreter-specific startup files** (for example a `sitecustomize` next to a system Python, or
  `node_modules` in a folder above the temp folder) are the installation's own and are not
  overridden beyond Python's `-I` and PHP's `-n`.

## Verdict

With F1–F5 fixed, the runner meets the epic's rules: off by default, agents' code always
confirmed with the code shown, a fresh folder, a time limit, an output cap, a cleared
environment, the whole tree ended on timeout or cancel, never a shell, and detection limited to
version probes. The open items above are hardening, not holes in those rules.
