# Parser 101

_before we start: This is a simplified conceptual walkthrough - `rl`-lang's actual parser differs in several ways_

## What is a Parser?

A parser takes a flat stream of tokens from the lexer and builds a structured tree (AST) that represents the grammatical structure of the code.

The lexer gives us this:

```
Token(Dec) Token(Int) Token(Identifier("x")) Token(Equal) Token(Number(10))
```

The parser turns it into something like:

```
VariableDeclaration {
    name: "x",
    type: Int,
    value: Integer(10)
}
```

## How does it work?

The most common approach is **recursive descent** - you write a function for each grammar rule, and those functions call each other recursively.

Think of it like reading a sentence. When you see "the cat sat on the mat", your brain naturally breaks it down:

```
sentence -> noun_phrase verb_phrase
noun_phrase -> article noun
verb_phrase -> verb noun_phrase
article -> "the"
noun -> "cat" | "mat"
verb -> "sat"
```

Programming languages work the same way, just with stricter rules.

## A Simple Example

Let's say we want to parse arithmetic expressions like `10 + 20 * 30`.

The grammar might look like:

```
expression -> term (('+' | '-') term)*
term       -> factor (('*' | '/') factor)*
factor     -> NUMBER | '(' expression ')'
```

Notice how `expression` calls `term`, which calls `factor`, which can call back to `expression` through parentheses. That's the "recursive" in recursive descent.

In code, this translates almost directly:

```rust
fn parse_expression(tokens: &[Token]) -> Expr {
    let mut left = parse_term(tokens);

    while current_token_is('+') || current_token_is('-') {
        let op = advance();
        let right = parse_term(tokens);
        left = Expr::Binary { left, op, right };
    }

    left
}

fn parse_term(tokens: &[Token]) -> Expr {
    let mut left = parse_factor(tokens);

    while current_token_is('*') || current_token_is('/') {
        let op = advance();
        let right = parse_factor(tokens);
        left = Expr::Binary { left, op, right };
    }

    left
}

fn parse_factor(tokens: &[Token]) -> Expr {
    if current_token_is_number() {
        let value = advance().as_number();
        Expr::Integer(value)
    } else if current_token_is('(') {
        advance(); // consume '('
        let expr = parse_expression(tokens);
        expect(')'); // consume ')'
        expr
    }
}
```

The key insight: **operator precedence is encoded in the call structure**. `expression` calls `term` which calls `factor`, so `*` binds tighter than `+` without needing any special logic.

## Statements vs Expressions

Most languages have two kinds of things:

- **Expressions** produce a value: `10 + 20`, `foo(x)`, `true and false`
- **Statements** do something: `dec int x = 10`, `while (true) { ... }`, `return 5`

A parser typically has separate functions for each:

```rust
fn parse_statement(tokens: &[Token]) -> Statement {
    match current_token() {
        Token::Dec => parse_variable_declaration(tokens),
        Token::While => parse_while_loop(tokens),
        Token::If => parse_if_statement(tokens),
        Token::Fn => parse_function_declaration(tokens),
        // ...
    }
}

fn parse_expression(tokens: &[Token]) -> Expr {
    // handles arithmetic, comparisons, function calls, etc.
}
```

## Error Handling

The simplest approach: stop at the first error.

```rust
fn parse(tokens: &[Token]) -> Result<Ast, Error> {
    let mut statements = Vec::new();

    while !is_at_end() {
        statements.push(parse_statement()?); // ? propagates errors
    }

    Ok(statements)
}
```

More sophisticated parsers can recover from errors and report multiple issues at once, but that's much harder to implement.

## What's Next?

Now that you understand the basics, the actual `rl`-lang parser uses the same ideas but handles a full language with records, tags, match expressions, lambdas, pipe operators, and more.
