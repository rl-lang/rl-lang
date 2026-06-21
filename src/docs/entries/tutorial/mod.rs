use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static STEP_COMMENTS: ConceptEntry = ConceptEntry {
    name: "1. comments",
    descriptions: &[DescriptionEntry {
        description: "single-line comments start with `//`, everything after is ignored",
        examples: &["// this is a comment\ndec int x = 10 // inline comment"],
    }],
};

pub static STEP_VARIABLES: ConceptEntry = ConceptEntry {
    name: "2. variables",
    descriptions: &[
        DescriptionEntry {
            description: "declare a mutable variable with `dec <type> <name> = <value>`",
            examples: &[
                "dec bool   is_ready = true\ndec int    count    = 1\ndec string label    = \"rl\"\ndec float  ratio    = 1.5\ndec char   letter   = 'x'",
            ],
        },
        DescriptionEntry {
            description: "reassign with `=`, mutate in place with `+=`, `-=`, `*=`, `/=`",
            examples: &[
                "dec int count = 1\ncount += 4 // 5\ncount -= 2 // 3\ncount *= 3 // 9\ncount /= 3 // 3",
            ],
        },
    ],
};

pub static STEP_CONSTANTS: ConceptEntry = ConceptEntry {
    name: "3. constants",
    descriptions: &[
        DescriptionEntry {
            description: "declare an immutable value with `CONST <type> <name> = <value>`, convention is UPPER_CASE",
            examples: &[
                "CONST int MAX_RETRIES = 3\nCONST string LANG = \"rl\"\nprintln(MAX_RETRIES, LANG) // 3 rl",
            ],
        },
        DescriptionEntry {
            description: "constant arrays use `CONST arr[<type>]`",
            examples: &["CONST arr[int] PRIMES = [2, 3, 5, 7, 11]\nprintln(PRIMES[0]) // 2"],
        },
    ],
};

pub static STEP_TYPES: ConceptEntry = ConceptEntry {
    name: "4. types",
    descriptions: &[
        DescriptionEntry {
            description: "rl is statically typed: `int`, `float`, `bool`, `string`, `char`",
            examples: &[
                "dec int    x = 42\ndec float  y = 3.14\ndec bool   b = true\ndec string s = \"hello\"\ndec char   c = 'a'",
            ],
        },
        DescriptionEntry {
            description: "std::types covers checking (`is_int`, `is_bool`, ...) and conversion (`to_int`, `to_float`, ...)",
            examples: &[
                "get is_int, to_int, to_string from std::types\n\nprintln(is_int(42))      // true\nprintln(to_int(\"0xff\"))  // 255\nprintln(to_string(42))   // \"42\"",
            ],
        },
        DescriptionEntry {
            description: "base conversions: `to_bin`, `to_hex`, `to_oct`",
            examples: &[
                "get to_bin, to_hex, to_oct from std::types\n\nprintln(to_bin(10)) // \"1010\"\nprintln(to_hex(255)) // \"ff\"\nprintln(to_oct(8)) // \"10\"",
            ],
        },
    ],
};

pub static STEP_ARRAYS: ConceptEntry = ConceptEntry {
    name: "5. arrays",
    descriptions: &[
        DescriptionEntry {
            description: "typed, zero-indexed, mutable — declare with `dec arr[<type>] <name> = [<items>]`",
            examples: &[
                "dec arr[int] nums = [10, 20, 30]\nprintln(nums[0]) // 10\nnums[1] = 99\nprintln(nums) // [10, 99, 30]",
            ],
        },
        DescriptionEntry {
            description: "nested arrays are supported",
            examples: &["dec arr[arr[int]] matrix = [[1, 2], [3, 4]]\nprintln(matrix[0][1]) // 2"],
        },
        DescriptionEntry {
            description: "all elements must share the same type",
            examples: &[
                "dec arr[int] nums = [1, 2, 3]\n// dec arr[int] bad = [1, \"two\"] // error: type mismatch",
            ],
        },
    ],
};

pub static STEP_OPERATORS: ConceptEntry = ConceptEntry {
    name: "6. operators",
    descriptions: &[
        DescriptionEntry {
            description: "arithmetic (`+ - * /`), grouping with `()` controls order",
            examples: &["dec int x = (2 + 3) * 4 // 20\ndec int y = 2 + 3 * 4   // 14"],
        },
        DescriptionEntry {
            description: "comparison (`== != < <= > >=`) always returns bool, logical `!` negates",
            examples: &[
                "println(5 == 5, 5 != 3, 3 < 10) // true true true\nprintln(!true) // false",
            ],
        },
        DescriptionEntry {
            description: "method-style call with `.` passes the value as the first argument",
            examples: &[
                "get arr_push from std::array\ndec arr[int] nums = [1, 2, 3]\nprintln(nums.arr_push(4)) // [1, 2, 3, 4]",
            ],
        },
    ],
};

pub static STEP_CONTROL_FLOW: ConceptEntry = ConceptEntry {
    name: "7. control flow",
    descriptions: &[
        DescriptionEntry {
            description: "`if` / `else if` / `else` branch on conditions",
            examples: &[
                "dec int score = 75\nif (score >= 90) {\n    println(\"A\")\n} else if (score >= 75) {\n    println(\"B\")\n} else {\n    println(\"F\")\n}",
            ],
        },
        DescriptionEntry {
            description: "`while` loops as long as the condition is true",
            examples: &["dec int i = 0\nwhile (i < 3) {\n    println(i)\n    i += 1\n}"],
        },
        DescriptionEntry {
            description: "`break` exits a loop early, `continue` skips to the next iteration",
            examples: &[
                "dec int i = 0\nwhile (i < 5) {\n    i += 1\n    if (i == 3) { continue }\n    println(i) // 1 2 4 5\n}",
            ],
        },
    ],
};

pub static STEP_LOOPS: ConceptEntry = ConceptEntry {
    name: "8. for loops",
    descriptions: &[
        DescriptionEntry {
            description: "C-style for loop: explicit init, condition, increment",
            examples: &[
                "dec int sum = 0\nfor [int j = 1, j < 6, j += 1] {\n    sum += j\n}\nprintln(sum) // 15",
            ],
        },
        DescriptionEntry {
            description: "range-based for loop: `for <var> in <start>..<end>` (exclusive)",
            examples: &[
                "dec int product = 1\nfor k in 1..5 {\n    product *= k\n}\nprintln(product) // 24",
            ],
        },
        DescriptionEntry {
            description: "iterable for loop walks an array directly",
            examples: &[
                "dec arr[int] evens = [2, 4, 6]\ndec int even_sum = 0\nfor n in evens {\n    even_sum += n\n}\nprintln(even_sum) // 12",
            ],
        },
    ],
};

pub static STEP_FUNCTIONS: ConceptEntry = ConceptEntry {
    name: "9. functions",
    descriptions: &[
        DescriptionEntry {
            description: "declare with `fn <name>(<type> <param>, ...) -> <type> { <body> }`",
            examples: &[
                "fn add(int a, int b) -> int {\n    return a + b\n}\nprintln(add(3, 4)) // 7",
            ],
        },
        DescriptionEntry {
            description: "functions are first-class values and can be stored in variables",
            examples: &[
                "fn double(int x) -> int {\n    return x * 2\n}\ndec fn f = double\nprintln(f(5)) // 10",
            ],
        },
    ],
};

pub static STEP_LAMBDAS: ConceptEntry = ConceptEntry {
    name: "10. lambdas",
    descriptions: &[
        DescriptionEntry {
            description: "anonymous functions, defined inline with `fn(<type> <param>, ...) { <body> }`",
            examples: &[
                "dec fn square = fn(int x) -> int {\n    return x * x\n}\nprintln(square(5)) // 25",
            ],
        },
        DescriptionEntry {
            description: "closures capture variables from the surrounding scope",
            examples: &[
                "dec int factor = 3\ndec fn triple = fn(int x) -> int {\n    return x * factor\n}\nprintln(triple(4)) // 12",
            ],
        },
    ],
};

pub static STEP_NULL: ConceptEntry = ConceptEntry {
    name: "11. null",
    descriptions: &[DescriptionEntry {
        description: "`null` represents the absence of a value; functions with no return implicitly return null",
        examples: &[
            "dec int x = null\nprintln(x) // null\n\nfn do_nothing() {\n    // implicitly returns null\n}",
        ],
    }],
};

pub static STEP_IMPORTS: ConceptEntry = ConceptEntry {
    name: "12. imports",
    descriptions: &[
        DescriptionEntry {
            description: "import a single stdlib function with `get std::<module>::<function>`",
            examples: &["get std::math::sqrt\n\nsqrt(9.0) // 3.0"],
        },
        DescriptionEntry {
            description: "import multiple stdlib functions with `get <fn1>, <fn2> from std::<module>`",
            examples: &["get sin, cos from std::math\n\nsin(0.0) // 0.0\ncos(0.0) // 1.0"],
        },
        DescriptionEntry {
            description: "import a local file with `get <filename>`, or named items with `get <fn> from <path>::<file>`",
            examples: &[
                "get utils // loads utils.rl",
                "get add from math::utils // imports add from math/utils.rl",
            ],
        },
    ],
};

pub static STEP_ENTRY_POINTS: ConceptEntry = ConceptEntry {
    name: "13. entry points",
    descriptions: &[
        DescriptionEntry {
            description: "source files work as scripts when no entry function is present",
            examples: &[
                "get println from std::io\n\nprintln(\"hello\") // runs top to bottom, no main() needed",
            ],
        },
        DescriptionEntry {
            description: "if a file declares `fn main()`, `rl run` registers declarations and runs `main()` instead",
            examples: &["fn main() {\n    std::io::println(\"hello\")\n}"],
        },
        DescriptionEntry {
            description: "a different zero-argument function can be selected as the entry point with `!#[entry]`",
            examples: &["!#[entry]\nfn start() {\n    std::io::println(\"hello\")\n}"],
        },
    ],
};

pub static STEP_STDLIB_MATH: ConceptEntry = ConceptEntry {
    name: "14. stdlib: math",
    descriptions: &[
        DescriptionEntry {
            description: "core math: rounding, powers, roots, primes, sequences",
            examples: &[
                "get factorial, is_prime, fibonacci, pow, sqrt, mod from std::math\n\nprintln(factorial(10)) // 3628800\nprintln(fibonacci(15)) // 610\nprintln(is_prime(97))  // true\nprintln(pow(2, 2))     // 4.0\nprintln(sqrt(4))       // 2.0\nprintln(mod(10, 3))    // 1",
            ],
        },
        DescriptionEntry {
            description: "trig functions operate in radians; `degrees`/`radians` convert between units",
            examples: &[
                "get sin, cos, degrees, radians from std::math\n\nprintln(sin(0.0))         // 0.0\nprintln(cos(0.0))         // 1.0\nprintln(degrees(3.14159)) // 180.0",
            ],
        },
        DescriptionEntry {
            description: "math constants live in std::math::consts and are called like functions",
            examples: &[
                "get PI, TAU, PHI, E from std::math::consts\n\nprintln(PI())  // ~3.14159\nprintln(TAU()) // ~6.283\nprintln(PHI()) // ~1.618\nprintln(E())   // ~2.718",
            ],
        },
    ],
};

pub static STEP_STDLIB_ARRAY: ConceptEntry = ConceptEntry {
    name: "15. stdlib: array",
    descriptions: &[
        DescriptionEntry {
            description: "basic manipulation: push, pop, slice, sort, unique",
            examples: &[
                "get arr_push, arr_sort, arr_slice, arr_unique from std::array\n\nprintln(arr_push([1, 2], 3))         // [1, 2, 3]\nprintln(arr_sort([3, 1, 2]))         // [1, 2, 3]\nprintln(arr_slice([1, 2, 3, 4], 1, 3)) // [2, 3]\nprintln(arr_unique([1, 2, 2, 3, 1])) // [1, 2, 3]",
            ],
        },
        DescriptionEntry {
            description: "aggregates: sum, max, min, product, count",
            examples: &[
                "get arr_sum, arr_max, arr_min, arr_product from std::array\n\nprintln(arr_sum([1, 2, 3, 4]))     // 10\nprintln(arr_max([3, 1, 4, 1, 5]))  // 5\nprintln(arr_min([3, 1, 4, 1, 5]))  // 1\nprintln(arr_product([1, 2, 3, 4])) // 24",
            ],
        },
        DescriptionEntry {
            description: "higher-order functions take a predicate or callback lambda",
            examples: &[
                "get arr_filter, arr_map, arr_reduce from std::array\n\nprintln(arr_filter([1, 2, 3, 4], fn(int x) -> bool { return x > 2 })) // [3, 4]\nprintln(arr_map([1, 2, 3], fn(int x) -> int { return x * 2 }))        // [2, 4, 6]\nprintln(arr_reduce([1, 2, 3, 4], fn(int acc, int x) -> int { return acc + x }, 0)) // 10",
            ],
        },
    ],
};

pub static STEP_STDLIB_STR: ConceptEntry = ConceptEntry {
    name: "16. stdlib: str",
    descriptions: &[
        DescriptionEntry {
            description: "inspecting and searching strings",
            examples: &[
                "get contains, starts_with, ends_with, index_of from std::str\n\nprintln(contains(\"hello\", \"ell\"))    // true\nprintln(starts_with(\"hello\", \"he\"))  // true\nprintln(index_of(\"hello\", \"ll\"))     // 2",
            ],
        },
        DescriptionEntry {
            description: "transforming strings",
            examples: &[
                "get to_upper, trim, replace, split, join from std::str\n\nprintln(\"  hi  \".trim().to_upper()) // HI\nprintln(replace(\"foo bar foo\", \"foo\", \"baz\")) // baz bar baz\nprintln(split(\"a,b,c\", \",\")) // [\"a\", \"b\", \"c\"]\nprintln(join([\"a\", \"b\", \"c\"], \"-\")) // a-b-c",
            ],
        },
        DescriptionEntry {
            description: "`format` substitutes `{}` placeholders in order",
            examples: &[
                "get format from std::str\n\nprintln(format(\"{} is {}\", \"age\", 30)) // age is 30",
            ],
        },
    ],
};

pub static STEP_STDLIB_IO: ConceptEntry = ConceptEntry {
    name: "17. stdlib: io",
    descriptions: &[
        DescriptionEntry {
            description: "console output with `print`/`println`, input with `read`/`read_int`/`read_float`",
            examples: &[
                "get println, read from std::io\n\nprintln(\"hello\")\ndec string name = read(\"enter your name: \")",
            ],
        },
        DescriptionEntry {
            description: "file I/O: reading, writing, and appending",
            examples: &[
                "get write_file, read_file, append_file from std::io\n\nwrite_file(\"index.html\", \"<p>hello</p>\")\nappend_file(\"info.txt\", \"name: Mohamed\")\ndec string data = read_file(\"backup_info.txt\")",
            ],
        },
    ],
};

pub static STEP_STDLIB_PATH: ConceptEntry = ConceptEntry {
    name: "18. stdlib: path",
    descriptions: &[DescriptionEntry {
        description: "build and inspect filesystem paths without touching the filesystem",
        examples: &[
            "get path_join, path_extension, path_stem, path_filename from std::path\n\nprintln(path_join(\"src\", \"main.rl\")) // \"src/main.rl\"\nprintln(path_extension(\"main.rl\"))    // \"rl\"\nprintln(path_stem(\"main.rl\"))         // \"main\"\nprintln(path_filename(\"/usr/bin/rl\")) // \"rl\"",
        ],
    }],
};

pub static STEP_STDLIB_FS: ConceptEntry = ConceptEntry {
    name: "19. stdlib: fs",
    descriptions: &[
        DescriptionEntry {
            description: "inspect files and directories",
            examples: &[
                "get path_exists, file_size, list_dir from std::fs\n\nprintln(path_exists(\"./Cargo.toml\")) // true\nprintln(file_size(\"./Cargo.toml\"))   // 215\nprintln(list_dir(\"./src\"))           // [\"./src/main.rl\", ...]",
            ],
        },
        DescriptionEntry {
            description: "create, copy, move, and clean up",
            examples: &[
                "get mkdir_all, copy_file, move_file, rmdir_all from std::fs\n\nmkdir_all(\"./build/assets/css\")\ncopy_file(\"a.txt\", \"b.txt\") // 1024 (bytes copied)\nmove_file(\"/tmp/a.txt\", \"/tmp/b.txt\")\nrmdir_all(\"./build\")",
            ],
        },
    ],
};

pub static STEP_STDLIB_RANDOM: ConceptEntry = ConceptEntry {
    name: "20. stdlib: random",
    descriptions: &[
        DescriptionEntry {
            description: "random ints, floats, and bools",
            examples: &[
                "get rand_int_range, rand_float, rand_bool from std::random\n\nprintln(rand_int_range(1, 6)) // e.g. 4\nprintln(rand_float())         // e.g. 0.3528\nprintln(rand_bool())          // e.g. true",
            ],
        },
        DescriptionEntry {
            description: "dice, array choice, and shuffling",
            examples: &[
                "get rand_dice, rand_choice, rand_shuffle from std::random\n\nprintln(rand_dice(6))               // e.g. 5\nprintln(rand_choice([1, 2, 3]))     // e.g. 2\nprintln(rand_shuffle([1, 2, 3, 4])) // e.g. [3, 1, 4, 2]",
            ],
        },
    ],
};
