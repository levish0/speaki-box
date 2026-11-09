# Agent instructions

## Work within the request

- Work autonomously within the requested scope. Preserve unrelated changes.
- Read [requirements](docs/requirements.md), [architecture](docs/architecture.md), and [conventions](docs/conventions.md) before changing behavior.
- Use descriptive Git branch names. Do not add `codex/` or another agent prefix.
- Make coherent local commits after completing the requested work. Push, open pull requests, tag, and publish only with user authorization.

## Keep responsibilities explicit

- Organize code by responsibility. Give each file, type, and function one clear role.
- Keep `main.rs` thin. Keep application composition in `app.rs` and startup environment access in `runtime.rs`.
- Keep module indexes limited to declarations and exports. Put plugin registration in `plugin.rs`.
- Each feature owns its configuration, state, messages, and systems. Use explicit project imports. The Bevy prelude is permitted.
- Prefer direct implementations. Do not add generic frameworks, compatibility layers, or speculative settings.
- Model closed sets of states with enums. Keep immutable startup choices separate from live settings.
- Declare system order when systems share state or produce and consume messages.
- Run physics at 60 Hz in `FixedUpdate`. Keep pointer input, audio, and presentation outside the fixed simulation.
- Keep platform code inside `window`. Explain each unsafe operation and its platform invariant.

## Write clear technical prose

**Write this 80% of the way to ASD-STE100.**

Apply this direction to specifications, READMEs, runbooks, release notes, verification records, and code comments.
Use short sentences, active verbs, explicit actors, and consistent terms.
State conditions, steps, and expected results. Preserve exact identifiers, commands, units, and constraints.
Use [the writing rules](docs/conventions.md#technical-writing). This is a clarity target, not a claim of formal compliance.

## Check the result

- Use `just` and the locked Cargo dependency graph. See [development](docs/development.md).
- Add regression tests for behavior changes. Do not add tests that only repeat configuration or implementation details.
- Run `just verify` and `just build` before completing a code change.
- For window or rendering changes, inspect the native app. Do not infer desktop transparency from a successful build or an offscreen render.
- Keep code, requirements, commands, and documentation aligned.
- Report local checks, native UI checks, remote CI, publication, and deployment separately. Never claim an unperformed check.
