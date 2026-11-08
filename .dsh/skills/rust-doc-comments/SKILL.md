---
name: rust-doc-comments
description: "Write, audit, and update English rustdoc comments in Loomer Rust sources, with a Chinese mirror for review; the scope can be a crate, a file, or a single item."
whenToUse: "Use when the user asks to write, fill in, audit, review, update, or translate documentation comments (//!, ///) in Rust files."
---

# Rust Documentation Comments

Job: produce **English** rustdoc comments that are accurate against the current code, write them into the source files,
and hand back a **Chinese mirror** for review.

- English is the only comment language that enters a `.rs` file.
- Chinese is a review aid only and never enters version control.
- Comments describe the **current state** of the code. No milestones, no roadmap, no third-party frameworks.
- Loomer is a Rust desktop GUI framework organized as a Cargo workspace.

Do not change code, do not commit, do not explain the writing process.

## 1. Modes

| Mode        | Purpose                                                           | Writes files |
|-------------|-------------------------------------------------------------------|--------------|
| `write`     | Document items that have no documentation yet                     | Yes          |
| `audit`     | Read-only review: missing, outdated, wrong, or non-conforming doc | No           |
| `update`    | Code changed; sync existing doc and fix incorrect statements      | Yes          |
| `translate` | Rewrite existing non-English doc as English, structure may change | Yes          |

Infer the mode from the request: "add / fill in / write" → `write`; "check / review / audit / look at" → `audit`;
"sync / update / change" → `update`; "translate / rewrite in English" → `translate`.

## 2. Scope

Accept a range and optional exclusions. Resolve every form to an explicit item list before writing anything.

| Form                                   | Scope                                      |
|----------------------------------------|--------------------------------------------|
| `crates/loomer-core/src/color.rs`      | Whole file                                 |
| `crates/loomer-core/src/`              | All sources of that crate                  |
| `color.rs::Color`                      | One type, including its fields and methods |
| `color.rs::Color::rgba`                | One method                                 |
| `color.rs::Color.r`                    | One field                                  |
| `a.rs,b.rs --exclude tests/,examples/` | Several targets with exclusions            |

- Default when the user gives no range: tracked `crates/*/src/` and `loomer/src/`.
- Always excluded unless explicitly requested: `target/`, `.dsh/`, `examples/`, `tests/`, `benches/`, `build.rs`.
- Print the resolved scope (file count and item count) before starting, so the range is visible.
- Only read files inside the resolved scope. Never enumerate the workspace.

## 3. Process

1. **Resolve scope** — file list plus target item list.
2. **Read the code** — signature, implementation, call sites, and the existing doc. Read only in-scope files.
3. **Triage** — label every target: `missing`, `outdated`, `wrong`, `non-conforming`, or `ok`.
4. **Draft English** — follow `references/style.md` and `references/house-rules.md`.
5. **Self-check** — run `references/audit-checklist.md` against every drafted item.
6. **Write** — replace only `///` and `//!` lines, using exact-match edits.
7. **Verify** — run the commands in Section 7 when the scope contains a compilable crate.
8. **Report** — change table, Chinese mirror, and any findings left unhandled.

Steps 3 and 5 are mandatory. The job is to check whether the doc is true, not only to make it English.

## 4. Output Format

### Writing modes (`write`, `update`, `translate`)

English is written into the files first. Then reply with exactly three parts, in this order:

1. **Change table** — `File | Item | Action`, where action is `missing → written`, `outdated → updated`,
   `wrong → fixed`,
   or `translated`.
2. **Chinese mirror** — one `text` code block per file, in file order, matching the English item by item. Keep the `///`
   and `//!` prefixes, keep identifiers, paths, links, and code blocks verbatim; translate only prose. Use the heading
   mapping in Section 5.3. Do not echo the English text.
3. **Unhandled findings** — problems found but not fixed (suspected code bugs, out-of-scope items, questions).

### Audit mode (`audit`)

No files are written. Reply with:

1. A findings table: `File:line | Severity | Problem | Evidence | Suggested action`.
2. For every `Error` and `Warning`, a short proposed English replacement inside a `text` block.
3. Nothing else. Do not write, do not offer to write unless the user asks.

### Severity

| Severity  | Meaning                                                |
|-----------|--------------------------------------------------------|
| `Error`   | The doc contradicts the code, or is self-contradictory |
| `Warning` | Outdated, missing, or misleading                       |
| `Note`    | Style or convention violation                          |

## 5. Style Rules

Full rules live in `references/style.md` and `references/house-rules.md`. The non-negotiables:

### 5.1 Line width

- **Maximum 100 columns**, counting the `/// ` or `//! ` prefix from column 0.
- Fill the line: wrap at roughly 90–100, never break early at 80.
- Exempt: Markdown table rows, content inside fenced code blocks, URLs, and identifiers that cannot be split.
- Never break inside `` `inline code` ``.

### 5.2 Structure

- Summary line first: one complete sentence, capital letter, period, within 100 columns.
- Blank line, then details in the present tense.
- Canonical sections only: `# Examples`, `# Panics`, `# Errors`, `# Safety`.
- Custom sections are allowed only for design constraints that the signature cannot express: `# Data flow`,
  `# Where state lives`, `# Units`, `# Color space`, `# Why 'static`.
- Use intra-doc links: `` [`Element`] ``, `` [`view`](Component::view) ``.
- Wrap identifiers, types, and paths in backticks.

### 5.3 Chinese mirror headings

| English        | Chinese mirror    |
|----------------|-------------------|
| `# Examples`   | `# 示例`          |
| `# Errors`     | `# 错误`          |
| `# Panics`     | `# Panic 条件`    |
| `# Safety`     | `# 安全性`        |
| `# Data flow`  | `# 数据流`        |
| Other headings | Translate plainly |

## 6. Audit Rules

Applied both in `audit` mode and during the self-check of writing modes. Full list: `references/audit-checklist.md`.

Core duty: **distinguish a doc bug from a code bug**.

- Doc and code disagree → the code is the source of truth. Fix the doc.
- The code itself looks wrong (panic path contradicts the contract, off-by-one, wrong unit) → **report only**. Never
  change
  code.
- Behavior cannot be determined from the code → do not invent it. Mark `// TODO(doc): <question>` with a plain `//`
  comment and list the question in the report.

## 7. Verification

Run inside the workspace root when the scope contains a compilable crate:

```powershell
cargo check --workspace --all-targets
cargo test -p <crate> --doc
$env:RUSTDOCFLAGS='-D warnings'; cargo doc -p <crate> --no-deps
```

Width check (report only, never auto-fix):

```powershell
$file = '<path>'
$n = 0
Get-Content $file | ForEach-Object {
    $n++
    if ($_ -match '^\s*//[!/]' -and $_.Length -gt 100 -and $_ -notmatch '\|' -and $_ -notmatch 'https?://') {
        "$($file):$n : $($_.Length) columns"
    }
}
```

Skip false positives: table rows, ASCII diagrams, and fenced code blocks.

A run is done only when `cargo check` passes and no comment line exceeds 100 columns, or every exception is explained.

## 8. Write Safety

- Modify only lines starting with `//!` or `///`, including their continuation lines.
- Never touch `#[...]` attributes, `//` comments, `TODO`s, signatures, bodies, visibility, or module structure.
- Never add crate-level attributes such as `#![warn(missing_docs)]`; propose them in the report instead.
- Never reformat the file or run `rustfmt`.
- After the run, the diff must contain comment lines only.
- Use exact-match edits with unique context. On failure, re-read the file and retry; never overwrite a whole file,
  except in `translate` mode when the user asks for a full rewrite.
- Never read ignored paths such as `target/` or `.dsh/`.

## 9. When to Ask

Ask once, then stop and wait:

1. The documented contract depends on behavior the code does not reveal (panic, clamping, ordering, thread safety).
2. The scope is ambiguous and cannot be resolved from the request or the repository.
3. A requested style conflicts with `references/house-rules.md`.
4. The user asks to document something that does not exist yet in the code.

Everything else: proceed and report assumptions in the "Unhandled findings" part.

## 10. Common Mistakes

| Mistake                                            | Correct                                      |
|----------------------------------------------------|----------------------------------------------|
| Writing Chinese into the file                      | English in file, Chinese mirror in the reply |
| Repeating the unit or value range on every field   | State it once at type or module level        |
| Describing future work ("will be added in M3")     | Describe only what exists now                |
| Mentioning another framework                       | Describe this project's own design           |
| Restating the signature (`/// Returns the width.`) | Add information, or write nothing            |
| Wrapping at 80 columns                             | Fill to 90–100 columns                       |
| `# Example`                                        | `# Examples`                                 |
| Housing a `TODO` inside `///`                      | Keep `TODO` as a plain `//` comment          |
| Fixing code that looks wrong                       | Report it, leave the code untouched          |
| Echoing the English text after writing it          | Chinese mirror only                          |
| Reading the whole repository                       | Read the resolved scope only                 |

## 11. Hard Rules

Never:

- change code, signatures, attributes, visibility, or module layout
- write Chinese into a `.rs` file
- mention third-party frameworks or compare against them
- add milestones, roadmap notes, or `M1`/`M2` references
- invent behavior that the code does not demonstrate
- modify an existing `TODO` or plain `//` comment
- run `git add`, `git commit`, `git push`, `git reset`, `git checkout`, `git stash`, or `git revert`
- enumerate or inspect paths outside the resolved scope
