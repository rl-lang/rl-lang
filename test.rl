// declaring arrays
dec arr[int] my_int_array = [10, 20, 30]
dec arr[bool] my_bool_array = [true, false, true]
dec arr[string] my_string_array = ["my", "world", "hello"]
dec arr[float] my_float_array = [1.0, 2.0, 3.0]
dec arr[char] my_char_array = ['.', 'r', 'l']

// declaring variables
dec bool my_bool = true
dec int my_int = 1
dec string my_string = "string"
dec float my_float = 1.0
dec char my_char = 'x'

// printing values (with newline)
println(my_float_array) // [1.0, 2.0, 3.0]
println(my_char_array) // ['.', 'r', 'l']
println(my_string_array) // ["my", "world", "hello"]
println(my_bool_array) // [true, false, true]
println(my_int_array) // [10, 20, 30]
println(my_int_array[1]) // 20
println(my_bool, my_int, my_string, my_float, my_char) // true 1 "string" 1.0 x

// assigning
my_bool = !my_bool
my_bool_array[0] = my_bool
println(my_bool_array) // [false, false, true]

my_int += 3 // 4
my_int += pow(my_int,my_int) // 260
my_int_array[0] = mod(my_int_array[1], my_int_array[2]) // 20

dec float my_float_sin = sin(my_float)
dec float my_int_cos = cos(my_int)
dec float my_float_tan = tan(my_float)
println(my_float_sin, my_int_cos, my_float_tan) // 0.8414709848078965 -0.7301941571456378 1.5574077246549023

// other std
println(len(my_int_array)) // 3

// loops

dec int i = 0
dec float x = 1.5
dec arr[float] arr_x = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
while (i < 10) {
  
  if ( mod(x, (x / 2.0)) > 10.0 ) {
    arr_x[i] = x + pow(x, x)
  } else if ( mod(x, (x / 3.0)) == 0.0) {
    arr_x[i] = x + -x * (x + pow(x , 3))
  } else {
    arr_x[i] = 90.09
  }
  
  x += x + 12.4
  i += 1
}
println(arr_x) // [-5.8125, 90.09, 90.09, -95295373.51360005, 90.09, 90.09, 90.09, -9744278800898.861, 90.09, 90.09]


// testing
dec arr[int] inner = [1, 2, 3]
dec arr[arr[int]] nested = [inner, inner]
println(nested)
println(nested[1][2])
nested[0][2] = 45
println(nested[0][2])
