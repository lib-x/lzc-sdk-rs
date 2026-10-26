# Protobuf Submodules and SDK Release Implementation Plan

> **For agentic workers:** Use the existing session to execute the authorized migration and release, then request an independent code review before publication. Steps use checkbox syntax for tracking.

**Goal:** Track official protobuf sources as Git submodules, update to their current revisions, and publish the updated Rust SDK to crates.io.

**Architecture:** Keep upstream checkouts under `upstream/` and use their Git commits as the version pins. Continue committing the normalized `proto/` tree and generated `src/gen/` bindings so consumers build without upstream checkouts or code generation tools.

**Tech Stack:** Git submodules, Bash, Buf, Rust 2024, Rust 1.88 MSRV, prost/tonic code generation.

**Spec:** The current user request supersedes the source-fetching mechanism in `../specs/2026-07-13-full-rust-sdk-design.md`; the existing runtime and consumer build requirements still apply.

## Global Constraints

- Keep upstream repositories unmodified and pin their exact commits in the parent repository.
- Preserve package layout, original/transformed SHA-256 manifests, and committed Rust bindings.
- Cargo consumers must not require Git submodules, Go, Buf, or protoc.
- Keep Rust 1.88 compatibility and avoid unrelated dependency upgrades.
- Publish only after local checks, packaged-crate verification, and independent review pass.

## Review Focus

- An ordinary clone without initialized submodules must still build the Rust SDK.
- An uninitialized maintenance checkout must receive an actionable initialization error.
- Source verification must work offline once the submodules are initialized.
- Updating a submodule must update provenance and all generated outputs deterministically.
- The published crate must contain `proto/` and `src/gen/`, excluding upstream checkouts.

### Task 1: Track upstream protobuf repositories

**Files:** `.gitmodules`, `upstream/lzc-sdk`, `upstream/lzc-baseos-protos`, `scripts/sync-protos.sh`, `.github/workflows/ci.yml`, `.github/workflows/sync-protos.yml`, `Cargo.toml`, `README.md`.

**Interfaces:** Consumes official SDK and BaseOS Git repositories. Produces initialized, pinned source checkouts read by `scripts/sync-protos.sh [--check]`.

- [x] Add both official repositories as submodules following their `master` branches for explicit `--remote` updates.
- [x] Replace temporary remote clones and hardcoded revisions with initialized local submodules and actual Git commit provenance.
- [x] Enable top-level submodule checkout in CI, exclude `upstream/` from the crate, and document initialization/update commands. The unrelated nested Swift gitlink is unavailable upstream.
- [x] Add daily/manual upstream synchronization with code generation, a protected update branch, and explicit CI build dispatch. Organization policy forbids GitHub Actions PR creation, so automatic PRs are not required.
- [x] Verify missing/dirty/unsafe-source diagnostics, offline synchronization, and shell syntax.

### Task 2: Refresh protocols and release metadata

**Files:** `proto/`, `src/gen/`, `tests/proto_manifest.rs` if the file inventory changes, `Cargo.toml`, `Cargo.lock`, `README.md`, `CHANGELOG.md`.

**Interfaces:** Consumes Task 1's checkouts and sync command. Produces regenerated protocol artifacts and consistent release metadata.

- [x] Compare the old pinned protocol inputs to current upstream commits and choose the version according to the resulting public API changes.
- [x] Synchronize protobuf files and regenerate Rust bindings using the pinned code generation plugins.
- [x] Update affected fixture code only if upstream API changes require it.
- [x] Update the crate version, lockfile, installation example, and changelog with the verified changes.
- [x] Run source/codegen checks, Rust formatting, Clippy, all-feature tests, documentation, and package verification with Rust 1.88.

### Task 3: Review and publish

**Interfaces:** Consumes Task 2's verified release candidate. Produces a published crate with read-back verification and matching source/tag delivery.

- [x] Independently review the full candidate diff against `v0.1.1`, fix release-blocking findings, and verify affected behavior.
- [x] Inspect the package archive and verify it builds without initialized submodules.
- [ ] Commit the intended files, publish the crate, and read back registry metadata and the uploaded archive.
- [ ] Deliver matching source and version tag to the origin repository, then check the resulting CI run.
- [ ] Record the new source/update procedure and release result in durable memory.
