//! Identifier and keyword scanner.
//!
//! Consumes a run of alphanumeric/underscore characters and maps the result to
//! the appropriate keyword [`TokenType`] or falls back to [`TokenType::Identifier`].
use crate::{tokenizer::Tokenizer, tokentypes::TokenType};

impl Tokenizer {
    /// Scans an identifier or keyword starting at the current position.
    ///
    /// Consumes alphanumeric characters and underscores, then checks if the
    /// result is a reserved word. If not, emits [`TokenType::Identifier`].
    ///
    /// # Reserved Words
    ///
    /// Each keyword has an optional Arabic alias (separated by `|` in the
    /// match arms). Both spellings produce the same [`TokenType`].
    ///
    /// | Category       | English Keywords                                        | Arabic Aliases |
    /// |----------------|---------------------------------------------------------|----------------|
    /// | Control flow   | `if`, `else`, `for`, `while`, `return`, `break`, `continue` | `إذا`, `وإلا`, `لكل`, `بينما`, `أرجع`, `توقف`, `استمر` |
    /// | Functions      | `fn`                                                    | `دالة`         |
    /// | Imports        | `get`, `from`, `in`                                     | `استورد`, `من`, `في` |
    /// | Logical        | `and`, `or`                                             | `و`, `أو`      |
    /// | Types          | `int`, `float`, `bool`, `string`, `byte`, `char`, `arr`, `error` | `عدد`, `عشري`, `منطقي`, `نص`, `بايت`, `حرف`, `مصفوفة`, `خطأ_` |
    /// | Declarations   | `dec`, `CONST`                                          | `علن`, `ثابت`  |
    /// | Literals       | `true`, `false`, `null`                                 | `صحيح`, `خطأ`, `فارغ` |
    /// | Special        | `as`                                                    | `كـ`           |
    ///
    /// `CONST` in uppercase is intentional.
    pub fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let value: String = self.source[self.start..self.current].iter().collect();

        match value.as_str() {
            "fn" | "دالة" => self.add_token(TokenType::Fn),
            "for" | "لكل" => self.add_token(TokenType::For),
            "while" | "بينما" => self.add_token(TokenType::While),
            "return" | "أرجع" => self.add_token(TokenType::Return),
            "continue" | "استمر" => self.add_token(TokenType::Continue),
            "break" | "توقف" => self.add_token(TokenType::Break),
            "get" | "استورد" => self.add_token(TokenType::Get),
            "from" | "من" => self.add_token(TokenType::From),
            "in" | "في" => self.add_token(TokenType::In),
            "is" => self.add_token(TokenType::Is),
            "or" | "أو" => self.add_token(TokenType::Or),
            "and" | "و" => self.add_token(TokenType::And),
            "null" | "فارغ" => self.add_token(TokenType::Null),
            "int" | "عدد" => self.add_token(TokenType::Int),
            "CONST" | "ثابت" => self.add_token(TokenType::Const),
            "float" | "عشري" => self.add_token(TokenType::Float),
            "bool" | "منطقي" => self.add_token(TokenType::Bool),
            "string" | "نص" => self.add_token(TokenType::String),
            "byte" | "بايت" => self.add_token(TokenType::Byte),
            "char" | "حرف" => self.add_token(TokenType::Char),
            "true" | "صحيح" => self.add_token(TokenType::BoolLiteral(true)),
            "false" | "ليس_صحيح" => self.add_token(TokenType::BoolLiteral(false)),
            "dec" | "أعلن" => self.add_token(TokenType::Dec),
            "if" | "إذا" => self.add_token(TokenType::If),
            "else" | "وإلا" => self.add_token(TokenType::Else),
            "arr" | "مصفوفة" => self.add_token(TokenType::Array),
            "as" | "بصفة" => self.add_token(TokenType::As),
            "error" | "خطأ" => self.add_token(TokenType::Error),
            "result" | "نتيجة" => self.add_token(TokenType::Result),
            "ok" | "نجاح" => self.add_token(TokenType::Ok),
            "err" | "فشل" => self.add_token(TokenType::Err),
            "match" | "طابق" => self.add_token(TokenType::Match),
            "record" | "سجل" => self.add_token(TokenType::Record),
            "impl" | "تنفيذ" => self.add_token(TokenType::Impl),
            "tag" | "وسم" => self.add_token(TokenType::Tag),
            "map" | "خريطة" => self.add_token(TokenType::Map),
            "set" | "مجموعة" => self.add_token(TokenType::Set),
            "loop" | "تكرار" => self.add_token(TokenType::Loop),
            "_" => self.add_token(TokenType::Wildcard),
            "uint" | "عدد_غير_مُوَقَّع" => self.add_token(TokenType::UInt),
            "big" | "كبير" => self.add_token(TokenType::Big),
            "small" | "صغير" => self.add_token(TokenType::Small),
            "sbyte" | "بايت_مُوَقَّع" => self.add_token(TokenType::SByte),
            "handle" | "مقبض" => self.add_token(TokenType::Handle),
            "type" | "نوع" => self.add_token(TokenType::Type),

            &_ => self.add_token(TokenType::Identifier(value)),
        }
    }
}
