get println from std::io

// --- 1. Variables and arithmetic ---
dec int x = 10
dec int y = 3
dec int sum = x + y
dec int diff = x - y
dec int prod = x * y
dec int quot = x / y
dec int neg = -x

println("=== Arithmetic ===")
println(sum)
println(diff)
println(prod)
println(quot)
println(neg)

// --- 2. Booleans and logic ---
dec bool a = true
dec bool b = false
dec bool cmp_gt = x > y
dec bool cmp_lt = x < y
dec bool cmp_eq = x == y
dec bool cmp_ne = x != y
dec bool cmp_le = x <= y
dec bool cmp_ge = x >= y
dec bool and_res = a and b
dec bool or_res = a or b
dec bool not_res = !a

println("")
println("=== Booleans ===")
println(a)
println(b)
println(cmp_gt)
println(cmp_lt)
println(cmp_eq)
println(cmp_ne)
println(cmp_le)
println(cmp_ge)
println(and_res)
println(or_res)
println(not_res)

// --- 3. Floats ---
dec float pi = 3.14159
dec float half = pi / 2.0

println("")
println("=== Floats ===")
println(pi)
println(half)

// --- 4. Strings and characters ---
dec string greeting = "Hello, world!"
dec string lang = "rl"
dec char ch = 'A'

println("")
println("=== Strings ===")
println(greeting)
println(lang)
println(ch)

// --- 5. Escape sequences ---
println("")
println("=== Escapes ===")
println("tab\there")
println("new\nline")
println("back\\slash")
println("quote\"here")
println("single\'quote")

// --- 6. Constants ---
CONST int MAX = 100
CONST string MSG = "constant string"

println("")
println("=== Constants ===")
println(MAX)
println(MSG)

// --- 7. Null ---
dec int nothing = null

println("")
println("=== Null ===")
println(nothing)

// --- 8. If / else-if / else ---
println("")
println("=== Conditionals ===")
if (x > 5) {
    println("x is big")
} else if (x > 2) {
    println("x is medium")
} else {
    println("x is small")
}

// --- 9. While loop ---
println("")
println("=== While Loop ===")
dec int i = 0
while (i < 3) {
    println(i)
    i = i + 1
}

// --- 10. For loop with break and continue ---
println("")
println("=== For Loop ===")
for [int j = 0, j < 5, j += 1] {
    if (j == 2) {
        continue
    }
    if (j == 4) {
        break
    }
    println(j)
}

// --- 11. ForEach ---
println("")
println("=== ForEach ===")
dec arr[int] items = [10, 20, 30]
for x in items {
    println(x)
}

// --- 12. ForRange ---
println("")
println("=== ForRange ===")
for k in 0..5 {
    println(k)
}

// --- 13. Loop ---
println("")
println("=== Loop ===")
dec int counter = 0
loop {
    println(counter)
    counter = counter + 1
    if (counter == 3) {
        break
    }
}

// --- 14. Functions ---
fn add(int a, int b) -> int {
    return a + b
}

fn greet(string name) {
    println(name)
}

println("")
println("=== Functions ===")
dec int res = add(100, 23)
println(res)
greet("from a function")

// --- 15. Cast expressions ---
println("")
println("=== Casts ===")
dec int as_big = 42
dec float from_int = as_big as float
println(from_int)

// --- 16. Tuple literal ---
println("")
println("=== Tuples ===")
dec (int, int, string) t = (1, 2, "three")
println(t)

// --- 17. Tuple destruction ---
println("")
println("=== Tuple Destruction ===")
dec (int, string) pair = (42, "hello")
dec int px, string py = pair
println(px)
println(py)

// --- 18. Array literal ---
println("")
println("=== Arrays ===")
dec arr[int] nums = [10, 20, 30]
println(nums[0])
nums[1] = 99
println(nums[1])

// --- 19. Record / struct ---
println("")
println("=== Records ===")
record Point {
    int x,
    int y,
}
dec Point p = Point { x: 10, y: 20 }
println(p.x)
p.x = 30
println(p.x)

// --- 20. Impl methods ---
println("")
println("=== Impl Methods ===")
impl Point {
    fn sum(Point a) -> int {
        return a.x + a.y
    }
}
println(p.sum())

// --- 21. Enum / tag ---
println("")
println("=== Enums ===")
tag Color {
    Red,
    Green,
    Blue,
}
dec Color c = Color.Red
println(c)

// --- 22. Match ---
println("")
println("=== Match ===")
match (c) {
    Color.Red => { println("red") }
    Color.Green => { println("green") }
    _ => { println("other") }
}

// --- 23. Ok / Err / Error ---
println("")
println("=== Results ===")
dec result[int] r = ok(42)
println(r)
dec result[int] r2 = err(1)
println(r2)

// --- 24. Error propagation ---
fn safe_div(int a, int b) -> result[int] {
    if (b == 0) { return err(0) }
    return ok(a / b)
}
dec result[int] divided = safe_div(10, 2)
println(divided)

// --- 25. Map ---
println("")
println("=== Maps ===")
get map_len from std::collections
dec map[string, int] ages = { "alice": 30, "bob": 25 }
println(ages)
println(map_len(ages))

// --- 26. Set ---
println("")
println("=== Sets ===")
get set_len from std::collections
dec set[int] s = { 1, 2, 3 }
println(s)
println(set_len(s))

println("")
println("done")

// --- 27. Math stdlib ---
println("")
println("=== Math Stdlib ===")
println(std::math::factorial(5))
println(std::math::gcd(12, 8))
println(std::math::lcm(4, 6))
println(std::math::is_prime(17))
println(std::math::fibonacci(10))
println(std::math::max(3, 7))
println(std::math::min(3, 7))
println(std::math::abs(-42))
println(std::math::sqrt(9.0))
println(std::math::sin(0.0))
println(std::math::cos(0.0))
println(std::math::round(3.7))
println(std::math::ceil(3.2))
println(std::math::floor(3.8))
println(std::math::pow(2.0, 10.0))
println(std::math::log2(256.0))

// --- 28. Math consts (all 25) ---
println("")
println("=== Math Consts ===")
println(std::math::consts::PI())
println(std::math::consts::E())
println(std::math::consts::TAU())
println(std::math::consts::PHI())
println(std::math::consts::INF())
println(std::math::consts::NAN())
println(std::math::consts::FRAC_1_PI())
println(std::math::consts::FRAC_1_SQRT_2())
println(std::math::consts::FRAC_2_PI())
println(std::math::consts::FRAC_2_SQRT_PI())
println(std::math::consts::FRAC_PI_2())
println(std::math::consts::FRAC_PI_3())
println(std::math::consts::FRAC_PI_4())
println(std::math::consts::FRAC_PI_6())
println(std::math::consts::FRAC_PI_8())
println(std::math::consts::SQRT_2())
println(std::math::consts::LN_2())
println(std::math::consts::LN_10())
println(std::math::consts::LOG2_E())
println(std::math::consts::LOG2_10())
println(std::math::consts::LOG10_2())
println(std::math::consts::LOG10_E())
println(std::math::consts::EULER_GAMMA())

// --- 29. Bitwise stdlib ---
println("")
println("=== Bitwise Stdlib ===")
println(std::bitwise::bit_and(255, 15))
println(std::bitwise::bit_or(240, 15))
println(std::bitwise::bit_xor(255, 15))
println(std::bitwise::bit_not(0))
println(std::bitwise::bit_shift_left(1, 4))
println(std::bitwise::bit_shift_right(128, 4))
println(std::bitwise::count_bits(170))
println(std::bitwise::leading_zeros(1))
println(std::bitwise::trailing_zeros(8))

// --- 30. String stdlib ---
println("")
println("=== String Stdlib ===")
println(std::str::to_upper("hello"))
println(std::str::to_lower("WORLD"))
println(std::str::trim("  spaces  "))
println(std::str::contains("hello world", "world"))
println(std::str::starts_with("hello", "hel"))
println(std::str::ends_with("hello", "llo"))
println(std::str::replace("foo bar foo", "foo", "baz"))
println(std::str::repeat("ab", 3))
println(std::str::index_of("hello", "ll"))
println(std::str::count("anaana", "ana"))
println(std::str::pad_left("42", 5, '0'))
println(std::str::pad_right("hi", 5, '.'))
println(std::str::slice("hello", 1, 4))
println(std::str::reverse("abcde"))
println(std::str::char_at("hello", 1))
println(std::str::chars("abc"))
println(std::str::bytes("Hi"))
println(std::str::split("a,b,c", ","))
println(std::str::join([10, 20, 30], "-"))

// --- 31. Time / Path ---
println("")
println("=== Time / Path ===")
println(std::time::time_now())
println(std::fs::path_exists("/tmp"))

// --- 32. Debug stdlib ---
println("")
println("=== Debug Stdlib ===")
std::debug::assert(true)
std::debug::assert_eq(1, 1)
std::debug::assert_ne(1, 2)
std::debug::assert_lt(1, 2)
std::debug::assert_le(1, 1)
std::debug::assert_gt(2, 1)
std::debug::assert_ge(1, 1)
std::debug::assert_approx_eq(1.0, 1.0)
println(std::debug::type_of(42))
println(std::debug::type_of("hi"))
println(std::debug::dbg(99))

// --- 33. Path stdlib ---
println("")
println("=== Path Stdlib ===")
println(std::path::path_extension("foo.txt"))
println(std::path::path_filename("/a/b/c.txt"))
println(std::path::path_parent("/a/b/c.txt"))
println(std::path::path_stem("/a/b/c.txt"))
println(std::path::path_join("/a/b", "c.txt"))
println(std::path::path_set_extension("foo.txt", "md"))
println(std::fs::path_is_dir("/tmp"))
println(std::fs::path_is_file("/etc/hostname"))

// --- 34. FS stdlib ---
println("")
println("=== FS Stdlib ===")
println(std::fs::file_size("/etc/hostname"))
println(std::fs::list_dir("/tmp"))

// --- 35. Process stdlib ---
println("")
println("=== Process Stdlib ===")
println(std::process::exec("echo hello from process"))
println(std::process::exec_code("true"))
println(std::process::exec_lines("echo a && echo b"))
println(std::process::cwd())

// --- 36. Time extended ---
println("")
println("=== Time Extended ===")
dec int ts = std::time::time_now()
println(std::time::format_date_str(ts))
println(std::time::format_time_str(ts))
println(std::time::time_parts(ts))

// --- 37. IO extended ---
println("")
println("=== IO Extended ===")
std::fs::mkdir("/tmp/rl_test_io")
std::fs::write_file("/tmp/rl_test_io/test.txt", "hello world")
println(std::fs::read_file("/tmp/rl_test_io/test.txt"))
std::fs::append_file("/tmp/rl_test_io/test.txt", " appended")
println(std::fs::read_file("/tmp/rl_test_io/test.txt"))
println(std::fs::read_lines("/tmp/rl_test_io/test.txt"))
std::io::eprintln("this goes to stderr")

// --- 38. Types ---
println("")
println("=== Types ===")
println(std::types::to_string(42))
println(std::types::to_bin(255))
println(std::types::to_hex(255))
println(std::types::to_oct(255))
println(std::types::to_int(3.7))
println(std::types::to_float(42))
println(std::types::to_bool(1))

// --- 39. Random ---
println("")
println("=== Random ===")
println(std::math::mod(std::random::rand_int(), 100))
println(std::random::rand_float())
println(std::random::rand_bool())
println(std::random::rand_int_range(1, 10))
println(std::random::rand_dice(6))
println(std::random::rand_range(10))
println(std::random::rand_string(5))
println(std::random::rand_char())

// --- 40. Collections extended ---
println("")
println("=== Collections Extended ===")
dec map[string, int] ages2 = { "alice": 30, "bob": 25 }
println(std::collections::map_get(ages2, "alice"))
println(std::collections::map_contains(ages2, "bob"))
println(std::collections::map_contains(ages2, "eve"))
std::collections::map_remove(ages2, "bob")
println(std::collections::map_contains(ages2, "bob"))
println(std::collections::map_to_array(ages2))

// --- 41. Array extended ---
println("")
println("=== Array Extended ===")
dec arr[int] nums2 = [5, 3, 1, 4, 2]
println(std::array::arr_first(nums2))
println(std::array::arr_last(nums2))
println(std::array::arr_contains(nums2, 3))
println(std::array::arr_contains(nums2, 9))
println(std::array::arr_index_of(nums2, 4))
println(std::array::arr_sum(nums2))
println(std::array::arr_max(nums2))
println(std::array::arr_min(nums2))
println(std::array::arr_sort(nums2))
println(std::array::arr_reverse(nums2))
println(std::array::arr_fill(7, 3))
println(std::array::arr_range(0, 5, 1))
println(std::array::arr_range(10, 0, -2))
println(std::array::arr_unique([1, 2, 2, 3, 3, 3]))
println(std::array::arr_concat([1, 2], [3, 4]))
println(std::array::arr_slice(nums2, 1, 4))
println(std::array::arr_push(nums2, 99))

// --- 42. Closures and lambdas ---
println("")
println("=== Closures ===")

// Basic lambda
dec fn square = fn (int x) -> int {
    return x * x
}
println(square(5))

// Closure capturing outer variable
dec int factor = 3
dec fn triple = fn (int x) -> int {
    return x * factor
}
println(triple(4))

// arr_map with closure
dec arr[int] doubled = std::array::arr_map(nums, fn (int x) -> int {
    return x * 2
})?
println(doubled)

// arr_filter with closure
dec arr[int] large = std::array::arr_filter(nums, fn (int x) -> bool {
    return x > 3
})?
println(large)

// arr_find_index
dec int idx = std::array::arr_find_index(nums, fn (int x) -> bool {
    return x == 3
})?
println(idx)

// arr_all
dec bool all_pos = std::array::arr_all(nums, fn (int x) -> bool {
    return x > 0
})?
println(all_pos)

// arr_any
dec bool any_big = std::array::arr_any(nums, fn (int x) -> bool {
    return x > 5
})?
println(any_big)

// arr_for_each
std::array::arr_for_each([10, 20, 30], fn (int x) {
    println(x)
})?

// arr_sort_by
dec arr[int] sorted = std::array::arr_sort_by(nums, fn (int a, int b) -> int {
    return a - b
})?
println(sorted)

// arr_flat_map
dec arr[int] nums3 = [10, 20]
dec arr[int] flat = std::array::arr_flat_map(nums, fn (int x) -> arr[int] {
    return [x, x * 10]
})?
println(flat)

// --- 43. Missing stdlib: types ---
println("")
println("=== Types Extended ===")
println(std::types::to_byte(256))
println(std::types::to_char(65))
dec result[int] err_val = err(99)
println(std::types::error_unwrap(err_val))

// --- 44. Missing stdlib: random ---
println("")
println("=== Random Extended ===")
println(std::random::rand_dices(3, 6))
println(std::random::rand_bytes(4))
println(std::random::rand_choice([10, 20, 30]))
println(std::random::rand_choices([10, 20, 30], 4))
println(std::random::rand_sample([10, 20, 30, 40, 50], 3))
println(std::random::rand_shuffle([1, 2, 3, 4, 5]))

// --- 45. Missing stdlib: io ---
println("")
println("=== IO Extended 2 ===")
std::fs::write_file("/tmp/rl_test_io/bytes.bin", "binary data")
println(std::fs::read_bytes("/tmp/rl_test_io/bytes.bin"))
std::fs::delete_file("/tmp/rl_test_io/bytes.bin")
