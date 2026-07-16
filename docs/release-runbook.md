# Sagnir Release Runbook

Status: policy

Sagnir uses one solo-maintainer release loop. Codex handles implementation,
tests, documentation, release notes, reports, commits, release gates, tags, and
pushes. The maintainer handles pentesting, reports GitHub CI status, and gives
the final tag-and-push instruction.

## 1. Implementation

1. Codex finishes the version criteria from [Version Plan](VERSION_PLAN.md).
2. Codex runs the applicable local gates, updates documentation and release
   notes, and commits as needed.
3. Codex stops and requests pentest for the exact implementation-stop commit.

## 2. Pentest

The maintainer runs the pentest.

If the pentest is green:

1. Codex removes any root `PENTEST.md`.
2. Codex writes `security/pentest/<tag>.md` with `Status: PASS`.
3. Codex records the exact tested commit, tester, date, scope, and notes.
4. Codex updates release documentation, runs the release gate, commits the
   release record, and waits for GitHub CI.

If the pentest reports issues:

1. The maintainer writes findings to root `PENTEST.md`.
2. Codex fixes the findings, adds tests and documentation, removes the scratch
   file, runs the local gates, and commits the fixes.
3. Codex requests a retest.
4. The loop repeats until the maintainer reports a green pentest.

Root `PENTEST.md` is scratch input only and is never committed.

## 3. GitHub CI

If GitHub CI is green:

1. The maintainer reports green status and explicitly instructs Codex to tag
   and push.
2. Codex verifies the final release gate, creates the tag, and pushes the
   requested commit and tag.

If GitHub CI reports an issue:

1. The maintainer provides the failing check or log.
2. Codex fixes the issue, records the CI finding and resolution in the
   permanent pentest report, runs the local gates, and commits the correction.
3. The project waits for GitHub CI again.

No tag is created before both pentest and GitHub CI are green. Ordinary
per-version releases require no external reviewer, committee, or separate
approval process.

## Commands

Run the complete local gate:

```bash
scripts/checks.sh
```

Run the security tool gate:

```bash
scripts/security_tool_gate.sh
```

Generate SBOM evidence when required by the target milestone:

```bash
scripts/generate-sbom.sh
```

Run the target version's release gate before requesting a tag.
