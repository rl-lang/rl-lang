# rl-lang Parser

## Overview

The `rl`-lang parser is a hand-written recursive descent parser with precedence climbing for expressions. It lives in `crates/rl-parser/src/`.

## Entry Point

```rust
// crates/rl-parser/src/parser_logic.rs:54
pub fn parse(tokens: Vec<Token>, source_file: SourceFile) -> Result<(Ast, Vec<Statement>), Error>
```

Takes tokens from the lexer, returns an arena-allocated AST and a list of top-level statements. Stops at the first error (no error recovery).

## Expression Precedence

Expressions use precedence climbing via the call stack - each function handles one precedence level:

```
parse_pipe          |>  (lowest precedence)
  parse_logical     and, or
    parse_equality  ==, !=
      parse_comparison  <, <=, >, >=, +=, -=, *=, /=
        parse_term      +, -
          parse_factor  *, /
            parse_unary  !, - (prefix)
              parse_primary  literals, identifiers, calls, arrays, maps, ...
                parse_postfix  .method(), as Type, ? (highest precedence)
```

## Statement Dispatch

`parse_statement_to_ast()` in `statements/mod.rs:63` dispatches on the current keyword:

| Token | Handler |
|-------|---------|
| `get` | `parse_import()` |
| `dec` | `parse_variable_declaration()` or `parse_infer_declaration()` |
| `const` | `parse_const_declaration()` |
| `while` | `parse_while()` |
| `loop` | `parse_loop()` |
| `for` | `parse_for()` |
| `if` | `parse_if()` |
| `fn` | `parse_function()` |
| `match` | `parse_match()` |
| `record` | `parse_record_declaration()` |
| `tag` | `parse_tag_declaration()` |
| `impl` | `parse_impl_block()` |
| `return` | inline |
| `break` | inline |
| `continue` | inline |

## Key Data Structures

```rust
// crates/rl-parser/src/parser_logic.rs:28
pub struct Parser {
    pub source_file: SourceFile,
    pub tokens: Vec<Token>,
    pub current: usize,                    // read cursor
    pub ast_arena: Ast,                    // arena for expressions
    pub record_names: HashSet<String>,     // known record types
    pub tag_names: HashSet<String>,        // known tag types
}
```

## Cursor Primitives

| Method | Purpose |
|--------|---------|
| `peek()` | Current token without consuming |
| `advance()` | Consume and return current token |
| `check(token_type)` | Test if current matches (without consuming) |
| `match_type(types)` | Try match + advance, returns bool |
| `previous()` | Most recently consumed token |
| `is_at_end()` | Check for Eof |

## Error Handling

No error recovery - first error stops parsing. Every parse function returns `Result<T, Error>` and uses `?` to propagate.

## Special Features

- **Newline skipping**: Newlines are skipped liberally, allowing significant-newline-style formatting
- **Optional semicolons**: Trailing `;` after statements is silently consumed
- **Record/tag disambiguation**: The parser tracks known record and tag names to distinguish `Name { ... }` (struct literal) from block expressions, and `Name.Variant` (enum variant) from field access
- **Checkpoint-and-rewind**: `parse_variable_declaration` saves and restores the cursor position to disambiguate tuple/destructure declarations from scalar declarations
