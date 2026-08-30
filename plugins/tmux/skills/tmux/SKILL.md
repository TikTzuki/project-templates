---
name: tmux
description: Remote-control tmux sessions for interactive CLIs by sending keystrokes and scraping pane output. Use when a task needs a real TTY — REPLs, TUI apps, interactive installers, or driving several terminal sessions in parallel.
allowed-tools: Bash, Read
---

# tmux

Use tmux only when you need an interactive TTY. For long-running but non-interactive work, prefer a plain background
Bash call (`run_in_background: true`) and read its output — that is simpler and does not need a pane.

tmux earns its place when the program **only** works against a terminal: REPLs, full-screen TUIs, prompts that read
from `/dev/tty`, or anything you must watch and type into over time.

## Quickstart (isolated socket)

Run every session on a private socket so you never touch the user's own tmux server:

```bash
SOCKET_DIR="${TMUX_SOCKET_DIR:-${TMPDIR:-/tmp}/claude-tmux-sockets}"
mkdir -p "$SOCKET_DIR"
SOCKET="$SOCKET_DIR/claude.sock"
SESSION=claude-python

tmux -S "$SOCKET" new -d -s "$SESSION" -n shell
tmux -S "$SOCKET" send-keys -t "$SESSION":0.0 -- 'PYTHON_BASIC_REPL=1 python3 -q' Enter
tmux -S "$SOCKET" capture-pane -p -J -t "$SESSION":0.0 -S -200
```

After starting a session, always tell the user how to watch it:

```
To monitor:
  tmux -S "$SOCKET" attach -t "$SESSION"
  tmux -S "$SOCKET" capture-pane -p -J -t "$SESSION":0.0 -S -200
```

## Socket convention

- Override the socket directory with `TMUX_SOCKET_DIR`; default is `${TMPDIR:-/tmp}/claude-tmux-sockets`.
- Default socket path: `"$TMUX_SOCKET_DIR/claude.sock"`.
- A private socket keeps your sessions out of `tmux ls` for the user's normal server.

## Targeting panes and naming

- Target format: `session:window.pane` (defaults to `:0.0`).
- Keep names short; avoid spaces.
- Inspect: `tmux -S "$SOCKET" list-sessions`, `tmux -S "$SOCKET" list-panes -a`.

## Finding sessions

- List sessions on your socket: `${CLAUDE_PLUGIN_ROOT}/skills/tmux/scripts/find-sessions.sh -S "$SOCKET"`.
- Scan all sockets: `${CLAUDE_PLUGIN_ROOT}/skills/tmux/scripts/find-sessions.sh --all` (uses `TMUX_SOCKET_DIR`).

## Sending input safely

- Prefer literal sends: `tmux -S "$SOCKET" send-keys -t target -l -- "$cmd"`.
- Control keys: `tmux -S "$SOCKET" send-keys -t target C-c`.
- **For interactive TUI apps, do not append `Enter` in the same `send-keys`.** These apps may treat a fast
  text+Enter sequence as a paste or as multi-line input and never submit it. This is timing-dependent, so send the
  text and `Enter` as separate commands with a small delay (tune per environment; increase if needed, or use
  `sleep 1` if sub-second sleeps aren't supported):

```bash
tmux -S "$SOCKET" send-keys -t target -l -- "$cmd" && sleep 0.1 && tmux -S "$SOCKET" send-keys -t target Enter
```

## Watching output

- Capture recent history: `tmux -S "$SOCKET" capture-pane -p -J -t target -S -200`.
- Wait for a prompt: `${CLAUDE_PLUGIN_ROOT}/skills/tmux/scripts/wait-for-text.sh -t session:0.0 -p 'pattern'`.
- Attaching is fine; detach with `Ctrl+b d`.

## Spawning processes

- For Python REPLs, set `PYTHON_BASIC_REPL=1` — the rich REPL redraws the line and breaks `send-keys` flows.

## Windows / WSL

- tmux is supported on macOS and Linux. On Windows, use WSL and install tmux inside WSL.

## Driving several sessions in parallel

tmux is a good fit when you need N independent terminals at once:

```bash
SOCKET="${TMPDIR:-/tmp}/claude-parallel.sock"

# Create the sessions
for i in 1 2 3; do
  tmux -S "$SOCKET" new-session -d -s "task-$i"
done

# Launch work in different directories
tmux -S "$SOCKET" send-keys -t task-1 "cd /tmp/project1 && ./run.sh" Enter
tmux -S "$SOCKET" send-keys -t task-2 "cd /tmp/project2 && ./run.sh" Enter

# For a TUI, split text and Enter with a delay (see above)
tmux -S "$SOCKET" send-keys -t task-1 -l -- "some prompt text" && sleep 0.1 && tmux -S "$SOCKET" send-keys -t task-1 Enter

# Poll for completion by checking whether the shell prompt came back
for sess in task-1 task-2 task-3; do
  if tmux -S "$SOCKET" capture-pane -p -t "$sess" -S -3 | grep -qE '❯|\$ $'; then
    echo "$sess: DONE"
  else
    echo "$sess: running..."
  fi
done

# Read full output from a finished session
tmux -S "$SOCKET" capture-pane -p -t task-1 -S -500
```

**Tips**

- Use separate git worktrees for parallel work on one repo, so branches never conflict.
- Install dependencies before starting the real command in a fresh clone.
- Detect completion by watching for the shell prompt (`❯` or `$`) to return.
- Be patient — don't kill a session just because it looks slow. Capture the pane and check.

## Cleanup

- Kill a session: `tmux -S "$SOCKET" kill-session -t "$SESSION"`.
- Kill all sessions on a socket:
  `tmux -S "$SOCKET" list-sessions -F '#{session_name}' | xargs -r -n1 tmux -S "$SOCKET" kill-session -t`.
- Remove everything on the private socket: `tmux -S "$SOCKET" kill-server`.

## Helper: wait-for-text.sh

Polls a pane for a regex (or fixed string) with a timeout.

```bash
${CLAUDE_PLUGIN_ROOT}/skills/tmux/scripts/wait-for-text.sh -t session:0.0 -p 'pattern' [-F] [-T 20] [-i 0.5] [-l 2000]
```

- `-t`/`--target` pane target (required)
- `-p`/`--pattern` regex to match (required); add `-F` for fixed string
- `-T` timeout seconds (integer, default 15)
- `-i` poll interval seconds (default 0.5)
- `-l` history lines to search (integer, default 1000)
