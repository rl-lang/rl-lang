# AST

## Overview

The AST (Abstract Syntax Tree) is the structured representation of your code after parsing. It's called "abstract" because it throws away unnecessary details (whitespace, comments, parentheses) and keeps only the essential structure.

For `dec int x = 10 + 20`, the AST looks something like:

```
Statement: VariableDeclaration
  name: "x"
  type: Int
  value: Expression: Binary
    left: Expression: Integer(10)
    operator: Add
    right: Expression: Integer(20)
```

## Expression Kinds

The `rl`-lang AST has these expression types:

### Literals
```
Null, Integer(i64), Float(f64), String(String), Bool(bool), Character(char)
```
Plus sized variants: `SInt(i32)`, `UInt(u64)`, `SUInt(u32)`, `Byte(u8)`, `SByte(i8)`, `BByte(u16)`, `BSByte(i16)`, `SFloat(f32)`

### Identifiers
```
Identifier(String)              // unresolved: just the name
ResolvedIdentifier { name, depth, slot }  // resolved: address in scope chain
```

### Operations
```
Binary { left, operator, right }   // 10 + 20
Unary { operator, operand }        // -x, !true
Grouping(expr)                     // (10 + 20)
Cast { value, target_type }        // 10 as float
```

### Collections
```
ArrayLiteral(vec![...])     // [1, 2, 3]
MapLiteral(vec![(k, v)])    // {"a": 1}
SetLiteral(vec![...])       // {1, 2, 3}
TupleLiteral(vec![...])     // (1, "hello")
```

### Calls and Methods
```
Call { path, args }           // foo(1, 2)
MethodCall { caller, method, args }  // obj.method(args)
CallExpr { callee, args }     // resolved: callee is ResolvedIdentifier
```

### Access
```
Index { target, index }          // arr[0]
IndexAssign { target, index, value }  // arr[0] = 10
FieldAccess { target, field }    // record.field
FieldAssign { target, field, value }  // record.field = 10
```

### Functions
```
Lambda { params, return_type, body }  // |x| x + 1
```

### Records and Tags
```
StructLiteral { name, fields }       // Point { x: 10, y: 20 }
EnumVariant { enum_name, variant }   // Color.Red
```

### Error Handling
```
OkLiteral(expr)      // ok(value)
ErrLiteral(expr)     // err(message)
ErrorLiteral(expr)   // error(message)
Propagate(expr)      // value?
```

## Statement Kinds

### Declarations
```
VariableDeclaration { name, type_annotation, value }
ConstantDeclaration { name, type_annotation, value }
FunctionDeclaration { name, params, return_type, body, attribute }
```

Plus resolved variants with slot indices, and array/map/set variants.

### Control Flow
```
While { condition, body }
Loop { body }
For { init, condition, update, body }
ForRange { variable, start, end, body }
ForEach { variable, iterable, body }
Conditional { if_branch, else_branch }
Match { value, arms }
```

### Other
```
Import { path }
ImportFile { path }
RecordDeclaration { name, fields }
TagDeclaration { name, variants }
ImplBlock { record_name, methods }
Return(value)
Break
Continue
Expression(expr)
```

## Type Annotations

Type annotations are enums with 40+ variants covering all type combinations:

```
Int, Float, String, Bool, Char, Null
Array(inner), Map(key, value), Set(inner), Tuple(types)
Record(name), Enum(name), Result(inner, error)
Fn, Infer, Generic(name)
Handle, HandleInfer
```

Plus constant variants (CInt, CFloat, etc.) for `const` declarations.

## The Arena Pattern

Instead of heap-allocating each expression node individually, `rl`-lang stores them in a flat `Vec` called an arena. Expressions are referenced by index (`ExprId`) rather than pointer.

This has several advantages:
- **Cache-friendly**: Nodes are contiguous in memory
- **No garbage collection**: Everything is dropped at once when the arena is dropped
- **Cheap cloning**: Just copy the Vec, no deep tree traversal
- **Safe**: The arena tracks ownership, so you can't use a node from one arena in another
