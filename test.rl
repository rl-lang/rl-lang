dec int a = 10
dec int b = 3
dec float x = 5.5
dec float y = 2.25
dec string msg = "RL Language Test Suite"
dec char escape = 'x'
dec bool flag = true

println("=== VARIABLE DECLARATIONS & ASSIGNMENTS ===")
println(a, b, x, y)
println(msg)
println(escape, flag)

println("=== ARITHMETIC OPERATIONS ===")
dec int sum = a + b
println("Sum (10 + 3):", sum)

dec int diff = a - b
println("Diff (10 - 3):", diff)

dec int product = a * b
println("Product (10 * 3):", product)

dec int quotient = a / b
println("Quotient (10 / 3):", quotient)

dec float fsum = x + y
println("Float sum (5.5 + 2.25):", fsum)

dec float fproduct = x * y
println("Float product (5.5 * 2.25):", fproduct)

println("=== MODULO OPERATION ===")
dec int mod_result = mod(a, b)
println("Modulo (10 mod 3):", mod_result)

dec int mod_large = mod(27, 5)
println("Modulo (27 mod 5):", mod_large)

println("=== POWER FUNCTION ===")
dec float pow_int = pow(2.0, 8.0)
println("Power (2^8):", pow_int)

dec float pow_frac = pow(4.0, 2.5)
println("Power (4^2.5):", pow_frac)

println("=== COMPARISON OPERATORS ===")
dec bool eq_test = a == 10
println("10 == 10:", eq_test)

dec bool neq_test = a != b
println("10 != 3:", neq_test)

dec bool lt_test = b < a
println("3 < 10:", lt_test)

dec bool lte_test = b <= 3
println("3 <= 3:", lte_test)

dec bool gt_test = a > b
println("10 > 3:", gt_test)

dec bool gte_test = a >= 10
println("10 >= 10:", gte_test)

println("=== LOGICAL OPERATIONS ===")
dec bool both_true = true
both_true = !false
println("!false:", both_true)

dec bool negated = !true
println("!true:", negated)

println("=== TRIGONOMETRIC FUNCTIONS ===")
dec float sin_result = sin(0.0)
println("sin(0):", sin_result)

dec float cos_result = cos(0.0)
println("cos(0):", cos_result)

dec float tan_result = tan(0.0)
println("tan(0):", tan_result)

println("=== UNARY OPERATIONS ===")
dec int neg_num = -a
println("Negated 10:", neg_num)

dec float neg_float = -x
println("Negated 5.5:", neg_float)

println("=== REASSIGNMENTS ===")
a = 20
println("a after reassignment:", a)

x = 10.75
println("x after reassignment:", x)

msg = "Updated message"
println(msg)

escape = 'z'
println("escape after reassignment:", escape)

flag = false
println("flag after reassignment:", flag)

println("=== COMPLEX EXPRESSIONS ===")
dec int complex1 = (a + b) * 2
println("(20 + 3) * 2 =", complex1)

dec float complex2 = (x + y) / 2.0
println("(10.75 + 2.25) / 2 =", complex2)

dec int complex3 = a - b + 5
println("20 - 3 + 5 =", complex3)

dec float complex4 = x * y + 1.5
println("10.75 * 2.25 + 1.5 =", complex4)

println("=== COMPOUND COMPARISONS ===")
dec bool comp1 = a > b
dec bool comp2 = x < 20.0
println("a > b:", comp1)
println("x < 20.0:", comp2)

println("=== STRING & CHAR VALUES ===")
dec string greeting = "Hello from RL"
println(greeting)

dec char ch1 = 'A'
dec char ch2 = '7'
dec char ch3 = '@'
println(ch1, ch2, ch3)

println("=== MULTIPLE PRINT ARGUMENTS ===")
println("Values:", a, b, x, y, flag)
println("All together:", sum, diff, product, quotient)

println("=== MATHEMATICAL OPERATIONS ON LITERALS ===")
println("2 + 3:", 2 + 3)
println("10 - 4:", 10 - 4)
println("6 * 7:", 6 * 7)
println("20 / 4:", 20 / 4)
println("5.5 + 4.5:", 5.5 + 4.5)

println("=== MORE POWER TESTS ===")
dec float pow_result1 = pow(3.0, 3.0)
println("3^3 =", pow_result1)

dec float pow_result2 = pow(10.0, 2.0)
println("10^2 =", pow_result2)

println("=== MORE MODULO TESTS ===")
dec int mod_test1 = mod(15, 4)
println("15 mod 4 =", mod_test1)

dec int mod_test2 = mod(100, 7)
println("100 mod 7 =", mod_test2)

println("=== EDGE CASES & BOUNDARIES ===")
dec int zero_ops = 5 + 0
println("5 + 0 =", zero_ops)

dec int neg_calc = -5 + 10
println("-5 + 10 =", neg_calc)

dec float div_result = 1.0 / 2.0
println("1.0 / 2.0 =", div_result)

println("=== FINAL SUMMARY ===")
println("a =", a)
println("b =", b)
println("x =", x)
println("y =", y)
println("msg =", msg)
println("escape =", escape)
println("flag =", flag)

println("=== ALL TESTS COMPLETE ===")
