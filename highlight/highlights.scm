;  keywords

[
  "if"
  "else"
  "while"
  "for"
  "in"
  "return"
  "break"
  "continue"
] @keyword.control

[
  "dec"
  "const"
  "fn"
  "arr"
] @keyword.declaration

[
  "int"
  "float"
  "bool"
  "string"
  "char"
  "array"
] @type.builtin

[
  "get"
  "from"
] @keyword.import

[
  "and"
  "or"
] @keyword.operator

;  literals

(integer_literal) @number
(float_literal)   @number.float
(string_literal)  @string
(char_literal)    @string.special.symbol
(bool_literal)    @boolean
(null_literal)    @constant.builtin

;  functions

; function declaration name
(function_declaration
  name: (identifier) @function)

; function call
(call_expression
  function: (identifier) @function.call)

; method call
(method_call_expression
  method: (identifier) @function.method)

; lambda
(lambda_expression) @function

;  variables

(variable_declaration
  name: (identifier) @variable)

(constant_declaration
  name: (identifier) @constant)

(parameter
  name: (identifier) @variable.parameter)

(identifier) @variable

;  operators

[
  "=="
  "!="
  "<"
  "<="
  ">"
  ">="
] @operator.comparison

[
  "="
  "+="
  "-="
  "*="
  "/="
] @operator.assignment

[
  "+"
  "-"
  "*"
  "/"
] @operator.arithmetic

"->" @operator.arrow
".." @operator.range
"::" @operator.path
"!"  @operator.bang
"!#" @operator.bang

;  punctuation

"."  @punctuation.delimiter
","  @punctuation.separator
";"  @punctuation.terminator
":"  @punctuation.delimiter

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket
"{" @punctuation.bracket
"}" @punctuation.bracket

;  comments

(line_comment)  @comment
(block_comment) @comment

;  module paths

(import_statement
  module: (identifier) @namespace)

(path_expression
  segment: (identifier) @namespace)
