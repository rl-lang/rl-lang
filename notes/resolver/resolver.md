# Resolver

## Overview

The resolver sits between parsing and compilation. It transforms string-based `Identifier("x")` references into slot-indexed `ResolvedIdentifier { name, depth, slot }` lookups, so the VM doesn't need to do string lookups at runtime.

Think of it like giving every variable an address. Instead of "find the variable called x", the VM says "give me the value at depth 2, slot 3".

## Entry Point

```rust
// crates/rl-resolver/src/statements.rs:37
pub fn resolve_program(&mut self, ast: Ast, statements: Vec<Statement>) -> Vec<Statement>
```

## How Name Resolution Works

### The Scope Chain

The resolver maintains a stack of scope frames:

```
scopes = [
    ["println", "len", "PI"],     // global scope (stdlib)
    ["x", "y"],                    // function scope
    ["i"],                         // loop scope
]
```

When resolving a name, it walks from innermost to outermost:

```rust
fn resolve_name(&self, name: &str) -> Option<(usize, usize)> {
    for (depth, scope) in self.scopes.iter().rev().enumerate() {
        if let Some(slot) = scope.iter().position(|n| n == name) {
            return Some((depth, slot));
        }
    }
    None
}
```

If `x` is at `scopes[1][0]`, it gets resolved to `depth=1, slot=0`.

### Rewriting Nodes

The resolver walks the AST and rewrites nodes in-place:

```
Identifier("x")  -->  ResolvedIdentifier { name: "x", depth: 1, slot: 0 }
Assign { name: "x", value: ... }  -->  ResolvedAssign { name: "x", depth: 1, slot: 0, value: ... }
VariableDeclaration { name: "x", ... }  -->  ResolvedVariableDeclaration { name: "x", slot: 2, ... }
```

### Declarations

When a new variable is declared, it's added to the current scope:

```rust
fn declare(&mut self, name: &str) -> usize {
    let slot = self.scopes.last_mut().unwrap().len();
    self.scopes.last_mut().unwrap().push(name.to_string());
    slot
}
```

The resolver is careful about ordering:
- Variable initializers are resolved *before* the name is declared (so `dec int x = x + 1` fails)
- Function names are declared *before* the body (so recursion works)

## Import Resolution

Imports are resolved **inline at compile time** - the imported file is read, lexed, parsed, and resolved within the current resolution pass.

```rust
fn load_import_file(&mut self, path: &[String]) -> Result<Vec<Statement>, Error> {
    // 1. Convert path to filesystem path (e.g., ["mymodule"] -> "mymodule.rl")
    // 2. Try direct path, then fall back to "deps/{name}/lib.rl"
    // 3. Check import cache (avoid re-parsing)
    // 4. Read, lex, parse the file
    // 5. Merge the AST into our arena
    // 6. Cache and return
}
```

The resolver tracks which files are currently being imported (`self.importing: HashSet<PathBuf>`) to detect circular imports.

## Scope Creation Points

| Context | What happens |
|---------|--------------|
| Function body | New scope pushed after declaring name (enables recursion) |
| Lambda body | New scope for parameters |
| ForEach/ForRange | Loop variable in its own scope (doesn't leak) |
| While/Loop | Body gets its own scope |
| ConditionalBranch | Scope pushed only if body introduces declarations |
| Match arms | Each arm gets its own scope |
| ImplBlock methods | Each method gets its own scope for parameters |

## Key Data Structures

```rust
pub struct Resolver {
    scopes: Vec<Vec<String>>,      // stack of scope frames
    current_dir: PathBuf,          // working directory for imports
    ast_arena: Ast,                // owns all Expression nodes
    importing: HashSet<PathBuf>,   // circular import guard
    import_cache: HashMap<PathBuf, Vec<Statement>>,  // parsed file cache
}
```

## Key Invariant

The `ExprId` (arena index) of a node never changes during resolution - only its `kind` is mutated in place. This means the resolver can rewrite `Identifier("x")` to `ResolvedIdentifier { ... }` without affecting any other nodes that reference the same expression.
