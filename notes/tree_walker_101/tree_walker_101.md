# Tree Walker 101

_before we start: This is a simplified conceptual walkthrough - `rl`-lang's tree walker was the original execution backend, now replaced by the bytecode VM_

## What is a Tree Walker?

A tree walker (or tree-walking interpreter) executes code by walking the AST directly. No compilation step - it reads the tree structure and does what each node says, one at a time.

Think of it like following directions:

```
"Go to the kitchen" -> walk to kitchen
"Open the fridge" -> open fridge
"Take the milk" -> take milk
```

Each instruction is executed immediately as it's encountered.

## How does it work?

The basic loop is:

```rust
fn eval(expr: &Expr) -> Value {
    match expr {
        Expr::Integer(n) => Value::Int(*n),
        Expr::String(s) => Value::Str(s.clone()),
        Expr::Binary { left, op, right } => {
            let l = eval(left);
            let r = eval(right);
            match op {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
            }
        }
        Expr::If { condition, then_branch, else_branch } => {
            if eval(condition).is_truthy() {
                eval(then_branch)
            } else {
                eval(else_branch)
            }
        }
        // ...
    }
}
```

The key insight: **evaluation is the traversal**. To execute `10 + 20 * 30`, you:
1. Visit the `+` node
2. Evaluate the left child (`10`) -> 10
3. Evaluate the right child (`*` node)
4. Visit the `*` node
5. Evaluate its left child (`20`) -> 20
6. Evaluate its right child (`30`) -> 30
7. Multiply: 20 * 30 = 600
8. Back at `+`: 10 + 600 = 610

## Variables and Scope

Variables need somewhere to live. The simplest approach is a symbol table (a map from names to values):

```rust
fn eval(expr: &Expr, env: &mut Environment) -> Value {
    match expr {
        Expr::Identifier(name) => env.get(name),
        Expr::Assign { name, value } => {
            let val = eval(value, env);
            env.set(name, val);
            val
        }
        Expr::Block(statements) => {
            env.push_scope();  // new inner scope
            let mut result = Value::Null;
            for stmt in statements {
                result = exec(stmt, env);
            }
            env.pop_scope();  // destroy inner scope
            result
        }
        // ...
    }
}
```

Scope works like nested boxes - inner boxes can see outer boxes, but not the other way around.

## Functions and Closures

When you call a function, you need to:
1. Save the current environment
2. Create a new environment with the function's parameters
3. Execute the function body
4. Restore the saved environment

```rust
fn call(func: &Function, args: &[Value], env: &Environment) -> Value {
    let mut func_env = Environment::new_enclosing(env); // child scope

    for (param, arg) in func.params.iter().zip(args) {
        func_env.define(param, arg.clone());
    }

    eval_block(&func.body, &mut func_env)
}
```

**Closures** are trickier - they capture variables from the enclosing scope. The function needs to remember the environment it was created in, not the one it's called from.

## Pros and Cons

**Pros:**
- Simple to implement
- Easy to debug (execution follows the source structure)
- Good for small scripts and REPLs

**Cons:**
- Slow (walking the tree is expensive)
- Every node visit has overhead (match, function call, etc.)
- Can't optimize across nodes (no compile step to see the big picture)

## Why use a bytecode VM instead?

A bytecode VM compiles the AST to a flat array of instructions first, then executes those instructions in a tight loop. This is much faster because:
- No tree traversal overhead
- Instructions are just bytes, not heap-allocated nodes
- The compiler can optimize before execution
- The dispatch loop is CPU-friendly (branch prediction, cache locality)

The `rl`-lang project originally used a tree walker, then switched to a bytecode VM for performance.
