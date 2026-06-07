# Maintaining the starter

This starter is a **living template**: new apps are built from it, and good
generic improvements made in those apps should flow *back* here so the next app
starts with them. This doc is the workflow for that.

## The model

- **The starter is the source of truth.** It holds reusable *patterns*, never
  any app's business logic (just like the `items` table and the sample
  notifications feed are placeholders, not real domains).
- **Apps are derived copies** that keep a link back to the starter, so
  improvements can move in both directions.

## Spawning a new app (keep the link)

Clone from the starter (or GitHub → "Use this template"), then keep the starter
reachable as a remote:

```bash
git clone <starter-repo> myapp
cd myapp
git remote rename origin starter        # the upstream template
git remote add origin <your-app-repo>   # the app's own home
```

Now `starter` is a remote you can pull updates from and push backports to.

## Backporting a feature (app → starter)

When you build something in an app and realize it belongs in the starter:

1. **Is it generic?** Domain-agnostic (a component, a desktop integration, a DB
   helper, a token) → candidate. Tangled with your domain → only the *pattern*
   goes back, not the specifics.
2. **Generalize it.** Strip app-specific names/tables/logic down to the reusable
   shape — the starter must stay domain-free (mirror how `items` is a
   placeholder model).
3. **Keep paths mirrored.** Build it in the same file layout the starter uses
   (`src/lib/...`, `src-tauri/src/...`) so it copies/cherry-picks back cleanly.
4. **Land it with its docs + guardrails:**
   - Add the code.
   - Update [`FEATURES.md`](./FEATURES.md) and the relevant deep doc
     (DESIGN / ARCHITECTURE / NOTIFICATIONS), and `CLAUDE.md` if it adds a
     convention.
   - Keep it green: the no-CSS guard (`pnpm build` / `pnpm check:css`) and
     `cargo build`.
   - Commit one feature per commit, clear message; bump the starter version.

Mechanically: `git cherry-pick <commit>` into the starter, or copy the files +
doc edits.

## Pulling starter updates (starter → app)

```bash
git fetch starter
git cherry-pick <commit>     # or merge, if histories share an ancestor
```

One-feature-per-commit is what makes this clean — a feature mixed with domain
changes won't cherry-pick.

## The Claude shortcut

The starter carries its own rules (`CLAUDE.md`, DESIGN/ARCHITECTURE
conventions, the no-CSS guard), so you can just say:

> "Backport the X feature from `myapp` into the starter."

Claude reads the app's implementation, generalizes it to placeholder form,
applies it in the right paths, and updates `FEATURES.md` + the relevant docs.
The docs are what keep backports consistent instead of drifting.

## The one rule

**The starter holds patterns, never your domain.** Anything you backport should
look like the `items` / sample-notifications placeholders: real structure, no
real business logic. If it can't be expressed generically, it's an *app*
feature — not a *starter* feature.
