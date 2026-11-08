# Audit Checklist

Sixteen checks applied in `audit` mode and during the self-check of every writing mode.

For each finding record: `file:line`, severity (`Error` / `Warning` / `Note`), the problem, the evidence from the code,
and the suggested action.

| #  | Check                                | Detect by                                                                                                                         | Severity |
|----|--------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------|----------|
| 1  | Doc contradicts the signature        | Compare parameter names, order, types, return type, `Option`/`Result`, mutability                                                 | Error    |
| 2  | Doc contradicts the body             | Read the implementation: branches, early returns, `unwrap`, `expect`, indexing, arithmetic                                        | Error    |
| 3  | Self-contradictory sentence          | Read each sentence in isolation; look for mutually exclusive terms (for example an encoding said to be both nonlinear and linear) | Error    |
| 4  | Documented promise not implemented   | Doc claims clamping, ordering, idempotence, or normalization that the body does not perform                                       | Error    |
| 5  | Missing failure documentation        | `pub fn` returning `Result` without `# Errors`; panicking path without `# Panics`; `unsafe` without `# Safety`                    | Warning  |
| 6  | Stale description                    | Field, parameter, or behavior renamed or changed after the doc was written                                                        | Warning  |
| 7  | Example claims more than it verifies | Assertions cover only part of the stated property; variables named after a value they do not hold                                 | Error    |
| 8  | Example does not compile             | `cargo test -p <crate> --doc`                                                                                                     | Error    |
| 9  | Broken intra-doc link                | `RUSTDOCFLAGS='-D warnings' cargo doc -p <crate> --no-deps`                                                                       | Error    |
| 10 | Undocumented public item             | Item reachable from another crate with no `///`                                                                                   | Warning  |
| 11 | Internal API not marked              | Public item that only makes sense inside the workspace, with no blockquote marker                                                 | Warning  |
| 12 | Repeated type-level facts            | Unit, range, or coordinate system repeated on each field                                                                          | Note     |
| 13 | Wrong or missing unit                | Length documented as physical pixels, or a unit stated on one item and contradicted on another                                    | Error    |
| 14 | Placeholder or roadmap text          | Milestone names, "will be added", "not yet provided", "temporarily"                                                               | Warning  |
| 15 | Style violations                     | Missing period or capital on the summary line, `# Example` instead of `# Examples`, non-canonical section name                    | Note     |
| 16 | Width or formatting violations       | Comment line over 100 columns outside tables and fenced blocks; wrapped inside inline code                                        | Note     |

## False positives to skip

- Comment lines that are table rows (`|`), fenced code content, ASCII diagrams, or bare URLs.
- Items marked `#[doc(hidden)]` or intentionally undocumented private helpers with obvious intent.
- Generated code.

## Classification rule

| Situation                                     | Action                                  |
|-----------------------------------------------|-----------------------------------------|
| Doc disagrees with code                       | Fix the doc                             |
| Code disagrees with its own apparent contract | Report as a finding; do not change code |
| Neither code nor doc settles the contract     | Ask, or mark `// TODO(doc): ...`        |
| Style only                                    | Fix the doc                             |

## Known traps

1. **Encoding words.** "sRGB" and "linear" are mutually exclusive; an encoding described with both is a contradiction.
2. **Doctest that passes for the wrong reason.** Assertions must cover the claim, not a subset.
3. **Unit drift.** The same number documented as logical pixels in one item and physical pixels in another.
4. **Silent rename.** A field or method renamed in code while the doc keeps the old term.
5. **Optional results.** `Option` returns never explain when `None` happens.
6. **Panic paths.** `expect`, `unwrap`, indexing, and division can introduce a `# Panics` obligation.
7. **Borrow and ownership claims.** Doc promises a clone, the code returns a reference, or the reverse.
8. **Const claims.** A value described as adjustable but declared `const`.

## Output shape

```text
File:line | Severity | Problem | Evidence | Suggested action
```

Then, for every `Error` and `Warning`, a proposed English replacement in a `text` block. Do not modify files in audit
mode.
