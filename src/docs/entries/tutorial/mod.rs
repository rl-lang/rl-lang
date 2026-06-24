//! Tutorial documentation entries (beginner, advanced, etc.).
use crate::docs::entry::{ConceptEntry, DescriptionEntry};

pub static STEP_FIRST_PROGRAM: ConceptEntry = ConceptEntry {
    name: "1. your first program",
    descriptions: &[
        DescriptionEntry {
            description: "programming is just giving a computer instructions, one line at a time. your first instruction: print something to the screen. println prints a value followed by a newline",
            examples: &["println(\"hello, world\")"],
        },
        DescriptionEntry {
            description: "you can print numbers and expressions too, not just strings",
            examples: &["println(42)\nprintln(1 + 1)"],
        },
        DescriptionEntry {
            description: "comments let you leave notes in your code. the computer ignores everything after //",
            examples: &["// this prints a greeting\nprintln(\"hello\") // inline comment"],
        },
        DescriptionEntry {
            description: "print does the same as println but does not add a newline at the end, so the next output appears on the same line",
            examples: &[
                "get print from std::io\n\nprint(\"hello \")\nprint(\"world\")\n// output: hello world",
            ],
        },
        DescriptionEntry {
            description: "exercise: print a welcome message for your guessing game. it should greet the player and tell them what the game is about\n\nexpected output:\n  welcome to the guessing game!\n  i am thinking of a number between 1 and 100\n  can you guess it?",
            examples: &[
                "// your game starts here\nprintln(\"welcome to the guessing game!\")\nprintln(\"i am thinking of a number between 1 and 100\")\nprintln(\"can you guess it?\")",
            ],
        },
    ],
};

pub static STEP_VARIABLES: ConceptEntry = ConceptEntry {
    name: "2. storing values",
    descriptions: &[
        DescriptionEntry {
            description: "a variable is a named box that holds a value. you declare one with dec, then its type, then its name, then its value. rl needs to know the type upfront and it never changes",
            examples: &[
                "dec int    score  = 0\ndec string name   = \"Mohamed\"\ndec bool   active = true\ndec float  ratio  = 1.5",
            ],
        },
        DescriptionEntry {
            description: "once declared you can read a variable by name, and reassign it with =",
            examples: &["dec int lives = 3\nprintln(lives) // 3\n\nlives = 2\nprintln(lives) // 2"],
        },
        DescriptionEntry {
            description: "for numbers you can use += -= *= /= to update a variable in place instead of writing the full reassignment",
            examples: &[
                "dec int score = 0\nscore += 10  // score is now 10\nscore += 5   // score is now 15\nscore -= 3   // score is now 12\nprintln(score) // 12",
            ],
        },
        DescriptionEntry {
            description: "constants are like variables but they can never be reassigned. declare with CONST. by convention use UPPER_CASE names",
            examples: &[
                "CONST int    MAX_SCORE = 100\nCONST string LANG      = \"rl\"\n\nprintln(MAX_SCORE) // 100\n// MAX_SCORE = 200  // error: cannot assign to constant",
            ],
        },
        DescriptionEntry {
            description: "exercise: store the game configuration in variables and constants. the secret number will come from random later, so use a placeholder for now\n\nexpected output:\n  secret number is: 42\n  you have 10 attempts",
            examples: &[
                "CONST int MAX_ATTEMPTS = 10\nCONST int MIN_NUMBER   = 1\nCONST int MAX_NUMBER   = 100\n\ndec int secret_number = 42 // placeholder until we learn random\ndec int attempts_left = MAX_ATTEMPTS\n\nprintln(\"secret number is: \") // just for testing\nprintln(secret_number)\nprintln(\"you have \")\nprintln(attempts_left)\nprintln(\"attempts\")",
            ],
        },
    ],
};

pub static STEP_TYPES: ConceptEntry = ConceptEntry {
    name: "3. types",
    descriptions: &[
        DescriptionEntry {
            description: "every value in rl has a type. the type tells rl what kind of data it is and what you can do with it. you already saw int, float, bool, string. here is what each one means",
            examples: &[
                "dec int    x = 42       // whole numbers, positive or negative\ndec float  y = 3.14     // numbers with a decimal point\ndec bool   b = true     // either true or false, nothing else\ndec string s = \"hello\"  // text, always in double quotes\ndec char   c = 'a'      // a single character, always in single quotes",
            ],
        },
        DescriptionEntry {
            description: "rl will not let you mix types. assigning the wrong type is caught before your program even runs",
            examples: &[
                "dec int x = 10\n// x = \"hello\"  // error: expected int, got string\n// x = 3.14    // error: expected int, got float",
            ],
        },
        DescriptionEntry {
            description: "byte is a special type for small unsigned integers from 0 to 255. plain number literals like 1, 42, 100 are bytes by default. they widen to int automatically when you assign them to an int variable",
            examples: &[
                "dec byte small = 200    // stays as byte\ndec int  big   = 200    // byte literal 200 widens to int\n\n// byte + byte = byte\n// byte + int  = int",
            ],
        },
        DescriptionEntry {
            description: "exercise: look at your game variables from the previous step. what type should each one be? write a comment next to each variable explaining your choice",
            examples: &[
                "CONST int MAX_ATTEMPTS = 10   // int: whole number, no decimals needed\nCONST int MIN_NUMBER   = 1    // int: a position in a range\nCONST int MAX_NUMBER   = 100  // int: a position in a range\n\ndec int  secret_number = 42   // int: we compare it with guesses\ndec int  attempts_left = 10   // int: a counter we will decrease",
            ],
        },
    ],
};

pub static STEP_IO: ConceptEntry = ConceptEntry {
    name: "4. talking to the user",
    descriptions: &[
        DescriptionEntry {
            description: "so far your program only talks at the user. to make it interactive you need to read input. read waits for the user to type something and press enter, then gives you back what they typed as a string",
            examples: &[
                "get read from std::io\n\ndec string name = read(\"what is your name? \")\nprintln(\"hello \")\nprintln(name)",
            ],
        },
        DescriptionEntry {
            description: "read_int and read_float do the same but convert the input to a number for you. use these when you expect the user to type a number",
            examples: &[
                "get read_int from std::io\n\ndec int age = read_int(\"how old are you? \")\nprintln(\"you are \")\nprintln(age)\nprintln(\" years old\")",
            ],
        },
        DescriptionEntry {
            description: "to build nicer output you can use concat from std::str to join multiple values into one string before printing",
            examples: &[
                "get concat from std::str\n\ndec string name = \"Mohamed\"\ndec int    age  = 25\nprintln(concat(\"hello \", name, \", you are \", age, \" years old\"))",
            ],
        },
        DescriptionEntry {
            description: "exercise: ask the player for a guess and echo it back. do not worry about checking if it is correct yet\n\nexpected output:\n  enter your guess: 50\n  you guessed: 50",
            examples: &[
                "get read_int from std::io\nget concat   from std::str\n\ndec int guess = read_int(\"enter your guess: \")\nprintln(concat(\"you guessed: \", guess))",
            ],
        },
    ],
};

pub static STEP_OPERATORS_AND_DECISIONS: ConceptEntry = ConceptEntry {
    name: "5. making decisions",
    descriptions: &[
        DescriptionEntry {
            description: "comparison operators compare two values and produce a bool. you use these to ask questions about your data",
            examples: &[
                "println(5 == 5)   // true  (equal)\nprintln(5 != 3)   // true  (not equal)\nprintln(3 < 10)   // true  (less than)\nprintln(10 > 3)   // true  (greater than)\nprintln(3 <= 3)   // true  (less than or equal)\nprintln(10 >= 10) // true  (greater than or equal)",
            ],
        },
        DescriptionEntry {
            description: "if runs a block of code only when a condition is true. else runs when it is false. you can chain as many else if branches as you need",
            examples: &[
                "dec int score = 75\n\nif (score >= 90) {\n    println(\"excellent\")\n} else if (score >= 60) {\n    println(\"good\")\n} else {\n    println(\"keep trying\")\n}",
            ],
        },
        DescriptionEntry {
            description: "! flips a bool. true becomes false, false becomes true",
            examples: &[
                "dec bool ready = false\n\nif (!ready) {\n    println(\"not ready yet\")\n}",
            ],
        },
        DescriptionEntry {
            description: "arithmetic works as you expect. parentheses control the order of evaluation",
            examples: &[
                "dec int a = (2 + 3) * 4  // 20\ndec int b = 2 + 3 * 4    // 14 (multiplication first)\ndec int c = 10 / 2       // 5",
            ],
        },
        DescriptionEntry {
            description: "exercise: add a guess check to your game. compare the guess to the secret number (still hardcoded as 42) and tell the player if they guessed too low, too high, or correctly\n\nexpected output (if guess is 30):\n  enter your guess: 30\n  too low!\n\nexpected output (if guess is 42):\n  enter your guess: 42\n  correct!",
            examples: &[
                "get read_int from std::io\nget concat   from std::str\n\ndec int secret = 42\ndec int guess  = read_int(\"enter your guess: \")\n\nif (guess < secret) {\n    println(\"too low!\")\n} else if (guess > secret) {\n    println(\"too high!\")\n} else {\n    println(\"correct!\")\n}",
            ],
        },
    ],
};

pub static STEP_LOOPS: ConceptEntry = ConceptEntry {
    name: "6. repeating things",
    descriptions: &[
        DescriptionEntry {
            description: "a while loop runs its block over and over as long as its condition stays true. as soon as the condition becomes false the loop stops",
            examples: &[
                "dec int count = 3\n\nwhile (count > 0) {\n    println(count)\n    count -= 1\n}\n// 3\n// 2\n// 1",
            ],
        },
        DescriptionEntry {
            description: "break exits the loop immediately no matter what the condition says. use it when something happens inside the loop that means you are done",
            examples: &[
                "dec int i = 0\n\nwhile (true) {\n    println(i)\n    i += 1\n    if (i == 3) { break } // stops after printing 0, 1, 2\n}",
            ],
        },
        DescriptionEntry {
            description: "continue skips the rest of the current iteration and jumps back to the condition check",
            examples: &[
                "dec int i = 0\n\nwhile (i < 5) {\n    i += 1\n    if (i == 3) { continue } // skip 3\n    println(i)\n}\n// 1\n// 2\n// 4\n// 5",
            ],
        },
        DescriptionEntry {
            description: "exercise: wrap your guess check in a loop so the player keeps guessing until they get it right or run out of attempts. decrease attempts_left each round and break when they win or lose\n\nexpected output:\n  attempts left: 10\n  enter your guess: 30\n  too low!\n  attempts left: 9\n  enter your guess: 42\n  correct! you got it!",
            examples: &[
                "get read_int from std::io\nget concat   from std::str\n\nCONST int MAX_ATTEMPTS = 10\ndec int secret         = 42\ndec int attempts_left  = MAX_ATTEMPTS\n\nwhile (attempts_left > 0) {\n    println(concat(\"attempts left: \", attempts_left))\n    dec int guess = read_int(\"enter your guess: \")\n    attempts_left -= 1\n\n    if (guess < secret) {\n        println(\"too low!\")\n    } else if (guess > secret) {\n        println(\"too high!\")\n    } else {\n        println(\"correct! you got it!\")\n        break\n    }\n}\n\nif (attempts_left == 0) {\n    println(concat(\"out of attempts! the number was \", secret))\n}",
            ],
        },
    ],
};

pub static STEP_FOR_LOOPS: ConceptEntry = ConceptEntry {
    name: "7. for loops",
    descriptions: &[
        DescriptionEntry {
            description: "while loops are great when you do not know how many times you will loop. when you do know, a for loop is cleaner. the range form goes from a start number up to but not including the end",
            examples: &["for i in 0..5 {\n    println(i)\n}\n// 0\n// 1\n// 2\n// 3\n// 4"],
        },
        DescriptionEntry {
            description: "the C-style for loop gives you full control: an initializer, a condition, and an increment, all in one line",
            examples: &[
                "for [int i = 1, i <= 5, i += 1] {\n    println(i)\n}\n// 1\n// 2\n// 3\n// 4\n// 5",
            ],
        },
        DescriptionEntry {
            description: "you can also loop over the elements of an array directly without needing an index at all",
            examples: &[
                "dec arr[string] days = [\"sat\", \"sun\", \"mon\"]\n\nfor day in days {\n    println(day)\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: before the game starts, print a row of dashes as a divider using a for loop. then after the game ends print a summary showing each attempt number\n\nexpected output:\n  ----------\n  welcome to the guessing game!\n  ----------",
            examples: &[
                "get print from std::io\n\n// print 10 dashes without a newline each\nfor i in 0..10 {\n    print(\"-\")\n}\nprintln(\"\") // move to next line\n\nprintln(\"welcome to the guessing game!\")\n\nfor i in 0..10 {\n    print(\"-\")\n}\nprintln(\"\")",
            ],
        },
    ],
};

pub static STEP_FUNCTIONS: ConceptEntry = ConceptEntry {
    name: "8. grouping logic into functions",
    descriptions: &[
        DescriptionEntry {
            description: "a function is a named block of code you can call by name. instead of copying the same lines everywhere you write them once as a function and call it wherever you need it",
            examples: &[
                "fn greet(string name) {\n    println(\"hello \")\n    println(name)\n}\n\ngreet(\"Ali\")\ngreet(\"Sara\")",
            ],
        },
        DescriptionEntry {
            description: "functions can give back a value with return. you declare what type they return with -> after the parameter list",
            examples: &[
                "fn add(int a, int b) -> int {\n    return a + b\n}\n\ndec int result = add(3, 4)\nprintln(result) // 7",
            ],
        },
        DescriptionEntry {
            description: "functions can call themselves, this is called recursion. each call works on a smaller version of the problem until it hits a base case that stops the recursion",
            examples: &[
                "fn countdown(int n) {\n    if (n == 0) {\n        println(\"go!\")\n        return\n    }\n    println(n)\n    countdown(n - 1)\n}\n\ncountdown(3)\n// 3\n// 2\n// 1\n// go!",
            ],
        },
        DescriptionEntry {
            description: "exercise: pull the hint logic and the divider printer out of your game loop into their own functions. your main game loop should just call them\n\nwrite:\n  fn print_divider() - prints a row of dashes\n  fn check_guess(int guess, int secret) -> string - returns \"low\", \"high\", or \"correct\"",
            examples: &[
                "get print   from std::io\nget concat  from std::str\n\nfn print_divider() {\n    for i in 0..20 {\n        print(\"-\")\n    }\n    println(\"\")\n}\n\nfn check_guess(int guess, int secret) -> string {\n    if (guess < secret) { return \"low\" }\n    if (guess > secret) { return \"high\" }\n    return \"correct\"\n}\n\n// in your game loop:\n// dec string result = check_guess(guess, secret)\n// if (result == \"correct\") { ... }",
            ],
        },
    ],
};

pub static STEP_ARRAYS: ConceptEntry = ConceptEntry {
    name: "9. collecting data",
    descriptions: &[
        DescriptionEntry {
            description: "an array holds multiple values of the same type in a sequence. you declare one with dec arr[type] and access elements by index starting from zero",
            examples: &[
                "dec arr[int] scores = [10, 20, 30]\n\nprintln(scores[0]) // 10\nprintln(scores[2]) // 30\n\nscores[1] = 99\nprintln(scores) // [10, 99, 30]",
            ],
        },
        DescriptionEntry {
            description: "arr_push adds an element to the end and gives you back the updated array. arr_pop removes the last element. these are in std::array",
            examples: &[
                "get arr_push, arr_pop from std::array\n\ndec arr[int] nums = [1, 2, 3]\nnums = arr_push(nums, 4)\nprintln(nums) // [1, 2, 3, 4]\n\nnums = arr_pop(nums)\nprintln(nums) // [1, 2, 3]",
            ],
        },
        DescriptionEntry {
            description: "len returns the number of elements in an array. arr_contains tells you if a value is in the array",
            examples: &[
                "get len, arr_contains from std::array\n\ndec arr[string] names = [\"ali\", \"sara\", \"omar\"]\nprintln(len(names))               // 3\nprintln(arr_contains(names, \"sara\")) // true",
            ],
        },
        DescriptionEntry {
            description: "you can loop over an array with for ... in to visit every element",
            examples: &[
                "dec arr[int] guesses = [30, 60, 42]\n\nfor g in guesses {\n    println(g)\n}\n// 30\n// 60\n// 42",
            ],
        },
        DescriptionEntry {
            description: "exercise: track every guess the player makes in an array. at the end of the game print the full guess history\n\nexpected output:\n  your guesses: [30, 60, 42]",
            examples: &[
                "get arr_push      from std::array\nget read_int      from std::io\nget concat        from std::str\n\ndec arr[int] guesses = []\n\n// inside your game loop, after reading a guess:\n// guesses = arr_push(guesses, guess)\n\n// after the game:\nprintln(concat(\"your guesses: \", guesses))",
            ],
        },
    ],
};

pub static STEP_STDLIB: ConceptEntry = ConceptEntry {
    name: "10. the standard library",
    descriptions: &[
        DescriptionEntry {
            description: "rl comes with a standard library of useful functions organized into modules. you import what you need with get. you have already used std::io and std::str. here are the ones most useful for your game",
            examples: &[
                "get rand_int_range from std::random\nget concat, format from std::str\nget arr_push, len  from std::array",
            ],
        },
        DescriptionEntry {
            description: "std::random gives you unpredictable numbers. rand_int_range returns a random int between two values inclusive. this is how your game will pick the secret number",
            examples: &[
                "get rand_int_range from std::random\n\ndec int secret = rand_int_range(1, 100)\nprintln(secret) // different every run",
            ],
        },
        DescriptionEntry {
            description: "std::str has format which works like concat but uses {} as placeholders. cleaner for building messages with multiple values",
            examples: &[
                "get format from std::str\n\ndec string name = \"Mohamed\"\ndec int    score = 95\nprintln(format(\"hello {}, your score is {}\", name, score))\n// hello Mohamed, your score is 95",
            ],
        },
        DescriptionEntry {
            description: "std::array has arr_sum, arr_min, arr_max for number arrays. useful for showing stats about the player's guesses at the end",
            examples: &[
                "get arr_sum, arr_min, arr_max from std::array\n\ndec arr[int] guesses = [30, 60, 42]\nprintln(arr_min(guesses)) // 30\nprintln(arr_max(guesses)) // 60\nprintln(arr_sum(guesses)) // 132",
            ],
        },
        DescriptionEntry {
            description: "exercise: replace the hardcoded secret number with rand_int_range. then at the end of the game show the player their lowest guess, highest guess, and total number of guesses using arr_min, arr_max, and len\n\nexpected output:\n  game over!\n  total guesses: 3\n  lowest guess:  30\n  highest guess: 60",
            examples: &[
                "get rand_int_range        from std::random\nget arr_min, arr_max, len from std::array\nget format                from std::str\n\ndec int secret = rand_int_range(1, 100)\n\n// ... game loop ...\n\nprintln(\"game over!\")\nprintln(format(\"total guesses: {}\", len(guesses)))\nprintln(format(\"lowest guess:  {}\", arr_min(guesses)))\nprintln(format(\"highest guess: {}\", arr_max(guesses)))",
            ],
        },
    ],
};

pub static STEP_LAMBDAS: ConceptEntry = ConceptEntry {
    name: "11. functions as values",
    descriptions: &[
        DescriptionEntry {
            description: "in rl functions are values just like numbers and strings. you can store a function in a variable with dec fn and call it through that variable",
            examples: &[
                "fn double(int x) -> int {\n    return x * 2\n}\n\ndec fn f = double\nprintln(f(5)) // 10",
            ],
        },
        DescriptionEntry {
            description: "a lambda is an anonymous function defined inline without a name. useful when you need a short function just once and do not want to name it",
            examples: &[
                "dec fn square = fn(int x) -> int {\n    return x * x\n}\n\nprintln(square(4)) // 16",
            ],
        },
        DescriptionEntry {
            description: "lambdas capture variables from the surrounding scope automatically",
            examples: &[
                "dec int base = 10\n\ndec fn add_base = fn(int x) -> int {\n    return x + base\n}\n\nprintln(add_base(5))  // 15\nprintln(add_base(20)) // 30",
            ],
        },
        DescriptionEntry {
            description: "the real power of lambdas is passing them to functions like arr_map and arr_filter which apply your function to every element of an array",
            examples: &[
                "get arr_map, arr_filter from std::array\n\ndec arr[int] nums   = [1, 2, 3, 4, 5, 6]\ndec arr[int] evens  = arr_filter(nums, fn(int x) -> bool { return x > 3 })\ndec arr[int] doubled = arr_map(evens, fn(int x) -> int { return x * 2 })\nprintln(doubled) // [8, 10, 12]",
            ],
        },
        DescriptionEntry {
            description: "exercise: use arr_filter and a lambda to count how many of the player's guesses were below the secret number, and how many were above\n\nexpected output:\n  guesses below: 2\n  guesses above: 1",
            examples: &[
                "get arr_filter, len from std::array\nget format          from std::str\n\n// assume secret and guesses exist from the game\ndec arr[int] below = arr_filter(guesses, fn(int g) -> bool { return g < secret })\ndec arr[int] above = arr_filter(guesses, fn(int g) -> bool { return g > secret })\n\nprintln(format(\"guesses below: {}\", len(below)))\nprintln(format(\"guesses above: {}\", len(above)))",
            ],
        },
    ],
};

pub static STEP_NULL: ConceptEntry = ConceptEntry {
    name: "12. null",
    descriptions: &[
        DescriptionEntry {
            description: "null means the absence of a value. a variable can hold null regardless of its declared type. functions that do not explicitly return anything return null implicitly",
            examples: &[
                "dec int x = null\nprintln(x) // null\n\nfn do_nothing() {\n    // returns null implicitly\n}",
            ],
        },
        DescriptionEntry {
            description: "use is_null from std::types to check if a value is null before using it. this avoids surprises when a function might return nothing",
            examples: &[
                "get is_null from std::types\n\nfn find_even(arr[int] nums) -> int {\n    for n in nums {\n        if (n > 10) { return n }\n    }\n    return null // nothing matched\n}\n\ndec int result = find_even([1, 2, 3])\n\nif (is_null(result)) {\n    println(\"nothing found\")\n} else {\n    println(result)\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: write a function find_first_correct that takes the guesses array and the secret number and returns the index of the first correct guess, or null if the player never guessed correctly\n\nhint: use a for loop with a counter variable alongside it",
            examples: &[
                "get is_null from std::types\n\nfn find_first_correct(arr[int] guesses, int secret) -> int {\n    dec int i = 0\n    for g in guesses {\n        if (g == secret) { return i }\n        i += 1\n    }\n    return null\n}\n\ndec int idx = find_first_correct(guesses, secret)\n\nif (is_null(idx)) {\n    println(\"you never guessed correctly\")\n} else {\n    println(format(\"you got it on guess number {}\", idx + 1))\n}",
            ],
        },
    ],
};

pub static STEP_COMPLETE_GAME: ConceptEntry = ConceptEntry {
    name: "13. the complete game",
    descriptions: &[
        DescriptionEntry {
            description: "you now know everything needed to build the full game. here is what the finished program should do:\n  1. pick a random secret number between 1 and 100\n  2. give the player 10 attempts\n  3. after each guess tell them too low, too high, or correct\n  4. stop when they win or run out of attempts\n  5. show a summary: total guesses, lowest, highest, guesses below vs above",
            examples: &[
                "// structure of the complete game\n\nfn print_divider() { ... }\nfn check_guess(int guess, int secret) -> string { ... }\n\ndec int      secret   = rand_int_range(1, 100)\ndec int      left     = 10\ndec arr[int] guesses  = []\n\nprint_divider()\nprintln(\"welcome to the guessing game!\")\nprintln(\"i am thinking of a number between 1 and 100\")\nprint_divider()\n\nwhile (left > 0) {\n    dec int    guess  = read_int(format(\"guess ({} left): \", left))\n    dec string result = check_guess(guess, secret)\n    guesses = arr_push(guesses, guess)\n    left -= 1\n\n    if (result == \"correct\") {\n        println(\"correct! you got it!\")\n        break\n    }\n    println(format(\"too {}!\", result))\n}\n\n// summary\nprint_divider()\nprintln(format(\"total guesses: {}\",  len(guesses)))\nprintln(format(\"lowest:        {}\",  arr_min(guesses)))\nprintln(format(\"highest:       {}\",  arr_max(guesses)))",
            ],
        },
        DescriptionEntry {
            description: "here is the full working implementation with all imports",
            examples: &[
                "get rand_int_range              from std::random\nget read_int, print, println    from std::io\nget format                      from std::str\nget arr_push, arr_min, arr_max,\n    arr_filter, len             from std::array\nget is_null                     from std::types\n\nfn print_divider() {\n    for i in 0..30 { print(\"-\") }\n    println(\"\")\n}\n\nfn check_guess(int guess, int secret) -> string {\n    if (guess < secret) { return \"low\" }\n    if (guess > secret) { return \"high\" }\n    return \"correct\"\n}\n\nfn find_first_correct(arr[int] guesses, int secret) -> int {\n    dec int i = 0\n    for g in guesses {\n        if (g == secret) { return i }\n        i += 1\n    }\n    return null\n}\n\nCONST int MAX_ATTEMPTS = 10\n\ndec int      secret  = rand_int_range(1, 100)\ndec int      left    = MAX_ATTEMPTS\ndec arr[int] guesses = []\ndec bool     won     = false\n\nprint_divider()\nprintln(\"welcome to the guessing game!\")\nprintln(\"i am thinking of a number between 1 and 100\")\nprint_divider()\n\nwhile (left > 0) {\n    dec int    guess  = read_int(format(\"guess ({} left): \", left))\n    dec string result = check_guess(guess, secret)\n    guesses = arr_push(guesses, guess)\n    left -= 1\n\n    if (result == \"correct\") {\n        won = true\n        println(\"correct! you got it!\")\n        break\n    }\n    println(format(\"too {}!\", result))\n}\n\nif (!won) {\n    println(format(\"out of attempts! the number was {}\", secret))\n}\n\ndec arr[int] below = arr_filter(guesses, fn(int g) -> bool { return g < secret })\ndec arr[int] above = arr_filter(guesses, fn(int g) -> bool { return g > secret })\ndec int      idx   = find_first_correct(guesses, secret)\n\nprint_divider()\nprintln(\"game summary\")\nprint_divider()\nprintln(format(\"total guesses: {}\", len(guesses)))\nprintln(format(\"lowest:        {}\", arr_min(guesses)))\nprintln(format(\"highest:       {}\", arr_max(guesses)))\nprintln(format(\"guesses below: {}\", len(below)))\nprintln(format(\"guesses above: {}\", len(above)))\n\nif (!is_null(idx)) {\n    println(format(\"you got it on attempt {}\", idx + 1))\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: extend the game with one of these ideas:\n  a) add a scoring system - more points for fewer guesses\n  b) add a play again loop - ask the player if they want another round\n  c) track wins across rounds and show a win rate at the very end\n  d) add difficulty levels - easy (1-50, 15 attempts), hard (1-200, 7 attempts)",
            examples: &[
                "// scoring example\nfn calculate_score(int attempts_used, int max_attempts) -> int {\n    dec int base  = 1000\n    dec int penalty = attempts_used * 50\n    dec int score = base - penalty\n    if (score < 0) { return 0 }\n    return score\n}\n\ndec int score = calculate_score(MAX_ATTEMPTS - left, MAX_ATTEMPTS)\nprintln(format(\"your score: {}\", score))",
            ],
        },
    ],
};

pub static ADV_INTRO: ConceptEntry = ConceptEntry {
    name: "1. what we are building",
    descriptions: &[
        DescriptionEntry {
            description: "in the beginner tutorial you built a self-contained game. everything lived in one file and nothing persisted between runs. real programs are different - they are split across multiple files, they save and load data, and they are built from reusable pieces.\n\nthis tutorial builds two things:\n  part 1 - a CSV library in csv.rl that parses, queries, and writes CSV files\n  part 2 - a task manager CLI in main.rl that imports and uses that library\n\nby the end you will have a program you can actually use day to day",
            examples: &[
                "// what the finished program looks like\n// $ rl run main.rl\n// task manager ready. type 'help' for commands\n// > add buy groceries\n// added task 1: buy groceries\n// > add write tutorial\n// added task 2: write tutorial\n// > done 1\n// marked task 1 as done\n// > list\n// [1] [done]    2026-06-20  buy groceries\n// [2] [pending] 2026-06-20  write tutorial\n// > remove 1\n// removed task 1\n// > quit\n// goodbye",
            ],
        },
        DescriptionEntry {
            description: "the CSV format we will use is simple: each row is one line, fields are separated by semicolons (not commas, to avoid conflicts with task text). the task file looks like this",
            examples: &[
                "// tasks.csv\n// id;status;created_at;text\n// 1;pending;1750000000;buy groceries\n// 2;done;1750000100;write tutorial\n// 3;pending;1750000200;fix bug in parser",
            ],
        },
        DescriptionEntry {
            description: "we use semicolons instead of commas so task text can contain commas freely. no quoted field handling needed - keep it simple, keep it readable",
            examples: &[
                "// valid task text with our format:\n// buy milk, eggs, and bread   <- comma in text, fine because delimiter is ;\n\n// would break a comma-delimited CSV:\n// buy milk, eggs, and bread   <- parser would split this into 4 fields",
            ],
        },
    ],
};

pub static ADV_MODULES: ConceptEntry = ConceptEntry {
    name: "2. splitting code across files",
    descriptions: &[
        DescriptionEntry {
            description: "you already know get from the beginner tutorial - you used it to import stdlib functions. the same keyword imports your own files. when you write get csv, rl looks for csv.rl in the same directory and runs it, making everything declared in it available",
            examples: &[
                "// main.rl\nget csv\n\n// now everything declared in csv.rl is available\n// csv_parse(...)\n// csv_serialize(...)\n// etc",
            ],
        },
        DescriptionEntry {
            description: "you can also import specific names from a file using the from syntax. this is cleaner when you only need a few things",
            examples: &[
                "// import specific functions from csv.rl\nget csv_parse, csv_serialize from csv\n\n// or from a subdirectory\nget csv_parse from lib::csv",
            ],
        },
        DescriptionEntry {
            description: "a file that is meant to be imported is just a regular .rl file with functions and constants declared at the top level. it should not have side effects - no println calls, no read calls, just declarations",
            examples: &[
                "// csv.rl - a library file\n// only declarations, no side effects\n\nCONST string DELIMITER = \";\"\n\nfn csv_parse(string raw) -> arr[arr[string]] {\n    // ...\n}\n\nfn csv_serialize(arr[arr[string]] rows) -> string {\n    // ...\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: create two files. csv.rl with a single constant DELIMITER = \";\", and main.rl that imports csv.rl and prints the delimiter\n\nexpected output:\n  delimiter is: ;",
            examples: &[
                "// csv.rl\nCONST string DELIMITER = \";\"\n\n// main.rl\nget csv\nget concat from std::str\n\nfn main() {\n    println(concat(\"delimiter is: \", DELIMITER))\n}",
            ],
        },
    ],
};

pub static ADV_STRING_PARSING: ConceptEntry = ConceptEntry {
    name: "3. parsing strings",
    descriptions: &[
        DescriptionEntry {
            description: "parsing means taking raw text and turning it into structured data your program can work with. it is one of the most common real-world tasks. you already know split from std::str - it is the foundation of CSV parsing",
            examples: &[
                "get split from std::str\n\ndec string row = \"1;pending;1750000000;buy groceries\"\ndec arr[string] fields = split(row, \";\")\n\nprintln(fields[0]) // 1\nprintln(fields[1]) // pending\nprintln(fields[2]) // 1750000000\nprintln(fields[3]) // buy groceries",
            ],
        },
        DescriptionEntry {
            description: "to parse a full CSV string you split it into lines first, then split each line into fields. you get an array of rows where each row is an array of strings",
            examples: &[
                "get split from std::str\n\ndec string csv = \"1;pending;buy milk\\n2;done;write code\"\ndec arr[string]        lines = split(csv, \"\\n\")\ndec arr[arr[string]]   rows  = []\n\n// we will fill this in with arr_push soon",
            ],
        },
        DescriptionEntry {
            description: "trim is important when parsing. files often have trailing newlines or spaces that will silently break comparisons if you do not strip them first",
            examples: &[
                "get split, trim from std::str\n\ndec string line = \"  1;pending;buy milk  \\n\"\ndec string clean = trim(line)\ndec arr[string] fields = split(clean, \";\")\nprintln(fields[1]) // pending  (not \"pending  \\n\")",
            ],
        },
        DescriptionEntry {
            description: "is_empty lets you skip blank lines - files often end with a trailing newline that produces an empty string when split",
            examples: &[
                "get split, trim, is_empty from std::str\n\ndec string csv   = \"row1\\nrow2\\n\"\ndec arr[string] lines = split(csv, \"\\n\")\n\nfor line in lines {\n    if (is_empty(trim(line))) { continue }\n    println(line) // row1, row2 - trailing empty line skipped\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: write a function csv_parse_row in csv.rl that takes a single CSV line as a string and returns an array of trimmed fields. test it in main.rl\n\nexpected output:\n  [1, pending, 1750000000, buy groceries]",
            examples: &[
                "// csv.rl\nget split, trim from std::str\n\nCONST string DELIMITER = \";\"\n\nfn csv_parse_row(string line) -> arr[string] {\n    get arr_map from std::array\n    dec arr[string] fields = split(line, DELIMITER)\n    return arr_map(fields, fn(string f) -> string { return trim(f) })\n}\n\n// main.rl\nget csv\nget concat from std::str\n\nfn main() {\n    dec arr[string] row = csv_parse_row(\"1;pending;1750000000;buy groceries\")\n    println(row)\n}",
            ],
        },
    ],
};

pub static ADV_CSV_PARSER: ConceptEntry = ConceptEntry {
    name: "4. building the CSV parser",
    descriptions: &[
        DescriptionEntry {
            description: "now build csv_parse - takes a full CSV string, splits into lines, skips blanks and the header, parses each row, returns an array of rows. this is the core of your library",
            examples: &[
                "// csv.rl\nget split, trim, is_empty from std::str\nget arr_push             from std::array\n\nfn csv_parse(string raw) -> arr[arr[string]] {\n    dec arr[string]      lines  = split(raw, \"\\n\")\n    dec arr[arr[string]] rows   = []\n    dec bool             header = true\n\n    for line in lines {\n        if (is_empty(trim(line))) { continue }\n        if (header) { header = false    continue } // skip header row\n        rows = arr_push(rows, csv_parse_row(line))\n    }\n\n    return rows\n}",
            ],
        },
        DescriptionEntry {
            description: "now build csv_serialize - the reverse. takes an array of rows, joins each row's fields with the delimiter, joins rows with newlines, prepends the header line",
            examples: &[
                "// csv.rl\nget join, concat from std::str\n\nCONST string HEADER = \"id;status;created_at;text\"\n\nfn csv_serialize_row(arr[string] row) -> string {\n    return join(row, DELIMITER)\n}\n\nfn csv_serialize(arr[arr[string]] rows) -> string {\n    get arr_map from std::array\n    dec arr[string] lines = arr_map(rows, fn(arr[string] r) -> string {\n        return csv_serialize_row(r)\n    })\n    return concat(HEADER, \"\\n\", join(lines, \"\\n\"))\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: test the round-trip. parse a CSV string then serialize it back and check that the output matches the input (minus any trailing whitespace)\n\nexpected output:\n  round-trip ok: true",
            examples: &[
                "// main.rl\nget csv\nget trim from std::str\n\nfn main() {\n    dec string input = \"id;status;created_at;text\\n1;pending;1750000000;buy milk\\n2;done;1750000100;write code\"\n\n    dec arr[arr[string]] rows   = csv_parse(input)\n    dec string           output = csv_serialize(rows)\n\n    println(concat(\"round-trip ok: \", trim(input) == trim(output)))\n}",
            ],
        },
    ],
};

pub static ADV_CSV_IO: ConceptEntry = ConceptEntry {
    name: "5. reading and writing CSV files",
    descriptions: &[
        DescriptionEntry {
            description: "you know the parser works on strings. now connect it to the filesystem. read_file from std::io gives you the file contents as a string - pass it straight to csv_parse",
            examples: &[
                "// csv.rl\nget read_file, write_file from std::io\nget path_exists          from std::path\n\nfn csv_load(string path) -> arr[arr[string]] {\n    if (!path_exists(path)) { return [] }\n    dec string raw = read_file(path)\n    return csv_parse(raw)\n}\n\nfn csv_save(string path, arr[arr[string]] rows) {\n    write_file(path, csv_serialize(rows))\n}",
            ],
        },
        DescriptionEntry {
            description: "path_exists is important here - if the file does not exist yet (first run of the program) you want to return an empty array, not crash",
            examples: &[
                "get path_exists from std::path\n\nfn csv_load(string path) -> arr[arr[string]] {\n    if (!path_exists(path)) {\n        return [] // first run, no file yet\n    }\n    return csv_parse(read_file(path))\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: add csv_load and csv_save to csv.rl. in main.rl load tasks.csv, print how many rows it has, add a test row, save it back, then load again and verify the count increased\n\nexpected output (first run):\n  loaded 0 rows\n  saved 1 row\n  reloaded 1 row",
            examples: &[
                "// main.rl\nget csv\nget arr_push, len from std::array\nget format         from std::str\n\nCONST string TASKS_FILE = \"tasks.csv\"\n\nfn main() {\n    dec arr[arr[string]] rows = csv_load(TASKS_FILE)\n    println(format(\"loaded {} rows\", len(rows)))\n\n    rows = arr_push(rows, [\"1\", \"pending\", \"1750000000\", \"test task\"])\n    csv_save(TASKS_FILE, rows)\n    println(format(\"saved {} row\", len(rows)))\n\n    dec arr[arr[string]] reloaded = csv_load(TASKS_FILE)\n    println(format(\"reloaded {} row\", len(reloaded)))\n}",
            ],
        },
    ],
};

pub static ADV_CSV_QUERY: ConceptEntry = ConceptEntry {
    name: "6. querying CSV data",
    descriptions: &[
        DescriptionEntry {
            description: "raw rows are just arrays of strings. to query them usefully you need helper functions that know which column index means what. define column constants so you never use magic numbers",
            examples: &[
                "// csv.rl\nCONST int COL_ID         = 0\nCONST int COL_STATUS     = 1\nCONST int COL_CREATED_AT = 2\nCONST int COL_TEXT       = 3\n\n// now instead of row[1] you write row[COL_STATUS]\n// readable and safe if columns ever change",
            ],
        },
        DescriptionEntry {
            description: "arr_filter with a lambda is how you query rows. the lambda receives a row and returns true if it matches. this is the pattern you will use for every filtered view",
            examples: &[
                "get arr_filter from std::array\n\n// get all pending tasks\ndec arr[arr[string]] pending = arr_filter(rows, fn(arr[string] row) -> bool {\n    return row[COL_STATUS] == \"pending\"\n})\n\n// get all done tasks\ndec arr[arr[string]] done = arr_filter(rows, fn(arr[string] row) -> bool {\n    return row[COL_STATUS] == \"done\"\n})",
            ],
        },
        DescriptionEntry {
            description: "arr_find lets you locate a single row by id. it returns the first matching row or null if nothing matches - always check with is_null before using the result",
            examples: &[
                "get arr_find  from std::array\nget is_null   from std::types\n\nfn csv_find_by_id(arr[arr[string]] rows, string id) -> arr[string] {\n    return arr_find(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] == id\n    })\n}\n\ndec arr[string] task = csv_find_by_id(rows, \"2\")\nif (is_null(task)) {\n    println(\"not found\")\n} else {\n    println(task[COL_TEXT])\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: add these query functions to csv.rl:\n  csv_filter_by(rows, col, value) -> arr[arr[string]]\n  csv_find_by_id(rows, id)        -> arr[string]\n\nthen test them in main.rl against a hardcoded set of rows",
            examples: &[
                "// csv.rl\nget arr_filter, arr_find from std::array\n\nfn csv_filter_by(arr[arr[string]] rows, int col, string value) -> arr[arr[string]] {\n    return arr_filter(rows, fn(arr[string] row) -> bool {\n        return row[col] == value\n    })\n}\n\nfn csv_find_by_id(arr[arr[string]] rows, string id) -> arr[string] {\n    return arr_find(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] == id\n    })\n}\n\n// main.rl test\ndec arr[arr[string]] rows = [\n    [\"1\", \"pending\", \"1750000000\", \"buy milk\"],\n    [\"2\", \"done\",    \"1750000100\", \"write code\"],\n    [\"3\", \"pending\", \"1750000200\", \"fix bug\"],\n]\n\ndec arr[arr[string]] pending = csv_filter_by(rows, COL_STATUS, \"pending\")\nprintln(len(pending)) // 2\n\ndec arr[string] task = csv_find_by_id(rows, \"2\")\nprintln(task[COL_TEXT]) // write code",
            ],
        },
    ],
};

pub static ADV_CSV_MUTATION: ConceptEntry = ConceptEntry {
    name: "7. mutating CSV data",
    descriptions: &[
        DescriptionEntry {
            description: "arrays in rl are values - when you filter or map you get a new array, the original is unchanged. mutation means building a new array with the change applied. this is the same pattern arr_push uses",
            examples: &[
                "get arr_map from std::array\n\n// update a field in one row, return a new rows array\nfn csv_update_field(arr[arr[string]] rows, string id, int col, string value) -> arr[arr[string]] {\n    return arr_map(rows, fn(arr[string] row) -> arr[string] {\n        if (row[COL_ID] != id) { return row }\n        // build updated row\n        dec arr[string] updated = row\n        updated[col] = value\n        return updated\n    })\n}",
            ],
        },
        DescriptionEntry {
            description: "deleting a row means filtering it out. arr_filter with the opposite condition gives you every row except the one you want gone",
            examples: &[
                "get arr_filter from std::array\n\nfn csv_remove_by_id(arr[arr[string]] rows, string id) -> arr[arr[string]] {\n    return arr_filter(rows, fn(arr[string] row) -> bool {\n        return row[COL_ID] != id\n    })\n}",
            ],
        },
        DescriptionEntry {
            description: "adding a row means generating a new ID first. the simplest approach: find the current max ID and add 1. arr_reduce works well here",
            examples: &[
                "get arr_reduce  from std::array\nget to_int, to_string from std::types\n\nfn csv_next_id(arr[arr[string]] rows) -> string {\n    if (len(rows) == 0) { return \"1\" }\n    dec int max_id = arr_reduce(\n        rows,\n        fn(int acc, arr[string] row) -> int {\n            dec int id = to_int(row[COL_ID])\n            if (id > acc) { return id }\n            return acc\n        },\n        0\n    )\n    return to_string(max_id + 1)\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: add these mutation functions to csv.rl:\n  csv_add_row(rows, fields)           -> arr[arr[string]]\n  csv_remove_by_id(rows, id)          -> arr[arr[string]]\n  csv_update_field(rows, id, col, val)-> arr[arr[string]]\n  csv_next_id(rows)                   -> string",
            examples: &[
                "// test in main.rl\ndec arr[arr[string]] rows = []\n\n// add\nrows = csv_add_row(rows, [csv_next_id(rows), \"pending\", \"1750000000\", \"buy milk\"])\nrows = csv_add_row(rows, [csv_next_id(rows), \"pending\", \"1750000100\", \"write code\"])\nprintln(len(rows)) // 2\n\n// update\nrows = csv_update_field(rows, \"1\", COL_STATUS, \"done\")\nprintln(csv_find_by_id(rows, \"1\")[COL_STATUS]) // done\n\n// remove\nrows = csv_remove_by_id(rows, \"1\")\nprintln(len(rows)) // 1",
            ],
        },
    ],
};

pub static ADV_PROGRAM_LOOP: ConceptEntry = ConceptEntry {
    name: "8. the program loop",
    descriptions: &[
        DescriptionEntry {
            description: "part 1 gave you a complete CSV library. now build the task manager on top of it. the program is a REPL - read, evaluate, print, loop. it reads a command, acts on it, prints the result, and loops until the user quits",
            examples: &[
                "// main.rl skeleton\nget csv\nget read, println from std::io\nget trim           from std::str\n\nCONST string TASKS_FILE = \"tasks.csv\"\n\nfn main() {\n    dec arr[arr[string]] tasks = csv_load(TASKS_FILE)\n    println(\"task manager ready. type 'help' for commands\")\n\n    while (true) {\n        dec string input   = read(\"> \")\n        dec string command = trim(input)\n\n        if (command == \"quit\") { break }\n\n        // dispatch commands here\n        println(concat(\"unknown command: \", command))\n    }\n\n    println(\"goodbye\")\n}",
            ],
        },
        DescriptionEntry {
            description: "commands like 'add buy milk' have two parts: the command name and the arguments. split on the first space to separate them. use index_of to find where the first space is, then slice to extract each part",
            examples: &[
                "get index_of, slice, trim from std::str\n\nfn parse_command(string input) -> arr[string] {\n    dec int space = index_of(input, \" \")\n    if (space == -1) {\n        return [trim(input), \"\"] // no args\n    }\n    dec string cmd  = slice(input, 0, space)\n    dec string args = trim(slice(input, space + 1, len(input)))\n    return [cmd, args]\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: get the REPL loop running. it should read input, parse the command, and for now just echo back what command and args it parsed. quit should exit cleanly\n\nexpected output:\n  > add buy milk\n  command: add  args: buy milk\n  > list\n  command: list  args:\n  > quit\n  goodbye",
            examples: &[
                "fn main() {\n    println(\"task manager ready. type 'help' for commands\")\n\n    while (true) {\n        dec string       input  = read(\"> \")\n        dec arr[string]  parts  = parse_command(trim(input))\n        dec string       cmd    = parts[0]\n        dec string       args   = parts[1]\n\n        if (cmd == \"quit\") { break }\n\n        println(format(\"command: {}  args: {}\", cmd, args))\n    }\n\n    println(\"goodbye\")\n}",
            ],
        },
    ],
};

pub static ADV_TIME: ConceptEntry = ConceptEntry {
    name: "9. working with time",
    descriptions: &[
        DescriptionEntry {
            description: "std::time gives you the current time as a unix timestamp - an integer counting seconds since January 1 1970. time_now() returns it. store this when a task is created so you know when it was added",
            examples: &[
                "get time_now from std::time\n\ndec int created_at = time_now()\nprintln(created_at) // e.g. 1750000000",
            ],
        },
        DescriptionEntry {
            description: "raw timestamps are not readable. format_date_str turns a timestamp into a date string. format_time_str gives you the time. format_time lets you build any pattern you want",
            examples: &[
                "get time_now, format_date_str, format_time_str, format_time from std::time\n\ndec int ts = time_now()\nprintln(format_date_str(ts))          // 2026-06-20\nprintln(format_time_str(ts))          // 14:32:07\nprintln(format_time(ts, \"%d/%m/%Y\"))  // 20/06/2026",
            ],
        },
        DescriptionEntry {
            description: "time_diff gives you the number of seconds between two timestamps. useful for showing how old a task is",
            examples: &[
                "get time_now, time_diff from std::time\n\ndec int created = 1750000000\ndec int now     = time_now()\ndec int age     = time_diff(created, now)\n\nprintln(format(\"{} seconds old\", age))",
            ],
        },
        DescriptionEntry {
            description: "time_parts breaks a timestamp into its components as an array: [year, month, day, hour, minute, second]",
            examples: &[
                "get time_now, time_parts from std::time\n\ndec arr[int] parts = time_parts(time_now())\nprintln(parts[0]) // year  e.g. 2026\nprintln(parts[1]) // month e.g. 6\nprintln(parts[2]) // day   e.g. 20",
            ],
        },
        DescriptionEntry {
            description: "exercise: write a function format_age(int created_at) -> string that returns a human readable age string. use time_diff and some arithmetic\n\nexpected output examples:\n  just now\n  5 minutes ago\n  3 hours ago\n  2 days ago",
            examples: &[
                "get time_now, time_diff from std::time\nget format              from std::str\n\nfn format_age(int created_at) -> int {\n    dec int diff    = time_diff(created_at, time_now())\n    dec int minutes = diff / 60\n    dec int hours   = minutes / 60\n    dec int days    = hours / 24\n\n    if (diff < 60)     { return \"just now\" }\n    if (minutes < 60)  { return format(\"{} minutes ago\", minutes) }\n    if (hours < 24)    { return format(\"{} hours ago\", hours) }\n    return format(\"{} days ago\", days)\n}",
            ],
        },
    ],
};

pub static ADV_COMMANDS_ADD_LIST: ConceptEntry = ConceptEntry {
    name: "10. add and list commands",
    descriptions: &[
        DescriptionEntry {
            description: "now wire up real commands. add creates a new task row using csv_next_id, time_now for the timestamp, and the args string as the text. it then saves immediately so nothing is lost if the program crashes",
            examples: &[
                "get time_now   from std::time\nget to_string  from std::types\nget concat     from std::str\n\nfn cmd_add(arr[arr[string]] tasks, string text) -> arr[arr[string]] {\n    dec string id         = csv_next_id(tasks)\n    dec string created_at = to_string(time_now())\n    dec arr[string] row   = [id, \"pending\", created_at, text]\n    tasks = csv_add_row(tasks, row)\n    csv_save(TASKS_FILE, tasks)\n    println(format(\"added task {}: {}\", id, text))\n    return tasks\n}",
            ],
        },
        DescriptionEntry {
            description: "list prints all tasks in a readable table. use format to align columns. format_date_str converts the stored timestamp string to a readable date",
            examples: &[
                "get format_date_str from std::time\nget to_int          from std::types\nget format          from std::str\n\nfn print_task(arr[string] row) {\n    dec string id      = row[COL_ID]\n    dec string status  = row[COL_STATUS]\n    dec string date    = format_date_str(to_int(row[COL_CREATED_AT]))\n    dec string text    = row[COL_TEXT]\n    println(format(\"[{}] [{}] {}  {}\", id, status, date, text))\n}\n\nfn cmd_list(arr[arr[string]] tasks) {\n    if (len(tasks) == 0) {\n        println(\"no tasks\")\n        return\n    }\n    for task in tasks {\n        print_task(task)\n    }\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: implement cmd_add and cmd_list. wire them into your main loop. the program should persist tasks between runs\n\nexpected output:\n  > add buy groceries\n  added task 1: buy groceries\n  > add write tutorial\n  added task 2: write tutorial\n  > list\n  [1] [pending] 2026-06-20  buy groceries\n  [2] [pending] 2026-06-20  write tutorial",
            examples: &[
                "// in main() dispatch block:\nif (cmd == \"add\") {\n    if (is_empty(args)) {\n        println(\"usage: add <task text>\")\n    } else {\n        tasks = cmd_add(tasks, args)\n    }\n} else if (cmd == \"list\") {\n    cmd_list(tasks)\n}",
            ],
        },
    ],
};

pub static ADV_COMMANDS_DONE_REMOVE: ConceptEntry = ConceptEntry {
    name: "11. done, remove, and clear",
    descriptions: &[
        DescriptionEntry {
            description: "done and remove both take a task ID as their argument. the ID comes in as a string from the command parser. validate it exists before acting - use csv_find_by_id and is_null",
            examples: &[
                "get is_null   from std::types\nget format    from std::str\n\nfn cmd_done(arr[arr[string]] tasks, string id) -> arr[arr[string]] {\n    dec arr[string] task = csv_find_by_id(tasks, id)\n    if (is_null(task)) {\n        println(format(\"no task with id {}\", id))\n        return tasks\n    }\n    tasks = csv_update_field(tasks, id, COL_STATUS, \"done\")\n    csv_save(TASKS_FILE, tasks)\n    println(format(\"marked task {} as done\", id))\n    return tasks\n}",
            ],
        },
        DescriptionEntry {
            description: "remove works the same way - find first, then delete. give the user a confirmation message either way so they know what happened",
            examples: &[
                "fn cmd_remove(arr[arr[string]] tasks, string id) -> arr[arr[string]] {\n    dec arr[string] task = csv_find_by_id(tasks, id)\n    if (is_null(task)) {\n        println(format(\"no task with id {}\", id))\n        return tasks\n    }\n    tasks = csv_remove_by_id(tasks, id)\n    csv_save(TASKS_FILE, tasks)\n    println(format(\"removed task {}\", id))\n    return tasks\n}",
            ],
        },
        DescriptionEntry {
            description: "clear removes all tasks with status done in one pass. arr_filter does the work - keep everything that is not done",
            examples: &[
                "fn cmd_clear(arr[arr[string]] tasks) -> arr[arr[string]] {\n    dec arr[arr[string]] remaining = csv_filter_by(tasks, COL_STATUS, \"pending\")\n    dec int removed = len(tasks) - len(remaining)\n    csv_save(TASKS_FILE, remaining)\n    println(format(\"cleared {} completed task(s)\", removed))\n    return remaining\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: implement done, remove, and clear. wire all five commands into the main loop. test the full flow: add tasks, mark some done, clear them, verify the list\n\nexpected output:\n  > done 1\n  marked task 1 as done\n  > clear\n  cleared 1 completed task(s)\n  > list\n  [2] [pending] 2026-06-20  write tutorial",
            examples: &[
                "} else if (cmd == \"done\") {\n    if (is_empty(args)) {\n        println(\"usage: done <id>\")\n    } else {\n        tasks = cmd_done(tasks, args)\n    }\n} else if (cmd == \"remove\") {\n    if (is_empty(args)) {\n        println(\"usage: remove <id>\")\n    } else {\n        tasks = cmd_remove(tasks, args)\n    }\n} else if (cmd == \"clear\") {\n    tasks = cmd_clear(tasks)\n}",
            ],
        },
    ],
};

pub static ADV_FILTERED_VIEWS: ConceptEntry = ConceptEntry {
    name: "12. filtered views and stats",
    descriptions: &[
        DescriptionEntry {
            description: "list alone shows everything. extend it to accept an optional argument: 'list done' or 'list pending' shows only those tasks. the args string you already parse makes this straightforward",
            examples: &[
                "fn cmd_list(arr[arr[string]] tasks, string filter) {\n    dec arr[arr[string]] view = tasks\n\n    if (filter == \"done\")    { view = csv_filter_by(tasks, COL_STATUS, \"done\") }\n    if (filter == \"pending\") { view = csv_filter_by(tasks, COL_STATUS, \"pending\") }\n\n    if (len(view) == 0) {\n        println(\"no tasks\")\n        return\n    }\n    for task in view {\n        print_task(task)\n    }\n}",
            ],
        },
        DescriptionEntry {
            description: "a stats command gives a useful summary. use arr_filter to count done vs pending, and format a clean report",
            examples: &[
                "fn cmd_stats(arr[arr[string]] tasks) {\n    dec int total   = len(tasks)\n    dec int done    = len(csv_filter_by(tasks, COL_STATUS, \"done\"))\n    dec int pending = total - done\n\n    println(format(\"total:   {}\", total))\n    println(format(\"done:    {}\", done))\n    println(format(\"pending: {}\", pending))\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: update cmd_list to accept the filter argument and add cmd_stats. add 'stats' to the dispatch loop\n\nexpected output:\n  > list pending\n  [2] [pending] 2026-06-20  write tutorial\n  > stats\n  total:   2\n  done:    1\n  pending: 1",
            examples: &[
                "// updated dispatch for list:\n} else if (cmd == \"list\") {\n    cmd_list(tasks, args) // args is \"done\", \"pending\", or \"\"\n} else if (cmd == \"stats\") {\n    cmd_stats(tasks)\n}",
            ],
        },
    ],
};

pub static ADV_HELP_POLISH: ConceptEntry = ConceptEntry {
    name: "13. help and polish",
    descriptions: &[
        DescriptionEntry {
            description: "help should print every available command with a short description. store the help text as a constant array of strings - one entry per command - and loop over it to print",
            examples: &[
                "CONST arr[string] HELP_LINES = [\n    \"  add <text>       add a new task\",\n    \"  done <id>        mark a task as done\",\n    \"  remove <id>      delete a task\",\n    \"  list             show all tasks\",\n    \"  list done        show completed tasks\",\n    \"  list pending     show pending tasks\",\n    \"  clear            remove all completed tasks\",\n    \"  stats            show task counts\",\n    \"  help             show this message\",\n    \"  quit             exit the program\",\n]\n\nfn cmd_help() {\n    println(\"commands:\")\n    for line in HELP_LINES {\n        println(line)\n    }\n}",
            ],
        },
        DescriptionEntry {
            description: "unknown commands should not silently do nothing. print a helpful message pointing the user to help",
            examples: &[
                "// at the end of the dispatch chain:\n} else {\n    println(format(\"unknown command: '{}'. type 'help' for commands\", cmd))\n}",
            ],
        },
        DescriptionEntry {
            description: "on startup show a summary so the user immediately knows what state they are in. this uses the same stats logic but formatted as a one-liner",
            examples: &[
                "fn print_startup_summary(arr[arr[string]] tasks) {\n    dec int total   = len(tasks)\n    dec int pending = len(csv_filter_by(tasks, COL_STATUS, \"pending\"))\n    println(format(\"task manager ready - {} task(s), {} pending. type 'help' for commands\", total, pending))\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: add help, unknown command handling, and the startup summary. run the full program and make sure every command works end to end",
            examples: &[
                "// startup\nprint_startup_summary(tasks)\n\n// in dispatch:\n} else if (cmd == \"help\") {\n    cmd_help()\n} else {\n    println(format(\"unknown command: '{}'. type 'help' for commands\", cmd))\n}",
            ],
        },
    ],
};

pub static ADV_COMPLETE: ConceptEntry = ConceptEntry {
    name: "14. the complete program",
    descriptions: &[
        DescriptionEntry {
            description: "your project should now have two files:\n  csv.rl  - the reusable CSV library\n  main.rl - the task manager that imports it\n\nhere is the full structure of each file",
            examples: &[
                "// csv.rl structure\n// imports\nget split, trim, is_empty, join, concat, format from std::str\nget arr_push, arr_map, arr_filter, arr_find,\n    arr_reduce, len                              from std::array\nget read_file, write_file                        from std::io\nget path_exists                                  from std::path\nget to_int, to_string                            from std::types\n\n// constants\nCONST string DELIMITER   = \";\"\nCONST string HEADER      = \"id;status;created_at;text\"\nCONST int    COL_ID         = 0\nCONST int    COL_STATUS     = 1\nCONST int    COL_CREATED_AT = 2\nCONST int    COL_TEXT       = 3\n\n// functions\n// csv_parse_row, csv_parse, csv_serialize_row, csv_serialize\n// csv_load, csv_save\n// csv_filter_by, csv_find_by_id\n// csv_next_id, csv_add_row, csv_remove_by_id, csv_update_field",
            ],
        },
        DescriptionEntry {
            description: "the main.rl structure",
            examples: &[
                "// main.rl structure\nget csv\nget time_now, format_date_str from std::time\nget read, println              from std::io\nget format, trim, is_empty,\n    index_of, slice, len       from std::str\nget is_null                    from std::types\nget arr_filter, len            from std::array\nget to_int                     from std::types\n\nCONST string TASKS_FILE = \"tasks.csv\"\nCONST arr[string] HELP_LINES = [ ... ]\n\n// functions\n// parse_command, format_age, print_task\n// print_startup_summary, cmd_help\n// cmd_add, cmd_list, cmd_done, cmd_remove, cmd_clear, cmd_stats\n\nfn main() {\n    dec arr[arr[string]] tasks = csv_load(TASKS_FILE)\n    print_startup_summary(tasks)\n\n    while (true) {\n        dec arr[string] parts = parse_command(trim(read(\"> \")))\n        dec string cmd  = parts[0]\n        dec string args = parts[1]\n\n        if      (cmd == \"quit\")   { break }\n        else if (cmd == \"add\")    { tasks = cmd_add(tasks, args) }\n        else if (cmd == \"done\")   { tasks = cmd_done(tasks, args) }\n        else if (cmd == \"remove\") { tasks = cmd_remove(tasks, args) }\n        else if (cmd == \"list\")   { cmd_list(tasks, args) }\n        else if (cmd == \"clear\")  { tasks = cmd_clear(tasks) }\n        else if (cmd == \"stats\")  { cmd_stats(tasks) }\n        else if (cmd == \"help\")   { cmd_help() }\n        else { println(format(\"unknown command: '{}'. type 'help'\", cmd)) }\n    }\n\n    println(\"goodbye\")\n}",
            ],
        },
        DescriptionEntry {
            description: "exercise: extend the program with one or more of these ideas:\n  a) search command - 'search <term>' filters tasks whose text contains the term\n  b) edit command - 'edit <id> <new text>' updates a task's text\n  c) due dates - add a due_at column, 'due <id> <days>' sets a due date, list shows overdue tasks differently\n  d) priorities - add a priority column (low/normal/high), 'list high' shows only high priority tasks\n  e) export command - 'export' writes a human-readable text report to tasks_report.txt",
            examples: &[
                "// search example\nget contains from std::str\n\nfn cmd_search(arr[arr[string]] tasks, string term) {\n    if (is_empty(term)) {\n        println(\"usage: search <term>\")\n        return\n    }\n    dec arr[arr[string]] results = arr_filter(tasks, fn(arr[string] row) -> bool {\n        return contains(row[COL_TEXT], term)\n    })\n    if (len(results) == 0) {\n        println(format(\"no tasks matching '{}'\", term))\n        return\n    }\n    for task in results {\n        print_task(task)\n    }\n}",
            ],
        },
    ],
};
