---
name: commit-message
description: "Generate English Conventional Commits messages for Loomis, a Rust desktop GUI framework, with a Chinese mirror for review; scope is derived from paths and the range can be limited or excluded."
whenToUse: "Use when the user asks for a commit message, commit info, or commit summary, or says 'help me commit'."
---

# Commit Message Generation

Job: read the current changes and produce one **English** Conventional Commits message plus a **Chinese mirror** with
identical structure and content.

- The English block is the only content to commit.
- The Chinese block is for line-by-line review and never enters version control.

Loomis is a Rust desktop GUI framework organized as a workspace of packages.
Do not commit, modify files, or explain.

## 1. Output Format

Emit exactly two `text` code blocks: Chinese first, English second, with no text between them.

```text
<type>(<scope>): <subject>

- <Point 1>
- <Point 2>
```

The Chinese block has the same shape, translated line by line: same count, same order, same content.
Keep `type`, `scope`, `!`, and footer labels (`BREAKING CHANGE:`, `Refs:`, `Closes:`) in English; translate only
subjects and points.

- No text outside the two blocks except the cases below.
- Follow Conventional Commits 1.0.0 and Git conventions.

### Exceptions

| Condition                                   | Output                            |
|---------------------------------------------|-----------------------------------|
| `git rev-parse --is-inside-work-tree` fails | One line: `当前目录不是 Git 仓库` |
| No changes in range                         | One line: `未检测到任何变更`      |
| User explicitly asks for English only       | English block only                |

### Example

```text
feat(layout): 新增 flex 间距与分轴对齐

- 新增 `Column::spacing`，用于设置子元素间距
- 修复内边距超过可用宽度时的溢出
- 重构 `LayoutEngine` 为测量与摆放两个阶段
```

```text
feat(layout): add flex spacing and per-axis alignment

- Add `Column::spacing` to set gaps between children
- Fix overflow when padding exceeds the available width
- Refactor `LayoutEngine` into measurement and placement passes
```

## 2. Process

Run the steps in order; do not skip any.

### 1 Confirm and list

Run `git rev-parse --is-inside-work-tree`. On failure, output the exception line and stop.

Default range: **staged changes only** (`git add`ed). Run `git diff --cached --name-status -M`; it works with and
without commits.

- If the user asks for another range (all changes, unstaged, specific paths, exclusions), use that range instead.
- Apply path filters when the user limits paths.
- Skip auto-generated files; do not analyze them.

### 2 Read every file

| Range            | Method                                                                       |
|------------------|------------------------------------------------------------------------------|
| Staged (default) | `git diff --cached -- <file>`                                                |
| Unstaged         | `git diff -- <file>`                                                         |
| Untracked file   | Read the file itself                                                         |
| All changes      | `git diff HEAD -- <file>`; without HEAD, apply the three rows above per file |

- Never read staged files with plain `git diff`; it only shows unstaged changes.
- Read truncated output to the end. Never skip a file or guess from its name.

### 3 Classify

For each file, record:

- Auto-generated or unrelated → drop.
- Change nature → group.
- Owning package → scope.
- Mergeable with other files into one logical change.
- Breaking-change candidate.
- Issue clue (user mention, branch name).

### 4 Ask if needed

Only for the cases in When to Ask; ask once and wait before continuing.

### 5 Write points first, subject last

- Order the points by group.
- Choose the subject from the dominant group; its `type` and verb must match and cover the whole body.
- Never write `feat` when the body is all refactoring.

### 6 Self-check

Fix every failed item and re-run the check:

- Every changed file maps to at least one point; every point is backed by the diff.
- Subject: lowercase first letter, no period, ≤ 72 characters, type matches the body.
- Points: uppercase first letter, no period, ≤ 72 characters per line, groups in order.
- Breaking change: `!` and `BREAKING CHANGE:` both present.
- Footers `Refs:` / `Closes:` are backed by evidence.
- Chinese and English blocks correspond line by line.
- Only two code blocks; no text outside.

Then emit the two blocks.

## 3. Field Rules

### type

One of: `feat` `fix` `docs` `style` `refactor` `perf` `test` `build` `ci` `chore` `revert`

| Change                                            | type       |
|---------------------------------------------------|------------|
| New public API, widget/component, or capability   | `feat`     |
| Crash, render, layout, or event fix               | `fix`      |
| Performance improvement                           | `perf`     |
| Internal change, same behavior                    | `refactor` |
| Formatting only                                   | `style`    |
| README, design docs, example text                 | `docs`     |
| Tests only                                        | `test`     |
| Build config, features, MSRV                      | `build`    |
| CI pipeline                                       | `ci`       |
| Dependencies, scripts, skill config, ignore rules | `chore`    |
| Revert a commit                                   | `revert`   |

Pick the dominant one when several apply. Code comments and rustdoc follow their code and do not count as `docs`.

### scope

Derive scope from the path; never hard-code package names:

- Under a `Cargo.toml` with `[package]`: use the package name without the project prefix `loomis-`. If the package name
  equals the project name (public facade), use `api`.
- Outside any package: root build/tooling config → `config`; `README.md` and `docs/` → `docs`; `examples/` → `examples`;
  `.github/` → `ci`; skills and ignore rules → `chore`.
- Otherwise use `root`.
- Across packages use the dominant one; use a comma only when they tie: `core,runtime`.
- A package's own `Cargo.toml` belongs to that package's scope.

Never invent scopes from features. Example: `crates/loomis-layout/src/lib.rs` → `layout`; `loomis/src/lib.rs` → `api`.

### Subject

Format: `<type>(<scope>): <subject>`.

- Start with the imperative verb of the chosen group; for `revert`, state the rollback in the imperative.
- Lowercase the first letter except proper nouns and identifiers.
- No trailing period.
- ≤ 72 characters including `type(scope): `; aim for ≤ 50.
- Count characters, not words.

### Body and grouping

Group points by change nature in this fixed order:

| Group                       | Verbs                                                                |
|-----------------------------|----------------------------------------------------------------------|
| Add                         | `Add` / `Introduce` / `Implement`                                    |
| Fix                         | `Fix`                                                                |
| Improve / adjust            | `Optimize` / `Improve` / `Update`                                    |
| Refactor                    | `Refactor` / `Restructure` / `Extract` / `Rename` / `Move` / `Split` |
| Remove                      | `Remove` / `Delete` / `Drop`                                         |
| Docs / test / build / chore | `Document` / `Test` / `Bump` / `Configure` / `Format`                |

- One imperative sentence per point, uppercase first letter, no trailing period.
- One thing per point; merge files of the same logical change into one point.
- ≤ 72 characters per line including `- `. Compress first; if it still does not fit, split into several points instead
  of forcing one long line.
- Within a group, sort by scope, then by path. Never interleave add, refactor, and remove.
- Do not expand auto-generated changes; if they are the bulk, write one dependency or build point.
- Write only what happened; never invent.

### BREAKING CHANGE

Mark as breaking when any applies:

- A public API is removed or renamed, or its signature, trait, or enum variant changes.
- Default behavior, layout rules, or event semantics change and user code must follow.
- Config format, feature name, or MSRV changes and downstream builds break.
- The user says so.

Require both `!` and the footer:

```text
feat(api)!: replace WindowOptions::size with dimensions

- Rename `WindowOptions::size` to `dimensions`

BREAKING CHANGE: `WindowOptions::size` is removed; use `WindowOptions::dimensions` instead
```

- Put `!` after `type(scope)` and before the colon.
- Write `BREAKING CHANGE:` in capitals, one blank line after the body.
- State what breaks and how to migrate.
- Keep the label in English in the Chinese block; translate only the explanation.

### Issues

- Reference: `Refs: #12`; close: `Closes: #12`; several: `Refs: #12, #34`.
- Add a footer only when the user gives or asks for it.
- On a clue without instruction, ask once.
- Put footers one blank line after the body; keep labels in English.

### English style

- Imperative, present tense: `Add` / `Fix` / `Refactor`, never `Added` / `Adds` / `Adding`.
- Never start with `This commit`, `I`, or `We`.
- Keep identifiers, type names, crate names, and paths verbatim; backticks allowed.
- Use standard spelling for proper nouns.
- Half-width punctuation; no emoji, headings, or bold.

## 4. When to Ask

Ask only in these cases, once, at most one round; do not emit code blocks until answered:

1. Breaking change is unclear and affects `!` or the footer.
2. There is an issue clue but no instruction to link it.
3. The change range is ambiguous and cannot be resolved from the repository.

Everything else: output directly without questions or explanation.

## 5. Common Mistakes

| Mistake                                 | Correct                             |
|-----------------------------------------|-------------------------------------|
| One block only                          | Chinese first, English second       |
| Blocks differ in count or order         | Match line by line                  |
| Subject capitalized, points lowercase   | Subject lowercase, points uppercase |
| Trailing period                         | None                                |
| Over 72 characters                      | Compress or split                   |
| Points out of order                     | Group and sort                      |
| `git diff HEAD` without HEAD            | Read the file itself                |
| Invented scope                          | Derive from the path                |
| Auto-generated changes in the body      | Skip them                           |
| Chinese block treated as commit content | Only the English block is committed |

## 6. Hard Rules

Never run:

`git add` `git commit` `git push` `git reset` `git checkout` `git stash` `git revert`

Never:

- create, modify, delete, or rename any project file
- include out-of-range or generated content in the message
- guess instead of reading
