# This is notes
will have random stuff that should be tracked

## To do
- [ ] finish `debug` std module
  - [x] finish implementation
    - [x] `dbg` 
    - [x] `typeof`
    - [x] `assert`
    - [x] `assert_eq`
    - [x] `assert_ne`
    - [x] `panic`
    - [x] `unreachable`
    - [x] `todo`
  - [x] finish documentation
  - [ ] finish tests
- [ ] finish `net` std module
  - [ ] finish implementation
  - [ ] finish documentation
  - [ ] finish tests
- [ ] finish `http` std module
  - [ ] finish implementation
  - [ ] finish documentation
  - [ ] finish tests
- [ ] refactor the following std modules
   - [ ] `str`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `result`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `random`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `process`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `path`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `math`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `io`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `fs`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `bitwise`
      - [ ] logic
      - [ ] tests
      - [ ] docs
   - [ ] `array`
      - [ ] logic
      - [ ] tests
      - [ ] docs
- [ ] finish the rest of documentation
  - [ ] finish concepts
    - [x] add new concepts
    - [ ] update the old concepts
  - [ ] finish tutorial
    - [ ] design the most simple and optimized version of tutorial programs
    - [ ] redesign the steps of tutorial
  - [ ] update stdlib

## New Types
- [ ] `record` a struct like
  - [ ] implement simple `record` with no internal functions
  - [ ] allow `record` to store function thus transforming it into class like
- [ ] `tag` an enum
  - [ ] implement simple `tag` that hold no value just tags
  - [ ] allow storing data in `tag` items
- [ ] `map` an array with key and value
- [ ] `set` an array with unique keys
- [ ] `num` an unsigned integer with 64 bit
- [ ] `path` path type
- [ ] `value` generic value type that should hold any other type

## VM
- [ ] think how it should look
- [ ] design basic vm compiler and runner
- [ ] redesign stdlib to work with vm and interpreter
- [ ] import the rest of interpreter logic

## New Mechanics
- [ ] macros
- [ ] `as` keyword for imported via `get` using custom names
