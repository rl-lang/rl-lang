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
