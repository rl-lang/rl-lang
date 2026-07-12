//! Sub-scanners for each literal and identifier kind.
//!
//! Each sub-module is an `impl Tokenizer` block that handles one token class:
//! - `character_literal` - single-quoted `'x'` characters
//! - `identifier` - keywords and user-defined names
//! - `number_literal` - integers and floats
//! - `string_literal` - double-quoted `"…"` strings
mod character_literal;
mod identifier;
mod number_literal;
mod string_literal;
