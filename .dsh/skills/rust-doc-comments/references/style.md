# Rustdoc Style

Reference for `rust-doc-comments`. Sources: RFC 1574 (API documentation conventions), the Rust API Guidelines
(`C-CRATE-DOC`, `C-EXAMPLE`, `C-FAILURE`, `C-LINK`), and the rustdoc book.

`references/house-rules.md` overrides anything here when the two disagree.

## 1. Line width

- Maximum **100 columns**, counted from column 0 and including the `/// ` or `//! ` prefix.
- Wrap at **90–100**. Do not break early at 80.
- Preferred break points: end of sentence, after a semicolon, after a comma, before a conjunction.
- Never break inside inline code, a link, or a path.
- Exempt from the limit: table rows, fenced code blocks, bare URLs, and single identifiers longer than the limit.

## 2. Item structure

```rust
/// Summary sentence.
///
/// Detail paragraph. Present tense, third person.
///
/// # Examples
///
/// ```
/// use crate::path::Item;
///
/// let value = Item::new(1.0);
/// assert_eq!(value.0, 1.0);
/// ```
pub fn new(value: f32) -> Self { /* ... */ }
```

| Part         | Rule                                                                                     |
|--------------|------------------------------------------------------------------------------------------|
| Summary line | One sentence, capital letter, period, ≤ 100 columns. This is all rustdoc shows in lists. |
| Detail       | Separate paragraph after a blank line.                                                   |
| Sections     | `# Examples`, `# Panics`, `# Errors`, `# Safety` — plural, exact names.                  |
| Markdown     | Standard CommonMark. Code fences need a language tag; use `text` for diagrams.           |

## 3. Sections by item kind

| Item                         | Required                                 | Optional                                             |
|------------------------------|------------------------------------------|------------------------------------------------------|
| Crate (`//!` at crate root)  | One-sentence purpose, layering position  | `# Examples`                                         |
| Module (`//!`)               | Responsibility, terminology, boundary    | `# Units`, `# Data flow`, tables                     |
| Struct / enum                | What it represents, invariants           | `# Examples`, field docs                             |
| Trait                        | Contract and implementor obligations     | `# Examples`, `# Why ...`                            |
| Method returning `Result`    | Purpose                                  | `# Errors` (mandatory when the error is non-obvious) |
| Function that can panic      | Purpose                                  | `# Panics` (mandatory when reachable)                |
| `unsafe fn` / `unsafe trait` | Purpose                                  | `# Safety` (mandatory)                               |
| Field                        | Noun phrase only                         | —                                                    |
| Enum variant                 | Noun phrase, when it is not self-evident | —                                                    |
| Const / associated const     | What the value means                     | `# Examples` when useful                             |

## 4. Sentence patterns

| Situation             | Pattern                                                 |
|-----------------------|---------------------------------------------------------|
| Constructor / factory | `Creates ... from ...`                                  |
| Getter                | `Returns ...`                                           |
| Conversion            | `Converts ... into ...`                                 |
| Read-only view        | `Returns a reference to ...`                            |
| Optional result       | `Returns [`None`] when ...`                             |
| Side effect           | `Takes ... out and leaves ... behind`                   |
| Constraint on caller  | `Must ...`, `Do not ...`                                |
| Invariant             | `The ... is always ...`                                 |
| Non-obvious default   | `Defaults to ...`                                       |
| Internal-only API     | `> **Framework-internal.** Not part of the public API.` |

Rules for sentences:

- Present tense, third person, active voice: `Creates`, `Returns`, `Converts`; never `This creates`, `We create`,
  `I created`.
- Imperative for constraints: `Do not premultiply ...`, `Must not be empty`.
- No contractions: write `Do not`, not `Don't`.
- One idea per sentence.
- Oxford comma in lists of three or more.
- Backticks around identifiers: `` `f32` ``, `` `Option<Size>` ``, `` `loomis_core::geometry::Size` ``.
- Half-width punctuation and spaces around Latin text.

## 5. Links

| Target              | Syntax                          |
|---------------------|---------------------------------|
| Type in scope       | `` [`Element`] ``               |
| Method on a type    | `` [`view`](Component::view) `` |
| Module              | `` [`geometry`] ``              |
| External crate item | `` [`std::sync::Arc`] ``        |

Prefer intra-doc links over plain code spans for anything the reader may want to navigate to. Run
`RUSTDOCFLAGS='-D warnings' cargo doc` to catch broken links.

## 6. Content layering

Information belongs at the highest level where it stays true:

| Information                    | Level           |
|--------------------------------|-----------------|
| Unit of measurement            | Module          |
| Coordinate system / origin     | Module          |
| Value range and clamping       | Type            |
| Alpha / color space convention | Module or type  |
| Which layer owns a conversion  | Module boundary |
| Per-field meaning              | Field           |
| Failure mode of one function   | Method          |

Do not repeat module-level facts on every field. State once, reference implicitly.

## 7. Markdown recipes

Table for parallel items:

```markdown
//! | Type | Role | Lifetime |
//! |---|---|---|
//! | [`Component`] | Reusable unit that owns state | Lives across frames |
```

Diagram in a `text` fence:

```markdown
//! ```text
//! Component (owns state)
//!     │ view()
//!     ▼
//! Element (UI description)
//! ```
```

Note block:

```markdown
/// > **Framework-internal.** Not part of the public API.
/// >
/// > `loomis-runtime` needs the component instance back in order to drive rendering.
```

## 8. Anti-patterns

| Anti-pattern                                     | Why it fails                                         |
|--------------------------------------------------|------------------------------------------------------|
| `/// Returns the width.` on `fn width()`         | Repeats the signature, adds nothing                  |
| `/// An RGBA color.` as the only line            | Misses the invariant that matters                    |
| Repeating `0.0..=1.0` on four fields             | Noise; state it once at type level                   |
| "Currently stores ..." with no update path       | Goes stale silently                                  |
| Mixed languages inside one crate                 | Inconsistent rustdoc and IDE tooltips                |
| Long paragraphs without structure                | Rustdoc renders them as a wall of text               |
| Missing `# Examples` on a non-obvious API        | Users cannot tell how it is meant to be used         |
| Examples whose assertions do not cover the claim | The example passes while documenting the wrong thing |
