# Lunie vendored fork of `big_space`

This fork exists so Lunie can use `big_space` for floating-origin rendering
against its **local Bevy fork** (`0.20.0-dev`), which is newer than any Bevy
version `big_space` publishes against.

## Branch

`bevy-0.20-compat`, based on `upstream/main` (`06c66a7`).

Based on `main` rather than the `v0.12.0` tag because main carries correctness
fixes in exactly the subsystems a floating-origin globe depends on --
`Stationary` race condition, spatial-hash correctness, change detection and
stationary init timing -- and doubles the test suite from 19 to 40.

## Local changes

1. **`fix: support Bevy 0.20 Result-returning query APIs`** -- three library
   sites where `Query::iter_many` now yields `Result`, plus four test sites
   where `SystemState::get_mut` now returns `Result`. This commit is intended
   for upstream and is kept isolated so it can be cherry-picked cleanly.

2. **`chore: point bevy deps at the Lunie bevy fork`** -- rewires every
   `bevy_*` dependency to a path dependency on `../../Pub_Code/bevy`. This is
   local-only and must never be upstreamed.

## Why path deps rather than `[patch.crates-io]`

`big_space` requires `bevy_* ^0.18.0`; the fork is `0.20.0-dev`. A patch entry
is **silently ignored** when the patched version does not satisfy the
requirement, so `[patch.crates-io]` resolves the registry copy instead and
leaves two incompatible Bevy versions in the dependency graph. Rewriting the
requirements to path deps is the only reliable way to bind to the fork.

Note that `big_space` depends only on granular subcrates (`bevy_app`,
`bevy_ecs`, `bevy_math`, `bevy_transform`, ...), never the umbrella `bevy`
crate, except in dev-dependencies.

## Verification

```
MACOSX_DEPLOYMENT_TARGET=26.0 cargo test --lib
# 40 passed; 0 failed
```
