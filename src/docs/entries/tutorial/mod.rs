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
