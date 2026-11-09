# Project conventions

## Code structure

Keep application composition explicit. `app.rs` installs plugins and declares shared schedule order.
Each feature owns its settings and systems. `mod.rs` declares modules; `plugin.rs` registers behavior.
Use complete, descriptive names. Use explicit imports for project types.
Use the Bevy prelude for common engine types.
Keep injected Bevy resources and query filters visible in system signatures.
The Clippy argument-count and type-complexity limits do not apply to these signatures.

Prefer concrete types and direct function calls. Introduce an abstraction only when it removes a real duplication or protects a documented invariant.
Keep platform-specific startup code inside `window`.
Document the reason for unsafe code and keep its scope small.

Use `FixedUpdate` for the 60 Hz simulation. Velocity values are pixels per simulation tick.
Use `Update` for pointer sampling, window events, audio requests, and visual effects.
Cap virtual frame time at 50 ms. Use real time for pointer velocity samples.
Declare an order between producers and consumers. Do not rely on plugin insertion order.

## Technical writing

**Write this 80% of the way to ASD-STE100.**

This target applies to specifications, README text, procedures, release notes, verification records, and code comments.
It means practical clarity. It does not mean formal compliance with the controlled dictionary.

- Give each sentence one main idea.
- Use active verbs. Name the actor when ownership matters.
- State the condition before the action.
- Give each procedure step one action and an expected result.
- Use one term for each concept. Explain a necessary technical term on first use.
- Preserve exact identifiers, commands, paths, limits, and units.
- Keep the intended document language. Apply the same clarity rules to Korean text.
- Write comments for reasons and invariants that the code cannot explain by itself.
- Tie validation claims to a date, version, environment, and actual result.

## Changes and verification

Keep dependency versions and `Cargo.lock` together. Use `--locked` for routine builds and checks.
Use maintained crate releases that support the same Bevy version.
Run the commands in [development](development.md).
Add focused tests for a changed behavior. Use native UI checks for compositor behavior.
Keep local validation separate from remote CI, published releases, and deployed builds.
