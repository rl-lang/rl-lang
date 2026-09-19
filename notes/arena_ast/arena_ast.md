# Arena AST

## Overview

The `rl`-lang AST uses an arena allocator for expression nodes. Instead of each `Expression` being a separate heap allocation, they're stored in a flat `Vec<Expression>` and referenced by index.

## The Arena

```rust
// crates/rl-ast/src/arena.rs:88
pub struct Arena<T> {
    id: u32,           // unique arena ID
    items: Vec<T>,     // the actual storage
}
```

Each arena gets a unique ID from a global atomic counter. This prevents accidentally using a node from one arena in another arena.

## The Handle

```rust
// crates/rl-ast/src/arena.rs:22
pub struct Id<T> {
    index: u32,        // position in the Vec
    arena_id: u32,     // which arena owns this
    _marker: PhantomData<T>,  // zero-cost type tag
}
```

`Id<T>` is a typed, Copy handle. It's just two integers - cheap to pass around, cheap to store.

```rust
pub type ExprId = Id<Expression>;
```

## Allocation

```rust
pub fn alloc(&mut self, item: T) -> Id<T> {
    let index = self.items.len() as u32;
    self.items.push(item);
    Id::new(index, self.id)
}
```

Just push onto the Vec and return the index. O(1) amortized.

## Access

```rust
pub fn get(&self, id: Id<T>) -> &T {
    self.check_owner(id, "get");
    &self.items[id.index as usize]
}
```

Every access checks that the ID belongs to this arena. If you try to use an ID from arena A to access arena B, it panics immediately with a clear message.

## The Ast Struct

```rust
// crates/rl-ast/src/lib.rs:31
pub struct Ast {
    pub exprs: Arena<Expression>,
    pub program_attributes: Vec<ProgramAttribute>,
}
```

The `Ast` owns the expression arena. Statements are stored separately in a `Vec<Statement>` because they don't need arena allocation (they're already tree-structured through their child expressions).

## Merging Arenas

When the parser processes multiple files (imports), each file gets its own `Ast`. These need to be merged into one.

The merge process:

```
1. Append source arena's items to target arena
2. Calculate offset = target.items.len() before merge
3. Walk every ExprId in both the moved expressions AND the statements
4. For each ExprId: rebase(index + offset, target_arena_id)
```

This rewrites every expression ID to point to the correct position in the merged arena.

```rust
fn rebase(&self, offset: u32, new_arena_id: u32) -> Id<T> {
    Id {
        index: self.index + offset,
        arena_id: new_arena_id,
        _marker: PhantomData,
    }
}
```

## Why Arena Instead of Box?

| Aspect | Box<T> | Arena<T> |
|--------|--------|----------|
| Allocation | One heap alloc per node | One alloc for all nodes |
| Deallocation | Individual drops, possible fragmentation | Drop everything at once |
| Cache locality | Nodes scattered in heap | Nodes contiguous in memory |
| Cloning | Deep tree traversal | Just clone the Vec |
| Borrowing | Can't borrow two siblings | Can borrow any node by index |
| Safety | Manual lifetime management | Arena tracks ownership |

The arena pattern is especially good for ASTs because:
- You create the whole tree at once (parsing)
- You traverse it many times (resolution, compilation)
- You drop it all at once (end of compilation)
- You need to store references between nodes (expressions reference other expressions)

## Example: Building an Expression

```rust
let mut ast = Ast::new();

// Allocate "10 + 20"
let left = ast.alloc_expr(ExpressionKind::Integer(10), span);
let right = ast.alloc_expr(ExpressionKind::Integer(20), span);
let expr = ast.alloc_expr(
    ExpressionKind::Binary {
        left,
        operator: BinaryOp::Add,
        right,
    },
    span,
);

// expr is just an ExprId (two integers)
// The actual nodes live in ast.exprs
```

## Performance Characteristics

- **Allocation**: O(1) amortized (Vec push)
- **Access**: O(1) (Vec index)
- **Merge**: O(n) where n is number of expressions
- **Drop**: O(1) (just drop the Vec)
- **Memory**: Compact, no per-node allocation overhead
