const C_KEYWORDS: &[&str] = &[
    "auto", "break", "case", "char", "const", "continue", "default", "do", "double", "else",
    "enum", "extern", "float", "for", "goto", "if", "inline", "int", "long", "register",
    "restrict", "return", "short", "signed", "sizeof", "static", "struct", "switch", "typedef",
    "union", "unsigned", "void", "volatile", "while",
];

/// libc/POSIX identifiers a file-scope RL global or function would collide
/// with (e.g. `dec clock = [...]` vs `clock()` from time.h). Locals shadow
/// safely, but globals and functions share the C top-level namespace.
const C_LIBC_RESERVED: &[&str] = &[
    "abort", "abs", "access", "alarm", "assert", "atexit", "atoi", "calloc", "chdir", "chmod",
    "clock", "close", "closedir", "creat", "ctime", "difftime", "div", "dup", "environ", "errno",
    "exit", "fclose", "fcntl", "fflush", "fgets", "fopen", "fork", "free", "fstat", "getenv",
    "getpid", "getcwd", "gmtime", "isatty", "kill", "labs", "lseek", "main", "malloc", "memcpy",
    "memmove", "memset", "mkdir", "mktime", "mmap", "open", "opendir", "perror", "pipe", "poll",
    "printf", "puts", "qsort", "raise", "rand", "read", "realloc", "readdir", "rename", "rmdir",
    "scanf", "setenv", "signal", "sleep", "snprintf", "socket", "sprintf", "srand", "stat",
    "strcat", "strcpy", "strlen", "system", "time", "tmpfile", "unlink", "wait", "write",
];

pub fn mangle(name: &str) -> String {
    if C_KEYWORDS.contains(&name) || C_LIBC_RESERVED.contains(&name) {
        return format!("rl_{}", name);
    }

    let mut result = String::new();
    for (i, ch) in name.chars().enumerate() {
        match ch {
            // The escape marker itself doubles so `U` stays injective.
            'U' => result.push_str("UU"),
            'a'..='z' | 'A'..='Z' | '_' => result.push(ch),
            '0'..='9' if i > 0 => result.push(ch),
            '0'..='9' => {
                result.push_str("rl_");
                result.push(ch);
            }
            '-' | '.' | ':' => result.push('_'),
            // Non-ASCII identifiers (Arabic, CJK, emoji, ...): hex-escape
            // each char so distinct names stay distinct in C.
            _ => result.push_str(&format!("U{:x}", ch as u32)),
        }
    }

    if result.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        result = format!("rl_{}", result);
    }

    result
}

pub fn escape_c_string(s: &str) -> String {
    let mut result = String::new();
    for ch in s.chars() {
        match ch {
            // Basic escapes
            '\n' => result.push_str("\\n"),
            '\t' => result.push_str("\\t"),
            '\r' => result.push_str("\\r"),
            '\\' => result.push_str("\\\\"),
            '"' => result.push_str("\\\""),
            '\'' => result.push_str("\\'"),
            '\0' => result.push_str("\\0"),

            // Control character escapes
            '\x07' => result.push_str("\\a"),
            '\x08' => result.push_str("\\b"),
            '\x0C' => result.push_str("\\f"),
            '\x0B' => result.push_str("\\v"),
            '\x1B' => result.push_str("\\033"),

            // C trigraph safety
            '?' => result.push_str("\\?"),

            // DEL
            '\x7F' => result.push_str("\\x7f"),

            // Other control characters
            c if (c as u32) < 0x20 => {
                result.push_str(&format!("\\x{:02x}", c as u32));
            }

            // Unicode characters (> 0x7F) - emit as raw UTF-8 bytes
            c if (c as u32) > 0x7F => {
                let mut buf = [0u8; 4];
                let utf8 = c.encode_utf8(&mut buf);
                for &byte in utf8.as_bytes() {
                    match byte {
                        b'"' => result.push_str("\\\""),
                        b'\\' => result.push_str("\\\\"),
                        b'?' => result.push_str("\\?"),
                        b if b.is_ascii_graphic() || b == b' ' => {
                            result.push(byte as char);
                        }
                        b => result.push_str(&format!("\\x{:02x}", b)),
                    }
                }
            }

            // Printable ASCII (0x20-0x7E) - pass through
            c => result.push(c),
        }
    }
    result
}

pub fn escape_c_char(ch: char) -> String {
    match ch {
        '\n' => "\\n".to_string(),
        '\t' => "\\t".to_string(),
        '\r' => "\\r".to_string(),
        '\\' => "\\\\".to_string(),
        '\'' => "\\'".to_string(),
        '\0' => "\\0".to_string(),
        '\x07' => "\\a".to_string(),
        '\x08' => "\\b".to_string(),
        '\x0C' => "\\f".to_string(),
        '\x0B' => "\\v".to_string(),
        '\x1B' => "\\033".to_string(),
        '?' => "\\?".to_string(),
        c if (c as u32) < 0x20 || (c as u32) == 0x7F => {
            format!("\\x{:02x}", c as u32)
        }
        c => format!("'{}'", c),
    }
}
