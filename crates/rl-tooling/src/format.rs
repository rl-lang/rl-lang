use rl_lexer::tokentypes::{Token, TokenType, Trivia};

/// Which line the opening `{` of a multiline block goes on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BraceStyle {
    /// `fn f() {` - brace ends the header line.
    SameLine,
    /// `fn f()` newline `{` - brace opens the block on its own line.
    Allman,
}

/// Knobs for the formatter. Defaults encode the house style.
#[derive(Debug, Clone, Copy)]
pub struct FormatOptions {
    /// Spaces per indent level.
    pub indent_width: usize,
    /// Where multiline opening braces go.
    pub brace_style: BraceStyle,
    /// A `fn` signature wider than this (or with more than 3 params)
    /// breaks its parameter list across lines.
    pub fn_sig_width: usize,
    /// A `(...)` / `[...]` group wider than this, with a top-level
    /// comma, breaks one item per line (recursively for nesting).
    pub list_width: usize,
}

impl Default for FormatOptions {
    fn default() -> Self {
        Self {
            indent_width: 4,
            brace_style: BraceStyle::Allman,
            fn_sig_width: 40,
            list_width: 40,
        }
    }
}

pub fn format_tokens(tokens: &[Token]) -> String {
    format_tokens_with(tokens, &FormatOptions::default())
}

pub fn format_tokens_with(tokens: &[Token], opts: &FormatOptions) -> String {
    Formatter::new(*opts).format(tokens)
}

struct Formatter {
    opts: FormatOptions,
    out: String,
    indent: usize,
    at_line_start: bool,
    prev: Option<TokenType>,
    prev_minus_is_unary: bool,
    /// Depth inside `{...}` blocks kept on one line (no newline inside).
    inline_depth: usize,
    /// `fn` name tracking for signature wrapping: 0 = none, 1 = saw `fn`,
    /// 2 = saw `fn` + name (or lambda `fn`, expecting `(`).
    fn_state: u8,
    /// Paren depth of a broken `fn` signature, if its params wrapped.
    sig_break_depth: Option<usize>,
    /// Saw `record`, expecting the name and opening brace.
    record_pending: bool,
    /// Inside a `record { ... }` declaration (fields go one per line).
    in_record: bool,
    /// Brace depth inside the current record declaration.
    record_depth: usize,
    /// The last `}` closed a single-line block (for `} else {` chains).
    just_closed_inline: bool,
    /// Swallow the source newline at exactly this token index (a
    /// structural newline already emitted its line break). Positional
    /// so it can never go stale.
    suppress_newline_at: Option<usize>,
    /// Depth inside `(...)` / `[...]` groups (all of them, broken or not).
    group_depth: usize,
    /// `group_depth` values at which broken list groups opened (innermost
    /// last). A comma breaks iff it sits directly in the top group.
    break_stack: Vec<usize>,
}

impl Formatter {
    fn new(opts: FormatOptions) -> Self {
        Self {
            opts,
            out: String::new(),
            indent: 0,
            at_line_start: true,
            prev: None,
            prev_minus_is_unary: false,
            inline_depth: 0,
            fn_state: 0,
            sig_break_depth: None,
            record_pending: false,
            in_record: false,
            record_depth: 0,
            just_closed_inline: false,
            suppress_newline_at: None,
            group_depth: 0,
            break_stack: Vec::new(),
        }
    }

    fn format(mut self, tokens: &[Token]) -> String {
        let mut idx = 0;
        while idx < tokens.len() {
            idx = self.push_token(tokens, idx);
        }
        self.out
    }

    fn indent_str(&self) -> String {
        " ".repeat(self.opts.indent_width * self.indent)
    }

    /// Whether the token after `idx` carries comment text in its leading
    /// trivia (a comment line whose break the lexer left behind as a
    /// standalone Newline token).
    fn next_has_comment(tokens: &[Token], idx: usize) -> bool {
        tokens.get(idx + 1).is_some_and(|t| {
            t.leading_trivia.iter().any(|tr| {
                matches!(
                    tr,
                    Trivia::LineComment(_) | Trivia::DocComment(_) | Trivia::BlockComment(_)
                )
            })
        })
    }

    /// Gap before leading trivia text: indent at line start, one space
    /// mid-line (inline block comments).
    fn push_trivia_gap(&mut self) {
        if self.at_line_start {
            self.out.push_str(&self.indent_str());
        } else {
            self.out.push(' ');
        }
    }

    /// Lookahead: is the `{` at `open_idx` a single-line block (no newline
    /// or trivia before its match)? Records never qualify - their fields
    /// always expand one per line.
    fn block_is_single_line(&self, tokens: &[Token], open_idx: usize) -> bool {
        if self.record_pending {
            return false;
        }
        let mut depth = 0usize;
        for tok in &tokens[open_idx..] {
            if !tok.leading_trivia.is_empty() || !tok.trailing_trivia.is_empty() {
                return false;
            }
            match &tok.token {
                TokenType::LeftBrace => depth += 1,
                TokenType::RightBrace => {
                    depth -= 1;
                    if depth == 0 {
                        return true;
                    }
                }
                TokenType::Newline => return false,
                TokenType::Eof => return false,
                _ => {}
            }
        }
        false
    }

    /// Lookahead from the `(` at `open_idx`: count top-level params and
    /// approximate the rendered width. Returns `(params, width)`.
    /// Bracket-nested commas (`map[string, int]`) don't count as params.
    fn sig_measure(tokens: &[Token], open_idx: usize) -> (usize, usize) {
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut commas = 0usize;
        let mut width = 0usize;
        let mut saw_token = false;
        for tok in &tokens[open_idx..] {
            match &tok.token {
                TokenType::LeftParen => paren_depth += 1,
                TokenType::RightParen => {
                    paren_depth -= 1;
                    if paren_depth == 0 {
                        break;
                    }
                }
                TokenType::LeftBracket => bracket_depth += 1,
                TokenType::RightBracket => {
                    bracket_depth = bracket_depth.saturating_sub(1);
                }
                TokenType::Comma if paren_depth == 1 && bracket_depth == 0 => commas += 1,
                // already-broken signatures measure the logical line
                TokenType::Newline => {}
                TokenType::Eof => break,
                _ => {}
            }
            width += tok.lexeme.len() + 1;
            saw_token = true;
        }
        let params = if saw_token { commas + 1 } else { 0 };
        (params, width)
    }

    /// Lookahead from a `(` / `[` at `open_idx`: count top-level commas
    /// and approximate the rendered width. Returns `(commas, width)`.
    /// Newlines are skipped so already-broken groups re-measure stably.
    fn group_measure(tokens: &[Token], open_idx: usize) -> (usize, usize) {
        let is_paren = matches!(
            tokens.get(open_idx).map(|t| &t.token),
            Some(TokenType::LeftParen)
        );
        let mut paren_depth = 0usize;
        let mut bracket_depth = 0usize;
        let mut commas = 0usize;
        let mut width = 0usize;
        for tok in &tokens[open_idx..] {
            // top-level = exactly one level inside the opening kind
            let top_level = paren_depth + bracket_depth == 1
                && (is_paren == (paren_depth == 1));
            match &tok.token {
                TokenType::LeftParen => paren_depth += 1,
                TokenType::RightParen => {
                    paren_depth = paren_depth.saturating_sub(1);
                    if is_paren && paren_depth == 0 && bracket_depth == 0 {
                        break;
                    }
                }
                TokenType::LeftBracket => bracket_depth += 1,
                TokenType::RightBracket => {
                    bracket_depth = bracket_depth.saturating_sub(1);
                    if !is_paren && paren_depth == 0 && bracket_depth == 0 {
                        break;
                    }
                }
                TokenType::Comma if top_level => commas += 1,
                TokenType::Newline => {}
                TokenType::Eof => break,
                _ => {}
            }
            width += tok.lexeme.len() + 1;
        }
        (commas, width)
    }

    /// Whether the `(` / `[` at `open_idx` should break one item per
    /// line: has a top-level comma and exceeds `list_width`.
    fn should_break_list(tokens: &[Token], open_idx: usize, opts: &FormatOptions) -> bool {
        let (commas, width) = Self::group_measure(tokens, open_idx);
        commas > 0 && width > opts.list_width
    }

    /// Track `fn name (` sequences for signature wrapping. Runs after
    /// every token except a signature-opening `(` (handled in its arm).
    fn track_fn_state(&mut self, tok: &TokenType) {
        if self.sig_break_depth.is_some() {
            return;
        }
        match tok {
            TokenType::Fn => self.fn_state = 1,
            TokenType::Identifier(_) if self.fn_state == 1 => self.fn_state = 2,
            TokenType::Newline => {}
            _ => self.fn_state = 0,
        }
    }

    fn push_token(&mut self, tokens: &[Token], idx: usize) -> usize {
        let tok = &tokens[idx];

        let mut saw_trivia = false;
        // Leading trivia never emits line breaks: comments are text, and
        // the Newline tokens own line termination. (Emitting breaks here
        // double-counts the comment's own line ending, which the lexer
        // leaves as a standalone Newline token.)
        for t in &tok.leading_trivia {
            saw_trivia = true;
            match t {
                Trivia::LineComment(c) => {
                    self.push_trivia_gap();
                    self.out.push_str("// ");
                    self.out.push_str(c);
                    self.out.push('\n');
                }
                Trivia::DocComment(c) => {
                    self.push_trivia_gap();
                    self.out.push_str("/// ");
                    self.out.push_str(c);
                    self.out.push('\n');
                }
                Trivia::BlockComment(c) => {
                    self.push_trivia_gap();
                    self.out.push_str("/* ");
                    self.out.push_str(c);
                    self.out.push_str(" */\n");
                }
                Trivia::BlankLine => {
                    if !self.in_record {
                        self.out.push('\n');
                    }
                    self.at_line_start = true;
                }
            }
        }
        // `} else` stays inline only when the `}` closed a single-line
        // block with nothing (not even trivia) between the two tokens.
        let inline_close = !saw_trivia && std::mem::take(&mut self.just_closed_inline);

        match &tok.token {
            TokenType::Eof => {}
            TokenType::Newline => {
                if self.suppress_newline_at == Some(idx) {
                    self.suppress_newline_at = None;
                    // swallowed newlines leave all other state untouched
                } else if self.out.is_empty() && Self::next_has_comment(tokens, idx) {
                    // file starts with a comment line: the lexer leaves
                    // the comment's own line break as a bare leading
                    // Newline token, while the text rides forward as
                    // trivia - emitting here would blank the first line.
                } else if self.in_record || self.inline_depth > 0 {
                    // swallowed newlines (structural duplicates, record
                    // and single-line layouts) leave all state untouched
                } else if !self.at_line_start {
                    self.out.push('\n');
                    self.at_line_start = true;
                    self.prev = Some(tok.token.clone());
                } else if !self.out.ends_with("\n\n") {
                    self.out.push('\n');
                    self.prev = Some(tok.token.clone());
                }
            }
            TokenType::RightBrace => {
                if self.inline_depth > 0 {
                    self.inline_depth -= 1;
                    self.just_closed_inline = true;
                    if self.at_line_start {
                        self.out.push_str(&self.indent_str());
                    } else if !matches!(self.prev, Some(TokenType::LeftBrace)) {
                        // `{ style = 0 }` keeps inner spaces; only `{}` is tight
                        self.out.push(' ');
                    }
                    self.out.push('}');
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                } else if self.in_record && self.record_depth == 1 {
                    // closing a record: trailing comma unless empty
                    match self.prev {
                        Some(TokenType::Comma) => {}
                        Some(TokenType::LeftBrace) => {}
                        _ => self.out.push(','),
                    }
                    self.in_record = false;
                    self.record_depth = 0;
                    self.indent = self.indent.saturating_sub(1);
                    if !self.at_line_start {
                        self.out.push('\n');
                    }
                    self.out.push_str(&self.indent_str());
                    self.out.push('}');
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                } else {
                    if self.in_record {
                        self.record_depth = self.record_depth.saturating_sub(1);
                    }
                    self.indent = self.indent.saturating_sub(1);
                    if self.at_line_start {
                        self.out.push_str(&self.indent_str());
                    } else {
                        self.out.push('\n');
                        self.out.push_str(&self.indent_str());
                    }
                    self.out.push('}');
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                }
            }
            TokenType::LeftBrace => {
                if self.record_pending {
                    // empty `record E {}` stays inline
                    if matches!(tokens.get(idx + 1).map(|t| &t.token), Some(TokenType::RightBrace))
                    {
                        self.push_lexeme(tok);
                        self.out.push_str("{}");
                        self.prev = Some(TokenType::RightBrace);
                        self.record_pending = false;
                        self.track_fn_state(&tok.token);
                        return idx + 2;
                    }
                    self.record_pending = false;
                    self.in_record = true;
                    self.record_depth = 1;
                    self.open_block_multiline(idx);
                    self.prev = Some(tok.token.clone());
                } else if self.in_record {
                    self.record_depth += 1;
                    self.open_block_multiline(idx);
                    self.prev = Some(tok.token.clone());
                } else if self.inline_depth > 0 {
                    if self.at_line_start {
                        self.out.push_str(&self.indent_str());
                    } else {
                        self.out.push_str(" {");
                    }
                    self.inline_depth += 1;
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                } else if self.block_is_single_line(tokens, idx) {
                    if self.at_line_start {
                        self.out.push_str(&self.indent_str());
                        self.out.push('{');
                    } else {
                        self.out.push_str(" {");
                    }
                    self.inline_depth = 1;
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                } else {
                    match self.opts.brace_style {
                        BraceStyle::SameLine => {
                            self.out.push_str(" {");
                            self.indent += 1;
                            self.at_line_start = false;
                        }
                        BraceStyle::Allman => self.open_block_multiline(idx),
                    }
                    self.prev = Some(tok.token.clone());
                }
            }
            TokenType::LeftParen => {
                if self.inline_depth == 0 && (self.fn_state == 2 || self.fn_state == 1) {
                    // `fn name (` or lambda `fn (`: maybe wrap the signature
                    let (params, width) = Self::sig_measure(tokens, idx);
                    self.fn_state = 0;
                    if (params > 3 || width > self.opts.fn_sig_width) && params > 0 {
                        self.out.push('(');
                        self.out.push('\n');
                        self.indent += 1;
                        self.out.push_str(&self.indent_str());
                        self.sig_break_depth = Some(1);
                        self.suppress_newline_at = Some(idx + 1);
                        self.at_line_start = false;
                        self.prev = Some(tok.token.clone());
                        self.group_depth += 1;
                    } else {
                        self.push_lexeme(tok);
                        self.group_depth += 1;
                    }
                } else if self.inline_depth == 0
                    && !self.in_record
                    && Self::should_break_list(tokens, idx, &self.opts)
                {
                    // wide call/group: one top-level item per line
                    self.emit_token_text(tok);
                    self.out.push('\n');
                    self.indent += 1;
                    self.group_depth += 1;
                    self.break_stack.push(self.group_depth);
                    self.suppress_newline_at = Some(idx + 1);
                    self.at_line_start = true;
                } else {
                    if let Some(d) = self.sig_break_depth.as_mut() {
                        *d += 1;
                    }
                    self.push_lexeme(tok);
                    self.group_depth += 1;
                }
            }
            TokenType::LeftBracket => {
                if self.inline_depth == 0
                    && !self.in_record
                    && Self::should_break_list(tokens, idx, &self.opts)
                {
                    // wide array/index/type group: one item per line
                    self.emit_token_text(tok);
                    self.out.push('\n');
                    self.indent += 1;
                    self.group_depth += 1;
                    self.break_stack.push(self.group_depth);
                    self.suppress_newline_at = Some(idx + 1);
                    self.at_line_start = true;
                } else {
                    self.push_lexeme(tok);
                    self.group_depth += 1;
                }
            }
            TokenType::RightParen => {
                if let Some(d) = self.sig_break_depth.as_mut() {
                    *d -= 1;
                    if *d == 0 {
                        self.sig_break_depth = None;
                        // `)` stays at param indent; dedent after it
                        if !self.at_line_start {
                            self.out.push('\n');
                        }
                        self.out.push_str(&self.indent_str());
                        self.out.push(')');
                        self.indent = self.indent.saturating_sub(1);
                        self.group_depth = self.group_depth.saturating_sub(1);
                        self.at_line_start = false;
                        self.prev = Some(tok.token.clone());
                        self.track_fn_state(&tok.token);
                        return idx + 1;
                    }
                }
                if self.break_stack.last() == Some(&self.group_depth) {
                    // calls never take a trailing comma (parse error)
                    self.break_stack.pop();
                    if !self.at_line_start {
                        self.out.push('\n');
                    }
                    self.indent = self.indent.saturating_sub(1);
                    self.out.push_str(&self.indent_str());
                    self.out.push(')');
                    self.group_depth = self.group_depth.saturating_sub(1);
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                    self.track_fn_state(&tok.token);
                    return idx + 1;
                }
                self.push_lexeme(tok);
                self.group_depth = self.group_depth.saturating_sub(1);
            }
            TokenType::RightBracket => {
                if self.break_stack.last() == Some(&self.group_depth) {
                    self.break_stack.pop();
                    // arrays take a trailing comma (parses fine)
                    if !matches!(self.prev, Some(TokenType::Comma)) {
                        self.out.push(',');
                    }
                    if !self.at_line_start {
                        self.out.push('\n');
                    }
                    self.indent = self.indent.saturating_sub(1);
                    self.out.push_str(&self.indent_str());
                    self.out.push(']');
                    self.group_depth = self.group_depth.saturating_sub(1);
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                    self.track_fn_state(&tok.token);
                    return idx + 1;
                }
                self.push_lexeme(tok);
                self.group_depth = self.group_depth.saturating_sub(1);
            }
            TokenType::Comma => {
                if self.in_record && self.record_depth == 1 {
                    // the next field (or `}`) indents itself via at_line_start
                    self.out.push(',');
                    self.out.push('\n');
                    self.suppress_newline_at = Some(idx + 1);
                    self.at_line_start = true;
                    self.prev = Some(tok.token.clone());
                } else if self.break_stack.last() == Some(&self.group_depth) {
                    // broken list item: the next item indents itself
                    self.out.push(',');
                    self.out.push('\n');
                    self.suppress_newline_at = Some(idx + 1);
                    self.at_line_start = true;
                    self.prev = Some(tok.token.clone());
                } else {
                    self.push_lexeme(tok);
                }
            }
            TokenType::Else => {
                if self.inline_depth == 0
                    && !self.in_record
                    && !inline_close
                    && matches!(self.prev, Some(TokenType::RightBrace))
                {
                    self.out.push('\n');
                    self.out.push_str(&self.indent_str());
                    self.out.push_str("else");
                    self.at_line_start = false;
                    self.prev = Some(tok.token.clone());
                } else {
                    self.push_lexeme(tok);
                }
            }
            _ => {
                // `record` only arms the record layout when a name (and
                // then `{`) actually follows; anything else disarms it so
                // broken input can't misformat a later block.
                if self.record_pending
                    && !matches!(
                        &tok.token,
                        TokenType::Record | TokenType::Identifier(_) | TokenType::Newline
                    )
                {
                    self.record_pending = false;
                }
                if tok.token == TokenType::Record {
                    self.record_pending = true;
                }
                self.push_lexeme(tok);
            }
        }

        for t in &tok.trailing_trivia {
            match t {
                Trivia::LineComment(c) => {
                    self.out.push_str("// ");
                    self.out.push_str(c);
                }
                Trivia::DocComment(c) => {
                    self.out.push_str("/// ");
                    self.out.push_str(c);
                }
                Trivia::BlockComment(c) => {
                    self.out.push_str("/*");
                    self.out.push_str(c);
                    self.out.push_str("*/");
                }
                Trivia::BlankLine => {}
            }
        }

        self.track_fn_state(&tok.token);
        idx + 1
    }

    /// Emit `{` for a multiline block per the brace style.
    fn open_block_multiline(&mut self, idx: usize) {
        match self.opts.brace_style {
            BraceStyle::SameLine => {
                self.out.push_str(" {");
                self.indent += 1;
                self.at_line_start = false;
            }
            BraceStyle::Allman => {
                if !self.at_line_start {
                    self.out.push('\n');
                }
                self.out.push_str(&self.indent_str());
                self.out.push('{');
                self.out.push('\n');
                self.indent += 1;
                self.suppress_newline_at = Some(idx + 1);
                self.at_line_start = true;
            }
        }
    }

    /// The default token emission: indent, spacing, lexeme, unary `-` state.
    fn push_lexeme(&mut self, tok: &Token) {
        let old_prev = self.prev.clone();
        self.emit_token_text(tok);
        if matches!(tok.token, TokenType::Minus) {
            self.prev_minus_is_unary = !is_value_token(old_prev.as_ref());
        }
    }

    /// Spacing + lexeme + `prev` bookkeeping without the unary-`-` state.
    /// Bracket arms use this directly so depth tracking stays exact.
    fn emit_token_text(&mut self, tok: &Token) {
        if self.at_line_start {
            self.out.push_str(&self.indent_str());
        } else if let Some(p) = &self.prev {
            let space = if matches!(p, TokenType::Minus) && self.prev_minus_is_unary {
                false
            } else {
                needs_space(p, &tok.token)
            };
            if space {
                self.out.push(' ');
            }
        }
        self.out.push_str(&tok.lexeme);
        self.at_line_start = false;
        self.prev = Some(tok.token.clone());
    }
}

/// Whether a space belongs between two adjacent real tokens.
pub fn needs_space(prev: &TokenType, curr: &TokenType) -> bool {
    use TokenType::*;

    // never a space right after these "opening"/tight tokens
    let no_space_after = matches!(
        prev,
        LeftParen | LeftBracket | Dot | DotDot | ColonColon | Hash | Bang | BangHash
    );
    // never a space right before these "closing"/tight tokens
    let no_space_before = matches!(
        curr,
        RightParen
            | RightBracket
            | Dot
            | DotDot
            | ColonColon
            | Comma
            | Colon
            | Semicolon
            | Question
    );

    if no_space_after || no_space_before {
        return false;
    }

    // empty block `{}` stays tight
    if matches!(prev, LeftBrace) && matches!(curr, RightBrace) {
        return false;
    }

    // function calls: identifier (or a value-producing expression)
    // immediately followed by (
    if matches!(curr, LeftParen) && matches!(prev, Identifier(_) | RightParen | RightBracket) {
        return false;
    }

    // index expressions and generic type brackets: `arr[i]`, `arr[string]`,
    // `map[string, int]`, `result[int]`, `set[int]`, etc.
    if matches!(curr, LeftBracket)
        && matches!(
            prev,
            Identifier(_) | RightParen | RightBracket | Array | Map | Set | Result
        )
    {
        return false;
    }

    true
}

/// Whether a token can end an expression / stand in as a value, meaning a
/// `-` immediately following it is a binary (subtraction) operator rather
/// than a unary negation.
fn is_value_token(prev: Option<&TokenType>) -> bool {
    use TokenType::*;
    matches!(
        prev,
        Some(
            Identifier(_)
                | NumberLiteral(_)
                | ByteLiteral(_)
                | StringLiteral(_)
                | CharacterLiteral(_)
                | FloatLiteral(_)
                | BoolLiteral(_)
                | RightParen
                | RightBracket
                | Null
        )
    )
}
