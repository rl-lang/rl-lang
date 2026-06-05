// ============================================================
// syntax.rl — rl language syntax showcase
// ============================================================


// ------------------------------------------------------------
// 1. variable declarations (mutable)
// ------------------------------------------------------------

dec bool   my_bool   = true
dec int    my_int    = 1
dec string my_string = "string"
dec float  my_float  = 1.0
dec char   my_char   = 'x'


// ------------------------------------------------------------
// 2. constant declarations (immutable)
// ------------------------------------------------------------

CONST int    MAX_SIZE  = 100
CONST float  PI        = 3.14159
CONST bool   DEBUG     = false
CONST string LANG_NAME = "rl"
CONST char   NEWLINE   = '\n'


// ------------------------------------------------------------
// 3. array declarations
// ------------------------------------------------------------

dec arr[int]    my_int_array    = [10, 20, 30]
dec arr[bool]   my_bool_array   = [true, false, true]
dec arr[string] my_string_array = ["my", "world", "hello"]
dec arr[float]  my_float_array  = [1.0, 2.0, 3.0]
dec arr[char]   my_char_array   = ['.', 'r', 'l']


// ------------------------------------------------------------
// 4. nested arrays
// ------------------------------------------------------------

dec arr[int]      inner  = [1, 2, 3]
dec arr[arr[int]] nested = [inner, inner]


// ------------------------------------------------------------
// 5. printing
// ------------------------------------------------------------

display::println(my_bool, my_int, my_string, my_float, my_char)  // true 1 string 1.0 x

display::println(my_int_array)    // [10, 20, 30]
display::println(my_bool_array)   // [true, false, true]
display::println(my_string_array) // [my, world, hello]
display::println(my_float_array)  // [1.0, 2.0, 3.0]
display::println(my_char_array)   // [., r, l]

display::println(my_int_array[1]) // 20

display::println(nested)          // [[1, 2, 3], [1, 2, 3]]
display::println(nested[1][2])    // 3


// ------------------------------------------------------------
// 6. assignment and mutation
// ------------------------------------------------------------

my_bool = !my_bool           // false
my_bool_array[0] = my_bool   // [false, false, true]
display::println(my_bool_array)

my_int += 3                  // 4
my_int += math::pow(my_int, my_int) // 260
display::println(my_int)

my_int_array[0] = math::mod(my_int_array[1], my_int_array[2]) // 20
display::println(my_int_array)

nested[0][2] = 45
display::println(nested[0][2]) // 45


// ------------------------------------------------------------
// 7. math stdlib
// ------------------------------------------------------------

dec float my_float_sin = math::sin(my_float)
dec float my_float_cos = math::cos(my_float)
dec float my_float_tan = math::tan(my_float)

display::println(my_float_sin, my_float_cos, my_float_tan)
// 0.8414709848078965 0.5403023058681398 1.5574077246549023

display::println(math::pow(2, 10))    // 1024
display::println(math::mod(17, 5))    // 2
display::println(display::len(my_int_array)) // 3


// ------------------------------------------------------------
// 8. control flow — if / else if / else
// ------------------------------------------------------------

dec int score = 75

if (score >= 90) {
  display::println("A")
} else if (score >= 75) {
  display::println("B")
} else if (score >= 60) {
  display::println("C")
} else {
  display::println("F")
}


// ------------------------------------------------------------
// 9. control flow — while loop
// ------------------------------------------------------------

dec int   i     = 0
dec float x     = 1.5
dec arr[float] arr_x = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]

while (i < 10) {
  if (math::mod(x, (x / 2.0)) > 10.0) {
    arr_x[i] = x + math::pow(x, x)
  } else if (math::mod(x, (x / 3.0)) == 0.0) {
    arr_x[i] = x + -x * (x + math::pow(x, 3))
  } else {
    arr_x[i] = 90.09
  }

  x += x + 12.4
  i += 1
}

display::println(arr_x)
// [-5.8125, 90.09, 90.09, -95295373.51360005, 90.09, 90.09, 90.09, -9744278800898.861, 90.09, 90.09]


// ------------------------------------------------------------
// 10. constant arrays
// ------------------------------------------------------------

CONST arr[int]    PRIMES  = [2, 3, 5, 7, 11]
CONST arr[string] DAYS    = ["sat", "sun", "mon", "tue", "wed", "thu", "fri"]
CONST arr[float]  WEIGHTS = [0.1, 0.4, 0.5]

display::println(PRIMES[0])  // 2
display::println(DAYS[6])    // fri
display::println(display::len(WEIGHTS)) // 3
