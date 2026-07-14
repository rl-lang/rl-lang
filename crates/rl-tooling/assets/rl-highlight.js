(function () {
  "use strict";

  var KEYWORDS = [
    "fn",
    "for",
    "while",
    "return",
    "continue",
    "break",
    "get",
    "from",
    "in",
    "or",
    "and",
    "null",
    "dec",
    "if",
    "else",
    "as",
    "match",
    "CONST",
  ];

  var TYPES = [
    "int",
    "float",
    "bool",
    "string",
    "byte",
    "char",
    "arr",
    "error",
    "result",
  ];

  var LITERALS = ["true", "false", "ok", "err"];

  var KEYWORD_SET = new Set(KEYWORDS);
  var TYPE_SET = new Set(TYPES);
  var LITERAL_SET = new Set(LITERALS);

  var TOKEN_PATTERNS = [
    { name: "comment", regex: /^\/\/[^\n]*/ },
    { name: "string", regex: /^"(?:[^"\\]|\\.)*"/ },
    { name: "char", regex: /^'(?:[^'\\]|\\.)*'/ },
    { name: "number", regex: /^\d+\.\d+|^\d+/ },
    { name: "word", regex: /^[A-Za-z_][A-Za-z0-9_]*/ },
    {
      name: "op",
      regex:
        /^(==|!=|<=|>=|->|=>|\+=|-=|\*=|\/=|::|\.\.|[+\-*/=<>!?&|.,:;(){}\[\]])/,
    },
    { name: "whitespace", regex: /^\s+/ },
  ];

  function classify(word) {
    if (KEYWORD_SET.has(word)) return "rl-kw";
    if (TYPE_SET.has(word)) return "rl-type";
    if (LITERAL_SET.has(word)) return "rl-lit";
    return "rl-ident";
  }

  function escapeHtml(s) {
    return s
      .replace(/&/g, "&amp;")
      .replace(/</g, "&lt;")
      .replace(/>/g, "&gt;")
      .replace(/"/g, "&quot;")
      .replace(/'/g, "&#39;");
  }

  function highlight(source) {
    var out = [];
    var pos = 0;
    var len = source.length;

    while (pos < len) {
      var rest = source.slice(pos);
      var matched = false;

      for (var i = 0; i < TOKEN_PATTERNS.length; i++) {
        var pat = TOKEN_PATTERNS[i];
        var m = pat.regex.exec(rest);
        if (m && m[0].length > 0) {
          var text = m[0];
          var cls;
          if (pat.name === "word") {
            cls = classify(text);
          } else if (pat.name === "whitespace") {
            out.push(escapeHtml(text));
            pos += text.length;
            matched = true;
            break;
          } else {
            cls = "rl-" + pat.name;
          }
          out.push('<span class="' + cls + '">' + escapeHtml(text) + "</span>");
          pos += text.length;
          matched = true;
          break;
        }
      }

      if (!matched) {
        out.push(escapeHtml(source[pos]));
        pos += 1;
      }
    }

    return out.join("");
  }

  function rlHighlightAll() {
    var blocks = document.querySelectorAll("pre.rl-code");
    for (var i = 0; i < blocks.length; i++) {
      var block = blocks[i];
      var source = block.textContent;
      block.innerHTML = highlight(source);
    }
  }

  window.rlHighlightAll = rlHighlightAll;

  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", rlHighlightAll);
  } else {
    rlHighlightAll();
  }
})();
