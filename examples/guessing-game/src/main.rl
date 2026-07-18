get rand_int_range from std::random
get read_int, print, println from std::io
get format, repeat from std::str
get arr_push, arr_min, arr_max, arr_filter, len from std::array
get is_null from std::types

fn print_divider() {
    "-".repeat(30).println()
}

fn check_guess(int guess, int secret) -> string {
    if (guess < secret) {
        return "low"
    }
    if (guess > secret) {
        return "high"
    }
    return "correct"
}

fn find_first_correct(arr[int] guesses, int secret) -> int {
    dec int i = 0
    for g in guesses {
        if (g == secret) {
            return i
        }
        i += 1
    }
    return null
}

CONST int MAX_ATTEMPTS = 10

dec int secret = rand_int_range(1, 100)?
dec int left = MAX_ATTEMPTS
dec arr[int] guesses = []
dec bool won = false

print_divider()
println("welcome to the guessing game!")
println("i am thinking of a number between 1 and 100")
print_divider()

while (left > 0) {
    dec int guess = read_int(format("guess ({} left): ", left))?
    dec string res = check_guess(guess, secret)
    guesses = arr_push(guesses, guess)?
    left -= 1

    if (res == "correct") {
        won = true
        println("correct! you got it!")
        break
    }
    println(format("too {}!", res))
}

if (!won) {
    println(format("out of attempts! the number was {}", secret))
}

dec arr[int] below = arr_filter(guesses, fn (int g) -> bool {
    return g < secret
})?
dec arr[int] above = arr_filter(guesses, fn (int g) -> bool {
    return g > secret
})?
dec int idx = find_first_correct(guesses, secret)

print_divider()
println("game summary")
print_divider()
println(format("total guesses: {}", len(guesses)))
println(format("lowest:        {}", arr_min(guesses)?))
println(format("highest:       {}", arr_max(guesses)?))
println(format("guesses below: {}", len(below)))
println(format("guesses above: {}", len(above)))

if (!is_null(idx)) {
    println(format("you got it on attempt {}", idx + 1))
}
