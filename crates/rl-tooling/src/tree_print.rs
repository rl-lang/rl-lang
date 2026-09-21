use rl_ast::arena::Arena;
use rl_ast::nodes::{Expression, ExpressionKind};
use rl_ast::statements::{Statement, StatementKind, TypeAnnotation};
use rl_lexer::tokentypes::Token;

pub fn print_tokens(tokens: &[Token]) {
    println!("TokenStream ({} tokens)", tokens.len());
    for (i, tok) in tokens.iter().enumerate() {
        let branch = if i + 1 == tokens.len() { "└── " } else { "├── " };
        println!("{}{:?}", branch, tok);
    }
}

pub fn print_statements(stmts: &[Statement], arena: &Arena<Expression>, label: &str) {
    println!("{} ({} statements)", label, stmts.len());
    print_stmts(stmts, "", arena);
}

fn print_stmts(stmts: &[Statement], prefix: &str, arena: &Arena<Expression>) {
    let n = stmts.len();
    for (i, stmt) in stmts.iter().enumerate() {
        let last = i + 1 == n;
        let conn = if last { "└── " } else { "├── " };
        let child_pfx = if last { "    " } else { "│   " };

        println!("{}{}{}", prefix, conn, stmt_summary(&stmt.kind, arena));

        let children = stmt_children(&stmt.kind, arena);
        for (j, child) in children.iter().enumerate() {
            let child_last = j + 1 == children.len();
            let cc = if child_last { "└── " } else { "├── " };
            match child {
                Child::Line(text) => {
                    println!("{}{}{}{}", prefix, child_pfx, cc, text);
                }
                Child::Branch(text, sub) => {
                    println!("{}{}{}{}", prefix, child_pfx, cc, text);
                    let sub_pfx = format!("{}{}{}", prefix, child_pfx, if child_last { "    " } else { "│   " });
                    for (k, s) in sub.iter().enumerate() {
                        let s_last = k + 1 == sub.len();
                        let sc = if s_last { "└── " } else { "├── " };
                        println!("{}{}{}", sub_pfx, sc, s);
                    }
                }
                Child::Stmts(text, sub) => {
                    println!("{}{}{}{}", prefix, child_pfx, cc, text);
                    let sub_pfx = format!("{}{}{}", prefix, child_pfx, if child_last { "    " } else { "│   " });
                    print_stmts(sub, &sub_pfx, arena);
                }
            }
        }
    }
}

enum Child<'a> {
    Line(String),
    Branch(String, Vec<String>),
    Stmts(String, &'a [Statement]),
}

fn stmt_summary(kind: &StatementKind, arena: &Arena<Expression>) -> String {
    match kind {
        StatementKind::Import { names, wildcard, path } => {
            if *wildcard {
                format!("Import * from {}", path.join("::"))
            } else {
                let display: Vec<String> = names
                    .iter()
                    .map(|(name, alias)| match alias {
                        Some(a) => format!("{name} as {a}"),
                        None => name.clone(),
                    })
                    .collect();
                format!("Import [{}] from {}", display.join(", "), path.join("::"))
            }
        }
        StatementKind::ImportFile { path } =>
            format!("ImportFile \"{}\"", path.join("::")),
        StatementKind::VariableDeclaration { name, type_annotation, value, .. } => {
            let v = expr_short(*value, arena);
            format!("dec {} {} = {}", fmt_type(type_annotation), name, v)
        }
        StatementKind::ResolvedVariableDeclaration { name, type_annotation, .. } =>
            format!("dec {} {} = ...", fmt_type(type_annotation), name),
        StatementKind::ConstantDeclaration { name, type_annotation, .. } =>
            format!("const {} {} = ...", fmt_type(type_annotation), name),
        StatementKind::ResolvedConstantDeclaration { name, type_annotation, .. } =>
            format!("const {} {} = ...", fmt_type(type_annotation), name),
        StatementKind::FunctionDeclaration { name, params, return_type, .. } => {
            let p: Vec<String> = params.iter().map(|p| format!("{}: {}", p.param_name, fmt_type(&p.param_type))).collect();
            format!("fn {}({}) -> {}", name, p.join(", "), fmt_type(return_type))
        }
        StatementKind::ResolvedFunctionDeclaration { name, params, return_type, .. } => {
            let p: Vec<String> = params.iter().map(|p| format!("{}: {}", p.param_name, fmt_type(&p.param_type))).collect();
            format!("fn {}({}) -> {}", name, p.join(", "), fmt_type(return_type))
        }
        StatementKind::Return(_) => "return".into(),
        StatementKind::Break => "break".into(),
        StatementKind::Continue => "continue".into(),
        StatementKind::While { .. } => "while".into(),
        StatementKind::Loop(_) => "loop".into(),
        StatementKind::For { .. } | StatementKind::ResolvedFor { .. } => "for".into(),
        StatementKind::ForRange { variable, .. } | StatementKind::ResolvedForRange { variable, .. } =>
            format!("for {} in ...", variable),
        StatementKind::ForEach { variable, .. } | StatementKind::ResolvedForEach { variable, .. } =>
            format!("for {} in ...", variable),
        StatementKind::Conditional { .. } => "if".into(),
        StatementKind::Match { .. } => "match".into(),
        StatementKind::Expression(_) => "expr;".into(),
        StatementKind::RecordDeclaration { name, fields } => {
            let f: Vec<String> = fields.iter().map(|(n, t)| format!("{}: {}", n, fmt_type(t))).collect();
            format!("record {} {{ {} }}", name, f.join(", "))
        }
        StatementKind::TagDeclaration { name, variants } =>
            format!("tag {} {{ {} }}", name, variants.join(", ")),
        StatementKind::ImplBlock { record, .. } =>
            format!("impl {}", record),
        StatementKind::ResolvedImplBlock { record, .. } =>
            format!("impl {}", record),
        StatementKind::DestructureDeclaration { bindings, .. } => {
            let b: Vec<String> = bindings.iter().map(|(t, n)| format!("{}: {}", fmt_type(t), n)).collect();
            format!("dec ({})", b.join(", "))
        }
        StatementKind::ResolvedDestructureDeclaration { bindings, .. } => {
            let b: Vec<String> = bindings.iter().map(|(t, n)| format!("{}: {}", fmt_type(t), n)).collect();
            format!("dec ({})", b.join(", "))
        }
        StatementKind::Range(_) => "range".into(),
        StatementKind::ConditionalBranch { .. } => "branch".into(),
        StatementKind::ResolvedImportFile { path, .. } =>
            format!("import \"{}\"", path.join("::")),
        StatementKind::ImportFileNamed { path, names } =>
            format!("import {} from {}", names.join(", "), path.join("::")),
        StatementKind::Array { name, type_annotation, .. } =>
            format!("dec {}[] {}", fmt_type(type_annotation), name),
        StatementKind::ConstantArray { name, type_annotation, .. } =>
            format!("const {}[] {}", fmt_type(type_annotation), name),
        StatementKind::ResolvedArray { name, type_annotation, .. } =>
            format!("dec {}[] {}", fmt_type(type_annotation), name),
        StatementKind::ResolvedConstantArray { name, type_annotation, .. } =>
            format!("const {}[] {}", fmt_type(type_annotation), name),
        StatementKind::Map { name, type_annotation, .. } =>
            format!("dec {} {}", fmt_type(type_annotation), name),
        StatementKind::ConstantMap { name, type_annotation, .. } =>
            format!("const {} {}", fmt_type(type_annotation), name),
        StatementKind::ResolvedMap { name, type_annotation, .. } =>
            format!("dec {} {}", fmt_type(type_annotation), name),
        StatementKind::ResolvedConstantMap { name, type_annotation, .. } =>
            format!("const {} {}", fmt_type(type_annotation), name),
        StatementKind::Set { name, type_annotation, .. } =>
            format!("dec {} {}", fmt_type(type_annotation), name),
        StatementKind::ConstantSet { name, type_annotation, .. } =>
            format!("const {} {}", fmt_type(type_annotation), name),
        StatementKind::ResolvedSet { name, type_annotation, .. } =>
            format!("dec {} {}", fmt_type(type_annotation), name),
        StatementKind::ResolvedConstantSet { name, type_annotation, .. } =>
            format!("const {} {}", fmt_type(type_annotation), name),
    }
}

fn stmt_children<'a>(kind: &'a StatementKind, arena: &'a Arena<Expression>) -> Vec<Child<'a>> {
    let mut c = Vec::new();
    match kind {
        StatementKind::VariableDeclaration { value, .. } |
        StatementKind::ResolvedVariableDeclaration { value, .. } => {
            c.push(Child::Branch("value:".into(), vec![expr_tree(&arena.get(*value).kind, arena)]));
        }
        StatementKind::ConstantDeclaration { value, .. } |
        StatementKind::ResolvedConstantDeclaration { value, .. } => {
            c.push(Child::Branch("value:".into(), vec![expr_tree(&arena.get(*value).kind, arena)]));
        }
        StatementKind::FunctionDeclaration { body, params, .. } |
        StatementKind::ResolvedFunctionDeclaration { body, params, .. } => {
            for p in params {
                c.push(Child::Line(format!("param {} {}", fmt_type(&p.param_type), p.param_name)));
            }
            c.push(Child::Stmts("body:".into(), body));
        }
        StatementKind::While { condition, body } => {
            c.push(Child::Line(format!("condition: expr[{:?}]", condition)));
            c.push(Child::Stmts("body:".into(), body));
        }
        StatementKind::Loop(body) => {
            c.push(Child::Stmts("body:".into(), body));
        }
        StatementKind::For { initializer, condition, increment, body } |
        StatementKind::ResolvedFor { initializer, condition, increment, body } => {
            c.push(Child::Branch("init:".into(), vec![stmt_summary(&initializer.kind, arena)]));
            c.push(Child::Line(format!("condition: expr[{:?}]", condition)));
            c.push(Child::Line(format!("increment: expr[{:?}]", increment)));
            c.push(Child::Stmts("body:".into(), body));
        }
        StatementKind::ForEach { iterable, body, .. } |
        StatementKind::ResolvedForEach { iterable, body, .. } => {
            c.push(Child::Line(format!("iterable: expr[{:?}]", iterable)));
            c.push(Child::Stmts("body:".into(), body));
        }
        StatementKind::ForRange { range, body, .. } |
        StatementKind::ResolvedForRange { range, body, .. } => {
            c.push(Child::Branch("range:".into(), vec![stmt_summary(&range.kind, arena)]));
            c.push(Child::Stmts("body:".into(), body));
        }
        StatementKind::Conditional { if_branch, else_branch } => {
            c.push(Child::Branch("if branch:".into(), vec![stmt_summary(&if_branch.kind, arena)]));
            if let Some(eb) = else_branch {
                c.push(Child::Branch("else branch:".into(), vec![stmt_summary(&eb.kind, arena)]));
            }
        }
        StatementKind::Match { arms, .. } => {
            for (i, (pat, body)) in arms.iter().enumerate() {
                let label = match pat {
                    rl_ast::statements::MatchPattern::Literal(eid) =>
                        format!("arm {}: expr[{:?}]", i, eid),
                    rl_ast::statements::MatchPattern::Wildcard =>
                        format!("arm {}: _", i),
                };
                c.push(Child::Stmts(label, body));
            }
        }
        StatementKind::RecordDeclaration { fields, .. } => {
            for (name, ta) in fields {
                c.push(Child::Line(format!("field {} {}", fmt_type(ta), name)));
            }
        }
        StatementKind::ImplBlock { methods, .. } |
        StatementKind::ResolvedImplBlock { methods, .. } => {
            c.push(Child::Stmts("methods:".into(), methods));
        }
        StatementKind::Expression(eid) => {
            let expr = arena.get(*eid);
            c.push(Child::Line(format!("expr: {}", expr_tree(&expr.kind, arena))));
        }
        StatementKind::Return(Some(eid)) => {
            let expr = arena.get(*eid);
            c.push(Child::Line(format!("value: {}", expr_tree(&expr.kind, arena))));
        }
        StatementKind::DestructureDeclaration { value, .. } |
        StatementKind::ResolvedDestructureDeclaration { value, .. } => {
            let expr = arena.get(*value);
            c.push(Child::Line(format!("value: {}", expr_tree(&expr.kind, arena))));
        }
        _ => {}
    }
    c
}

fn expr_short(id: rl_ast::ExprId, arena: &Arena<Expression>) -> String {
    let expr = arena.get(id);
    expr_tree(&expr.kind, arena)
}

fn expr_tree(kind: &ExpressionKind, arena: &Arena<Expression>) -> String {
    match kind {
        ExpressionKind::Null => "null".into(),
        ExpressionKind::Integer(v) => format!("{}", v),
        ExpressionKind::SInt(v) => format!("{}", v),
        ExpressionKind::UInt(v) => format!("{}", v),
        ExpressionKind::SUInt(v) => format!("{}", v),
        ExpressionKind::Float(v) => format!("{}", v),
        ExpressionKind::SFloat(v) => format!("{}", v),
        ExpressionKind::Bool(v) => format!("{}", v),
        ExpressionKind::String(v) => format!("\"{}\"", v),
        ExpressionKind::Character(v) => format!("'{}'", v),
        ExpressionKind::Byte(v) => format!("0x{:02x}", v),
        ExpressionKind::Identifier(name) => name.clone(),
        ExpressionKind::ResolvedIdentifier { name, depth, slot } =>
            format!("{} (d={}, s={})", name, depth, slot),
        ExpressionKind::Binary { left, operator, right } => {
            let l = expr_tree(&arena.get(*left).kind, arena);
            let r = expr_tree(&arena.get(*right).kind, arena);
            format!("({} {:?} {})", l, operator, r)
        }
        ExpressionKind::Unary { operator, operand } => {
            let o = expr_tree(&arena.get(*operand).kind, arena);
            format!("{:?} {}", operator, o)
        }
        ExpressionKind::Grouping(inner) => format!("({})", expr_tree(&arena.get(*inner).kind, arena)),
        ExpressionKind::Call { path, args } => {
            let a: Vec<String> = args.iter().map(|id| expr_tree(&arena.get(*id).kind, arena)).collect();
            format!("{}({})", path.join("::"), a.join(", "))
        }
        ExpressionKind::MethodCall { method, args, .. } => {
            let a: Vec<String> = args.iter().map(|id| expr_tree(&arena.get(*id).kind, arena)).collect();
            format!(".{}({})", method.join("::"), a.join(", "))
        }
        ExpressionKind::CallExpr { callee, args } => {
            let c = expr_tree(&arena.get(*callee).kind, arena);
            let a: Vec<String> = args.iter().map(|id| expr_tree(&arena.get(*id).kind, arena)).collect();
            format!("{}({})", c, a.join(", "))
        }
        ExpressionKind::FieldAccess { target, field } => {
            let t = expr_tree(&arena.get(*target).kind, arena);
            format!("{}.{}", t, field)
        }
        ExpressionKind::Index { target, index } => {
            let t = expr_tree(&arena.get(*target).kind, arena);
            let i = expr_tree(&arena.get(*index).kind, arena);
            format!("{}[{}]", t, i)
        }
        ExpressionKind::IndexAssign { target, index, value } => {
            let t = expr_tree(&arena.get(*target).kind, arena);
            let i = expr_tree(&arena.get(*index).kind, arena);
            let v = expr_tree(&arena.get(*value).kind, arena);
            format!("{}[{}] = {}", t, i, v)
        }
        ExpressionKind::FieldAssign { target, field, value } => {
            let t = expr_tree(&arena.get(*target).kind, arena);
            let v = expr_tree(&arena.get(*value).kind, arena);
            format!("{}.{} = {}", t, field, v)
        }
        ExpressionKind::ResolvedAssign { name, value, .. } => {
            let v = expr_tree(&arena.get(*value).kind, arena);
            format!("{} = {}", name, v)
        }
        ExpressionKind::Assign { name, value } => {
            let v = expr_tree(&arena.get(*value).kind, arena);
            format!("{} = {}", name, v)
        }
        ExpressionKind::ArrayLiteral(elems) => {
            let e: Vec<String> = elems.iter().map(|id| expr_tree(&arena.get(*id).kind, arena)).collect();
            format!("[{}]", e.join(", "))
        }
        ExpressionKind::MapLiteral(pairs) => {
            let p: Vec<String> = pairs.iter().map(|(k, v)| {
                let k_str = expr_tree(&arena.get(*k).kind, arena);
                let v_str = expr_tree(&arena.get(*v).kind, arena);
                format!("{}: {}", k_str, v_str)
            }).collect();
            format!("{{{}}}", p.join(", "))
        }
        ExpressionKind::SetLiteral(elems) => {
            let e: Vec<String> = elems.iter().map(|id| expr_tree(&arena.get(*id).kind, arena)).collect();
            format!("{{{}}}", e.join(", "))
        }
        ExpressionKind::TupleLiteral(elems) => {
            let e: Vec<String> = elems.iter().map(|id| expr_tree(&arena.get(*id).kind, arena)).collect();
            format!("({})", e.join(", "))
        }
        ExpressionKind::StructLiteral { name, fields } => {
            let f: Vec<String> = fields.iter().map(|(n, id)| {
                format!("{}: {}", n, expr_tree(&arena.get(*id).kind, arena))
            }).collect();
            format!("{} {{ {} }}", name, f.join(", "))
        }
        ExpressionKind::EnumVariant { enum_name, variant } =>
            format!("{}.{}", enum_name, variant),
        ExpressionKind::Cast { value, target_type } => {
            let v = expr_tree(&arena.get(*value).kind, arena);
            format!("{} as {}", v, fmt_type(target_type))
        }
        ExpressionKind::Propagate(inner) => {
            let v = expr_tree(&arena.get(*inner).kind, arena);
            format!("{}?", v)
        }
        ExpressionKind::OkLiteral(inner) => {
            let v = expr_tree(&arena.get(*inner).kind, arena);
            format!("ok({})", v)
        }
        ExpressionKind::ErrLiteral(inner) => {
            let v = expr_tree(&arena.get(*inner).kind, arena);
            format!("err({})", v)
        }
        ExpressionKind::ErrorLiteral(inner) => {
            let v = expr_tree(&arena.get(*inner).kind, arena);
            format!("error({})", v)
        }
        ExpressionKind::Lambda { params, .. } | ExpressionKind::ResolvedLambda { params, .. } => {
            let p: Vec<String> = params.iter().map(|p| p.param_name.clone()).collect();
            format!("fn({})", p.join(", "))
        }
        _ => format!("{:?}", std::mem::discriminant(kind)),
    }
}

fn fmt_type(t: &TypeAnnotation) -> String {
    match t {
        TypeAnnotation::Int => "int".into(),
        TypeAnnotation::UInt => "uint".into(),
        TypeAnnotation::SInt => "sint".into(),
        TypeAnnotation::SUInt => "suint".into(),
        TypeAnnotation::Float => "float".into(),
        TypeAnnotation::SFloat => "sfloat".into(),
        TypeAnnotation::Bool => "bool".into(),
        TypeAnnotation::String => "string".into(),
        TypeAnnotation::Char => "char".into(),
        TypeAnnotation::Byte => "byte".into(),
        TypeAnnotation::SByte => "sbyte".into(),
        TypeAnnotation::BByte => "bbyte".into(),
        TypeAnnotation::BSByte => "bsbyte".into(),
        TypeAnnotation::Null => "null".into(),
        TypeAnnotation::Infer => "_".into(),
        TypeAnnotation::Error => "error".into(),
        TypeAnnotation::Fn => "fn".into(),
        TypeAnnotation::Array(inner) => format!("arr[{}]", fmt_type(inner)),
        TypeAnnotation::CArray(inner) => format!("c_arr[{}]", fmt_type(inner)),
        TypeAnnotation::Map(k, v) => format!("map[{}, {}]", fmt_type(k), fmt_type(v)),
        TypeAnnotation::CMap(k, v) => format!("c_map[{}, {}]", fmt_type(k), fmt_type(v)),
        TypeAnnotation::Set(inner) => format!("set[{}]", fmt_type(inner)),
        TypeAnnotation::CSet(inner) => format!("c_set[{}]", fmt_type(inner)),
        TypeAnnotation::Result(inner) => format!("result[{}]", fmt_type(inner)),
        TypeAnnotation::CResult(inner) => format!("c_result[{}]", fmt_type(inner)),
        TypeAnnotation::Tuple(elems) => {
            let s: Vec<String> = elems.iter().map(fmt_type).collect();
            format!("({})", s.join(", "))
        }
        TypeAnnotation::CTuple(elems) => {
            let s: Vec<String> = elems.iter().map(fmt_type).collect();
            format!("({})", s.join(", "))
        }
        TypeAnnotation::Record(name) => name.clone(),
        TypeAnnotation::CRecord(name) => name.clone(),
        TypeAnnotation::Enum(name) => name.clone(),
        TypeAnnotation::CEnum(name) => name.clone(),
        TypeAnnotation::Generic(name) => format!("<{}>", name),
        TypeAnnotation::Callback(params, ret) => {
            let p: Vec<String> = params.iter().map(fmt_type).collect();
            format!("fn({}) -> {}", p.join(", "), fmt_type(ret))
        }
        TypeAnnotation::Handle(h) => format!("handle({:?})", h),
        TypeAnnotation::HandleInfer => "handle(_)".into(),
        TypeAnnotation::CError => "c_error".into(),
        _ => format!("{:?}", t),
    }
}
