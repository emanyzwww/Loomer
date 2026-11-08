# House Rules

Project-specific rules for `rust-doc-comments`. These override `references/style.md` whenever the two disagree.
Edit this file freely; the workflow in `SKILL.md` reads it at run time.

## 1. Language

- Source files contain **English only**. Chinese is a review mirror, never written into a `.rs` file.
- The Chinese mirror is delivered in `text` code blocks after the files are written. It does not echo the English text.
- Identifiers, paths, type names, links, and code inside fenced blocks stay English in the mirror.

## 2. Width

- 100 columns maximum, counted from column 0 including the `/// ` / `//! ` prefix.
- Fill the line: wrap at 90–100, never early at 80.
- Exempt: table rows, fenced code block content, bare URLs, single identifiers that cannot be split.
- Never break inside inline code or a link.

## 3. Present state only

Document what the code does **now**.

| Forbidden                                                 | Why                                 |
|-----------------------------------------------------------|-------------------------------------|
| `M1`, `M2`, phase names, milestones                       | Belongs to planning, not to the API |
| "will be added later", "not yet provided", "planned"      | Goes stale; always wrong eventually |
| Placeholder sections for APIs that do not exist           | Documents fiction                   |
| "currently", "for now", "temporarily" as the only framing | Signals churn instead of contract   |

When an API is missing, do not describe its absence. Either it exists and is documented, or it is not mentioned.

## 4. No third-party references

Do not name, cite, compare with, or borrow terminology from other frameworks, UI toolkits, or projects.

- Say what Loomis does, in Loomis terms.
- No "X-style", "similar to Y", "unlike Z".
- Zero external product names in comments.

## 5. Comments that must not be touched

- `//` implementation comments stay as they are.
- Existing `TODO`, `FIXME`, `NOTE` lines stay byte-identical, including their wording and language.
- New uncertainty markers use a plain comment, never a doc comment: `// TODO(doc): <question>`.
- Never convert a `//` comment into a `///` doc comment, or the reverse.

## 6. Internal APIs

Public-but-internal items are documented and marked with a blockquote on the first detail line:

```rust
/// Takes the stored component out and leaves an empty element behind.
///
/// > **Framework-internal.** Not part of the public API.
/// >
/// > `loomis-runtime` needs the component instance back in order to drive rendering.
pub fn take_component(&mut self) -> Option<Box<dyn Component>>
```

Do not add `#[doc(hidden)]`; attributes are code changes.

## 7. Coverage

Document every item in scope, including private ones. Private items matter for readers of the source, even though
rustdoc
hides them by default.

Priority order when the user gives no narrower target:

1. crate root `//!`
2. module `//!`
3. public types and traits
4. public functions and methods
5. fields and enum variants
6. private items that carry non-obvious intent

## 8. Custom sections

Allowed only when the information cannot be expressed by the signature. Keep the set small and consistent:

| Section                                             | Use for                                         |
|-----------------------------------------------------|-------------------------------------------------|
| `# Data flow`                                       | How values move between layers or between types |
| `# Where state lives`                               | Ownership and lifetime of mutable data          |
| `# Why 'static`                                     | Rationale for a bound the reader will question  |
| `# Units`                                           | Measurement unit and where conversion happens   |
| `# Color space`                                     | Encoding and alpha convention                   |
| `# Coordinate system`                               | Origin and axis directions                      |
| `# Panics` / `# Errors` / `# Safety` / `# Examples` | Canonical rustdoc sections                      |

Never invent a section for one sentence. Fold short notes into the detail paragraph.

## 9. Facts stated once

| Fact                                    | Where                   |
|-----------------------------------------|-------------------------|
| Unit (logical pixel)                    | Module                  |
| Coordinate origin and axis direction    | Module                  |
| Which layer converts to physical pixels | Module, at the boundary |
| Value range of a type                   | Type                    |
| Alpha convention                        | Module or type          |
| Purpose of one function                 | Function                |
| Meaning of one field                    | Field, as a noun phrase |

Do not repeat these on every field or method.

## 10. Examples

- Structs and traits that are not obvious: one runnable example at type level.
- Methods: add an example only when usage is non-obvious.
- Every example must compile. Assertions must cover the property the text claims, not a weaker one.
- Diagrams and ASCII art go in a `text` fence.
- Do not add examples that require a window, a GPU, or a running event loop.

## 11. Terminology (use consistently)

| Term               | Meaning in this project                                            |
|--------------------|--------------------------------------------------------------------|
| component          | Long-lived unit that owns state and produces a description         |
| element            | One-shot description of the UI, recreated on every render          |
| layout pass        | Phase that turns constraints into a rectangle per element          |
| drawing pass       | Phase that turns the laid-out tree into primitives                 |
| logical pixel      | Unit of every length; converted to physical pixels at the boundary |
| framework-internal | Public for other crates of this workspace, not for users           |

Prefer these words over synonyms. Do not introduce a second word for the same concept.

## 12. Crate and module docs

- Crate `//!`: one-sentence purpose, position in the layering, conventions that apply crate-wide.
- Module `//!`: responsibility, the types it defines, the invariants it owns.
- Use a Markdown table when the module defines several related types.
- Keep the layering text diagram free of dead links; it is plain text, not links.

## 13. Audit reporting

- The code is the source of truth for documentation.
- If the code looks wrong, report it as a finding and leave it unchanged.
- If behavior cannot be determined, do not guess: mark `// TODO(doc): ...` and list the question in the report.
- If a rule here conflicts with the request, ask once before proceeding.
