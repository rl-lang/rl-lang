pub struct StdEntry {
    pub name: &'static str,
    pub functions: &'static [(&'static str, &'static str)],
}

pub fn stdlib_entries() -> Vec<StdEntry> {
    vec![
        StdEntry {
            name: "math",
            functions: &[
                ("acos(x)", "arc cosine of x in radians"),
                ("asin(x)", "arc sine of x in radians"),
                ("atan(x)", "arc tangent of x in radians"),
                (
                    "atan2(a, b)",
                    "arc tangent of a/b using signs to determine quadrant",
                ),
                ("cos(x)", "cosine of x in radians"),
                ("degrees(x)", "convert radians to degrees"),
                ("exp(x)", "e raised to the power x"),
                ("factorial(x)", "product of all integers from 1 to x"),
                ("fibonacci(x)", "xth fibonacci number"),
                ("gcd(a, b)", "greatest common divisor of a and b"),
                (
                    "hypot(a, b)",
                    "length of the hypotenuse given two sides: √(a² + b²)",
                ),
                ("is_prime(x)", "true if x is a prime number"),
                ("lcm(a, b)", "least common multiple of a and b"),
                (
                    "lerp(x, y, t)",
                    "linear interpolation between x and y by factor t",
                ),
                ("log(x, base)", "logarithm of x in the given base"),
                ("log2(x)", "base-2 logarithm of x"),
                ("log10(x)", "base-10 logarithm of x"),
                (
                    "map_range(x, in_min, in_max, out_min, out_max)",
                    "re-map x from one range to another",
                ),
                ("mod(a, b)", "remainder of a divided by b"),
                ("radians(x)", "convert degrees to radians"),
                ("sign(x)", "returns -1, 0, or 1 based on the sign of x"),
                ("sin(x)", "sine of x in radians"),
                ("tan(x)", "tangent of x in radians"),
            ],
        },
        StdEntry {
            name: "math::consts",
            functions: &[
                ("E()", "euler's number (~2.718)"),
                ("EUER_GAMMA()", "euler-mascheroni constant (~0.577)"),
                ("FRAC_1_PI()", "1/π"),
                ("FRAC_1_SQRT_2()", "1/√2"),
                ("FRAC_2_PI()", "2/π"),
                ("FRAC_2_SQRT_PI()", "2/√π"),
                ("FRAC_PI_2()", "π/2"),
                ("FRAC_PI_3()", "π/3"),
                ("FRAC_PI_4()", "π/4"),
                ("FRAC_PI_6()", "π/6"),
                ("FRAC_PI_8()", "π/8"),
                ("INF()", "positive infinity"),
                ("is_inf(x)", "true if x is infinite"),
                ("is_nan(x)", "true if x is not a number"),
                ("LN_10()", "natural log of 10"),
                ("LN_2()", "natural log of 2"),
                ("LOG10_2()", "base-10 log of 2"),
                ("LOG10_E()", "base-10 log of e"),
                ("LOG2_10()", "base-2 log of 10"),
                ("LOG2_E()", "base-2 log of e"),
                ("NAN()", "not-a-number value"),
                ("PHI()", "golden ratio (~1.618)"),
                ("PI()", "π (~3.14159)"),
                ("SQRT_2()", "square root of 2 (~1.414)"),
                ("TAU()", "τ = 2π (~6.283)"),
            ],
        },
        StdEntry {
            name: "display",
            functions: &[
                ("print(x)", "print without newline"),
                ("println(x)", "print with newline"),
                ("len(x)", "length of string or array"),
            ],
        },
        StdEntry {
            name: "io",
            functions: &[
                ("input()", "read a line from stdin"),
                ("input(prompt)", "prints prompt and read a line from stdin"),
            ],
        },
    ]
}
