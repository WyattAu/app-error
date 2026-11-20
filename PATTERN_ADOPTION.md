# PATTERN_ADOPTION

app-error (package `error-classify`) declares its relationship to the
WyattAu pattern canon (`WyattAu/pattern-standards`). Validation:
`scripts/adoption-check.py` in the canon repo; format:
`references/adoption-format.md`.

| Pattern | Status | Evidence |
|---|---|---|
| P72 | adopted | src/lib.rs:34 (AppError trait gives every error a uniform shape: HTTP code, recovery class, user message, stable kind) + src/variants.rs:145 (CommonError implements it for all 12 variants) |
| P4 | partial | src/variants.rs:183 (Internal errors render a fixed client-facing message, hiding internal detail; message-level only, no Sensitive<T> wrapper) |

Scope note: this crate is a leaf error-classification library with no
store, network, or deploy surface; baseline-tier patterns outside the
two rows above do not apply. T1 (parse, don't validate) is not
claimed: the crate classifies errors after construction and has no
boundary parsing of its own.
