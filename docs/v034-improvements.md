# v0.3.4 Feature Expansion & Hardening - Implementation Summary

8 improvements across error hardening, CRUD expansion, watch enhancements, and documentation.

## Error Hardening (3 fixes)
1. **export.rs** — Replaced `file.unwrap()` with `if let Some(ref path)` in CSV and Markdown export paths
2. **main.rs** — Moved issue ID regex to `OnceLock` pattern (matching output.rs convention)
3. **main.rs** — Improved `--filter` help text: documented dot-notation, `~=` operator, case-insensitivity, AND logic

## CRUD Expansion (5 new subcommands)
4. **roadmaps create** — `roadmapCreate` mutation with `--description`
5. **roadmaps update** — `roadmapUpdate` mutation with `--name`, `--description`, `--dry-run`
6. **initiatives create** — `initiativeCreate` mutation with `--description`, `--status`
7. **initiatives update** — `initiativeUpdate` mutation with `--name`, `--description`, `--status`, `--dry-run`
8. **documents delete** — `documentDelete` mutation with `--force`, `--dry-run`, confirmation prompt

## Watch Expansion (2 new watch targets)
9. **watch project** — Polls project `updatedAt` with state and progress display
10. **watch team** — Polls team with active cycle info, resolves team key to UUID

## Updated Help Text
- Roadmaps: added create/update examples
- Initiatives: added create/update examples
- Documents: added delete example
- Watch: restructured as subcommands (issue/project/team)
- Filter: documented operators, dot-notation, case behavior

## Test Results
- **167 unit tests** passing
- **58 integration tests** passing (was 46)
- **225 total** (was 213)
- **0 warnings** in build output

## Files Changed
8 files, 715 insertions, 25 deletions.
