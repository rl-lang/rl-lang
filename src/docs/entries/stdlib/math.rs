use crate::docs::entry::{FnEntry, StdEntry};
pub static MATH: StdEntry = StdEntry {
    name: "math",
    description: "functions for math",
    functions: FUNCTIONS,
};

static FUNCTIONS: &'static [&'static FnEntry] = &[
    &ABS, &ACOS, &ASIN, &ATAN, &ATAN2, &CEIL, &CLAMP, &COS, &DEGREES, &EXP, &FACTORIAL, &FIBONACCI,
    &FLOOR, &GCD, &HYPOT, &IS_PRIME, &LCM, &LERP, &LOG, &LOG2, &LOG10, &MAP_RANGE, &MAX, &MIN,
    &MOD, &POW, &RADIANS, &ROUND, &SIGN, &SIN, &SQRT, &TAN,
];

static ABS: FnEntry = FnEntry {
    signature: "abs(number)",
    description: "returns the absolute value of number",
    example: "get std::math::abs\n\ndec int x = -1\nx.abs() // 1",
};

static ACOS: FnEntry = FnEntry {
    signature: "acos(x)",
    description: "arc cosine of x in radians",
    example: "get std::math::acos\n\nacos(1.0) // 0.0",
};

static ASIN: FnEntry = FnEntry {
    signature: "asin(x)",
    description: "arc sine of x in radians",
    example: "get std::math::asin\n\nasin(1.0) // 1.5707...",
};

static ATAN: FnEntry = FnEntry {
    signature: "atan(x)",
    description: "arc tangent of x in radians",
    example: "get std::math::atan\n\natan(1.0) // 0.7853...",
};

static ATAN2: FnEntry = FnEntry {
    signature: "atan2(a, b)",
    description: "arc tangent of a/b using signs to determine quadrant",
    example: "get std::math::atan2\n\natan2(1.0, 1.0) // 0.7853...",
};

static CEIL: FnEntry = FnEntry {
    signature: "ceil(x)",
    description: "smallest integer greater than or equal to x",
    example: "get std::math::ceil\n\nceil(2.12) // 3.0",
};

static CLAMP: FnEntry = FnEntry {
    signature: "clamp(x, min, max)",
    description: "clamps x between min and max, returning min if x < min, max if x > max",
    example: "get std::math::clamp\n\nclamp(12, 15, 20) // 15",
};

static COS: FnEntry = FnEntry {
    signature: "cos(x)",
    description: "cosine of x in radians",
    example: "get std::math::cos\n\ncos(0.0) // 1.0",
};

static DEGREES: FnEntry = FnEntry {
    signature: "degrees(x)",
    description: "convert radians to degrees",
    example: "get std::math::degrees\n\ndegrees(3.14159) // 180.0",
};

static EXP: FnEntry = FnEntry {
    signature: "exp(x)",
    description: "e raised to the power x",
    example: "get std::math::exp\n\nexp(1.0) // 2.718...",
};

static FACTORIAL: FnEntry = FnEntry {
    signature: "factorial(x)",
    description: "product of all integers from 1 to x",
    example: "get std::math::factorial\n\nfactorial(5) // 120",
};

static FIBONACCI: FnEntry = FnEntry {
    signature: "fibonacci(x)",
    description: "xth fibonacci number",
    example: "get std::math::fibonacci\n\nfibonacci(7) // 13",
};

static FLOOR: FnEntry = FnEntry {
    signature: "floor(x)",
    description: "largest integer less than or equal to x",
    example: "get std::math::floor\n\nfloor(1.23) // 1.0",
};

static GCD: FnEntry = FnEntry {
    signature: "gcd(a, b)",
    description: "greatest common divisor of a and b",
    example: "get std::math::gcd\n\ngcd(12, 8) // 4",
};

static HYPOT: FnEntry = FnEntry {
    signature: "hypot(a, b)",
    description: "length of the hypotenuse given two sides: √(a² + b²)",
    example: "get std::math::hypot\n\nhypot(3.0, 4.0) // 5.0",
};

static IS_PRIME: FnEntry = FnEntry {
    signature: "is_prime(x)",
    description: "true if x is a prime number",
    example: "get std::math::is_prime\n\nis_prime(7) // true",
};

static LCM: FnEntry = FnEntry {
    signature: "lcm(a, b)",
    description: "least common multiple of a and b",
    example: "get std::math::lcm\n\nlcm(4, 6) // 12",
};

static LERP: FnEntry = FnEntry {
    signature: "lerp(x, y, t)",
    description: "linear interpolation between x and y by factor t",
    example: "get std::math::lerp\n\nlerp(0.0, 10.0, 0.5) // 5.0",
};

static LOG: FnEntry = FnEntry {
    signature: "log(x, base)",
    description: "logarithm of x in the given base",
    example: "get std::math::log\n\nlog(100.0, 10.0) // 2.0",
};

static LOG2: FnEntry = FnEntry {
    signature: "log2(x)",
    description: "base-2 logarithm of x",
    example: "get std::math::log2\n\nlog2(8.0) // 3.0",
};

static LOG10: FnEntry = FnEntry {
    signature: "log10(x)",
    description: "base-10 logarithm of x",
    example: "get std::math::log10\n\nlog10(1000.0) // 3.0",
};

static MAP_RANGE: FnEntry = FnEntry {
    signature: "map_range(x, in_min, in_max, out_min, out_max)",
    description: "re-map x from one range to another",
    example: "get std::math::map_range\n\nmap_range(5.0, 0.0, 10.0, 0.0, 100.0) // 50.0",
};

static MAX: FnEntry = FnEntry {
    signature: "max(a, b)",
    description: "returns the larger of a and b",
    example: "get std::math::max\n\nmax(4, 6) // 6",
};

static MIN: FnEntry = FnEntry {
    signature: "min(a, b)",
    description: "returns the smaller of a and b",
    example: "get std::math::min\n\nmin(4, 6) // 4",
};

static MOD: FnEntry = FnEntry {
    signature: "mod(a, b)",
    description: "remainder of a divided by b",
    example: "get std::math::mod\n\nmod(10, 3) // 1",
};

static POW: FnEntry = FnEntry {
    signature: "pow(a, b)",
    description: "raises a to the power of b",
    example: "get std::math::pow\n\npow(2, 2) // 4.0",
};

static RADIANS: FnEntry = FnEntry {
    signature: "radians(x)",
    description: "convert degrees to radians",
    example: "get std::math::radians\n\nradians(180.0) // 3.14159...",
};

static ROUND: FnEntry = FnEntry {
    signature: "round(x)",
    description: "rounds x to the nearest integer",
    example: "get std::math::round\n\nround(2.2) // 2.0",
};

static SIGN: FnEntry = FnEntry {
    signature: "sign(x)",
    description: "returns -1, 0, or 1 based on the sign of x",
    example: "get std::math::sign\n\nsign(-5) // -1",
};

static SIN: FnEntry = FnEntry {
    signature: "sin(x)",
    description: "sine of x in radians",
    example: "get std::math::sin\n\nsin(0.0) // 0.0",
};

static SQRT: FnEntry = FnEntry {
    signature: "sqrt(x)",
    description: "square root of x",
    example: "get std::math::sqrt\n\nsqrt(4) // 2.0",
};

static TAN: FnEntry = FnEntry {
    signature: "tan(x)",
    description: "tangent of x in radians",
    example: "get std::math::tan\n\ntan(0.0) // 0.0",
};
