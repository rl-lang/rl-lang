<img src="logo.svg" width="200">

# RL - programming language

## statically typed interpreter language built in rust

## syntax

```rl
// declaring variables
dec int a = 10
dec int b = 17
dec int c = 90
// initializing immutable array
dec arr[int] Array = [a, b, c]
// using array variables
dec int x = Array[0] * Array[2]   // x = 10 * 90
println(x)   // 900
```

## build & run

### compile from source

```bash
git clone https://github.com/mohamedgonem/rl-lang
cd rl-lang
cargo run run test.rl
```

## usage

`cargo run run <sourcefile.rl>`

or after compiling

`rl run <sourcefile.rl>`

## status

it is still work in progress

### types

`int`
`float`
`bool`
`string`
`char`
`arr[type]` -> `int` , `float` , `bool` , `string` , `char`  , `null`
`null`

### logical 

`==`  compare
`<=`  less than or equal to
`>=`  greater than or equal to
`<`     less than
`>`     greater than
`!`      bool negation

### arthimatics

`x += 3`  => `x = x + 3`
`x -= 3`   => `x = x - 3`
`x *= 3`   => `x = x * 3`
`x /= 3`   => `x = x / 3`

works on `float` and `int`

### comments

`//`  will ignore what comes after till it hits newline

### std library 

#### display

`print(args)` prints arguments with whitespace between

`println(args)` same as `print` but adds newline

`len(args)` returns the count of items in array

#### math

`pow(args)`  accepts two arguments only and returns the first power the second argument

`mod(args)` accepts two arguments only and returns the first modulo the second argument

`sin(arg)` one argument only returns a `float` type of sin the argument given

`cos(arg)` one argument only returns a `float` type of cos the argument given

`tan(arg)` one argument only returns a `float` type of tan the argument given

#### io

`input()` captures terminal input 

`input(arg)` accepts prompt argument that shows before accepting inputs

### loops

`if` , `else` , and `else if` 

```rl
if (condition) {
  body
} else if (condition) {
  body
} else {
  body
}
```

can use `if` alone or `if` and `else if`

`while` 

```rl
while (condition) {
body
}
```

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) at your option.
