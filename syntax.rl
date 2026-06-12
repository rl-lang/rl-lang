// ============================================================
// syntax.rl — rl-lang syntax showcase
// ============================================================
// A living reference for every language feature.
// Run with: rl run syntax.rl


// ------------------------------------------------------------
// 1. imports
// ------------------------------------------------------------
get println, len from std::display
get sin, cos, tan, pow, mod, is_prime, factorial, fibonacci from std::math
get PI, TUA, PHI, E from std::math::consts


// ------------------------------------------------------------
// 2. variable declarations (mutable)
// ------------------------------------------------------------
dec bool   my_bool   = true
dec int    my_int    = 1
dec string my_string = "string"
dec float  my_float  = 1.0
dec char   my_char   = 'x'


// ------------------------------------------------------------
// 3. constant declarations (immutable)
// ------------------------------------------------------------
CONST int    MAX_SIZE  = 100
CONST float  EULER     = 2.71828
CONST bool   DEBUG     = false
CONST string LANG_NAME = "rl"
CONST char   NEWLINE   = '\n'


// ------------------------------------------------------------
// 4. array declarations
// ------------------------------------------------------------
dec arr[int]    my_int_array    = [10, 20, 30]
dec arr[bool]   my_bool_array   = [true, false, true]
dec arr[string] my_string_array = ["my", "world", "hello"]
dec arr[float]  my_float_array  = [1.0, 2.0, 3.0]
dec arr[char]   my_char_array   = ['.', 'r', 'l']


// ------------------------------------------------------------
// 5. constant arrays
// ------------------------------------------------------------
CONST arr[int]    PRIMES  = [2, 3, 5, 7, 11]
CONST arr[string] DAYS    = ["sat", "sun", "mon", "tue", "wed", "thu", "fri"]
CONST arr[float]  WEIGHTS = [0.1, 0.4, 0.5]


// ------------------------------------------------------------
// 6. printing
// ------------------------------------------------------------
println(my_bool, my_int, my_string, my_float, my_char) // true 1 string 1.0 x
println(my_int_array)    // [10, 20, 30]
println(my_bool_array)   // [true, false, true]
println(my_string_array) // [my, world, hello]
println(my_float_array)  // [1.0, 2.0, 3.0]
println(my_char_array)   // [., r, l]
println(my_int_array[1]) // 20
println(PRIMES[0])       // 2
println(DAYS[6])         // fri
println(len(WEIGHTS))    // 3


// ------------------------------------------------------------
// 7. assignment and mutation
// ------------------------------------------------------------
my_bool = !my_bool            // false
my_bool_array[0] = my_bool    // [false, false, true]
println(my_bool_array)

my_int += 3                   // 4
my_int += pow(my_int, my_int) // 260
println(my_int)

my_int_array[0] = mod(my_int_array[1], my_int_array[2]) // 20
println(my_int_array)


// ------------------------------------------------------------
// 8. math stdlib
// ------------------------------------------------------------
dec float my_float_sin = sin(my_float)
dec float my_float_cos = cos(my_float)
dec float my_float_tan = tan(my_float)
println(my_float_sin, my_float_cos, my_float_tan)
// 0.8414709848078965 0.5403023058681398 1.5574077246549023

dec float circle_area = PI() * pow(5.0, 2.0) // πr²
dec float golden      = PHI()                 // ~1.618
dec float full_turn   = TUA()                 // ~6.283
dec float napier      = E()                   // ~2.718
println(circle_area, golden, full_turn, napier)

println(pow(2, 10))   // 1024
println(mod(17, 5))   // 2
println(is_prime(97)) // true
println(factorial(7)) // 5040
println(fibonacci(10)) // 55


// ------------------------------------------------------------
// 9. control flow — if / else if / else
// ------------------------------------------------------------
dec int score = 75
if (score >= 90) {
    println("A")
} else if (score >= 75) {
    println("B")
} else if (score >= 60) {
    println("C")
} else {
    println("F")
}


// ------------------------------------------------------------
// 10. control flow — while loop
// ------------------------------------------------------------
dec int i = 0
dec float x = 1.5
dec arr[float] arr_x = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
while (i < 10) {
    if (mod(x, (x / 2.0)) > 10.0) {
        arr_x[i] = x + pow(x, x)
    } else if (mod(x, (x / 3.0)) == 0.0) {
        arr_x[i] = x + -x * (x + pow(x, 3))
    } else {
        arr_x[i] = 90.09
    }
    x += x + 12.4
    i += 1
}
println(arr_x)
// [-5.8125, 90.09, 90.09, -95295373.51360005, 90.09, 90.09, 90.09, -9744278800898.861, 90.09, 90.09]


// ------------------------------------------------------------
// 11. control flow — for loops
// ------------------------------------------------------------

// C-style: explicit init, condition, increment
dec int sum = 0
for [int j = 1, j < 11, j += 1] {
    sum += j
}
println(sum) // 55

// range-based: i in start..end
dec int product = 1
for k in 1..6 {
    product *= k
}
println(product) // 120

// iterable: i in [list]
dec arr[int] evens = [2, 4, 6, 8, 10]
dec int even_sum = 0
for n in evens {
    even_sum += n
}
println(even_sum) // 30

// break and continue
dec int count = 0
for [int m = 0, m < 100, m += 1] {
    if (is_prime(m)) {
        count += 1
    }
    if (count == 10) {
        break
    }
}
println(count) // 10


// ------------------------------------------------------------
// 12. functions
// ------------------------------------------------------------

fn add (int a, int b) {
    return a + b
}

fn collatz (int n) {
    dec int steps = 0
    while (n != 1) {
        if (mod(n, 2) == 0) {
            n = n / 2
        } else {
            n = n * 3 + 1
        }
        steps += 1
    }
    return steps
}

println(add(3, 4))       // 7
println(collatz(27))     // 111


// ------------------------------------------------------------
// 13. first-class functions and fn arrays
// ------------------------------------------------------------
dec fn double = fn(int x) { return x * 2 }
dec fn square = fn(int x) { return x * x }

dec arr[fn] transforms = [double, square]

dec int val = 5
println(transforms[0](val)) // 10
println(transforms[1](val)) // 25

// passing functions as arguments
fn apply (fn f, int x) {
    return f(x)
}

println(apply(double, 6)) // 12
println(apply(square, 6)) // 36
