use crate::entry::{ConceptCategory, ConceptEntry, DescriptionEntry, DescriptionKind};
pub static STEP_COMPLETE_GAME: ConceptEntry = ConceptEntry {
    name: "14. the complete game",
    summary: "the complete game",
    category: ConceptCategory::Tooling,
    prerequisites: &[],
    descriptions: &[
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "you now know everything needed to build the full game. here is what the finished program should do:\n  1. pick a random secret number between 1 and 100\n  2. give the player 10 attempts\n  3. after each guess tell them too low, too high, or correct\n  4. stop when they win or run out of attempts\n  5. show a summary: total guesses, lowest, highest, guesses below vs above",
            examples: &[
                "// structure of the complete game\n\nfn print_divider() { ... }\nfn check_guess(int guess, int secret) -> string { ... }\n\ndec int      secret   = rand_int_range(1, 100)?\ndec int      left     = 10\ndec arr[int] guesses  = []\n\nprint_divider()\nprintln(\"welcome to the guessing game!\")\nprintln(\"i am thinking of a number between 1 and 100\")\nprint_divider()\n\nwhile (left > 0) {\n    dec int    guess = read_int(format(\"guess ({} left): \", left))?\n    dec string res   = check_guess(guess, secret)\n    guesses = arr_push(guesses, guess)?\n    left -= 1\n\n    if (res == \"correct\") {\n        println(\"correct! you got it!\")\n        break\n    }\n    println(format(\"too {}!\", res))\n}\n\n// summary\nprint_divider()\nprintln(format(\"total guesses: {}\",  len(guesses)))\nprintln(format(\"lowest:        {}\",  arr_min(guesses)?))\nprintln(format(\"highest:       {}\",  arr_max(guesses)?))",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "notice this uses `?` on every call that can fail: rand_int_range, read_int, arr_push, arr_min, and arr_max are all fallible now. this is a script (no fn main), so a `?` that hits an err just stops the whole program right there with that error - exactly like the pitfall in the results chapter said. also notice the variable holding check_guess's return value is now called res instead of result, so it doesn't get confused with the result[T] type",
            examples: &[],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "here is the full working implementation with all imports",
            examples: &[
                "get rand_int_range           from std::random\nget read_int, print, println from std::io\nget format, repeat           from std::str\nget arr_push, arr_min, arr_max,\n    arr_filter, len          from std::array\nget is_null                  from std::types\n\nfn print_divider() {\n    \"-\".repeat(30).println()\n}\n\nfn check_guess(int guess, int secret) -> string {\n    if (guess < secret) { return \"low\" }\n    if (guess > secret) { return \"high\" }\n    return \"correct\"\n}\n\nfn find_first_correct(arr[int] guesses, int secret) -> int {\n    dec int i = 0\n    for g in guesses {\n        if (g == secret) { return i }\n        i += 1\n    }\n    return null\n}\n\nCONST int MAX_ATTEMPTS = 10\n\ndec int      secret  = rand_int_range(1, 100)?\ndec int      left    = MAX_ATTEMPTS\ndec arr[int] guesses = []\ndec bool     won     = false\n\nprint_divider()\nprintln(\"welcome to the guessing game!\")\nprintln(\"i am thinking of a number between 1 and 100\")\nprint_divider()\n\nwhile (left > 0) {\n    dec int    guess = read_int(format(\"guess ({} left): \", left))?\n    dec string res   = check_guess(guess, secret)\n    guesses = arr_push(guesses, guess)?\n    left -= 1\n\n    if (res == \"correct\") {\n        won = true\n        println(\"correct! you got it!\")\n        break\n    }\n    println(format(\"too {}!\", res))\n}\n\nif (!won) {\n    println(format(\"out of attempts! the number was {}\", secret))\n}\n\ndec arr[int] below = arr_filter(guesses, fn (int g) -> bool {\n    return g < secret\n})?\ndec arr[int] above = arr_filter(guesses, fn (int g) -> bool {\n    return g > secret\n})?\ndec int idx = find_first_correct(guesses, secret)\n\nprint_divider()\nprintln(\"game summary\")\nprint_divider()\nprintln(format(\"total guesses: {}\", len(guesses)))\nprintln(format(\"lowest:        {}\", arr_min(guesses)?))\nprintln(format(\"highest:       {}\", arr_max(guesses)?))\nprintln(format(\"guesses below: {}\", len(below)))\nprintln(format(\"guesses above: {}\", len(above)))\n\nif (!is_null(idx)) {\n    println(format(\"you got it on attempt {}\", idx + 1))\n}",
            ],
            expected_output: &[],
        },
        DescriptionEntry {
            kind: DescriptionKind::Explanation,
            title: None,
            description: "exercise: extend the game with one of these ideas:\n  a) add a scoring system - more points for fewer guesses\n  b) add a play again loop - ask the player if they want another round\n  c) track wins across rounds and show a win rate at the very end\n  d) add difficulty levels - easy (1-50, 15 attempts), hard (1-200, 7 attempts)",
            examples: &[
                "// scoring example\nfn calculate_score(int attempts_used, int max_attempts) -> int {\n    dec int base  = 1000\n    dec int penalty = attempts_used * 50\n    dec int score = base - penalty\n    if (score < 0) { return 0 }\n    return score\n}\n\ndec int score = calculate_score(MAX_ATTEMPTS - left, MAX_ATTEMPTS)\nprintln(format(\"your score: {}\", score))",
            ],
            expected_output: &[],
        },
    ],
    pitfalls: &[],
    related: &[],
    related_stdlib: &[],
    since: None,
};
