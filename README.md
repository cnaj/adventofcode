# Advent of Code 2017 [![Build Status](https://travis-ci.org/cnaj/adventofcode.svg?branch=master)](https://travis-ci.org/cnaj/adventofcode)

This is the code for my attempt at http://adventofcode.com/2017 with Rust.

## Program characteristics

Some key points for me to look up how I solved things, and where I used certain Rust features.

### Day 1

- line-by-line output, with optional input as argument
- byte digit conversion
- cycling iterator

### Day 2

- `Deref<Target = str>`
- `IntoIterator<Item = Result<T, E>>`
- compute result while iterating over input lines

### Day 3

- `lib.rs`
- Documentation tests :bulb:
- line-by-line output, with optional input as argument

### Day 4

- `HashMap`
- compute result while iterating over input lines

### Day 5

- Read instructions from `stdin` into list and process afterwards
- mutable access to slice

### Day 6

- line-by-line output, with optional input as argument
- skip/cycle iterators

### Day 7

- `struct` to keep state while reading lines from `stdin` :bulb:
- `regex::Regex` :bulb:
- `enum` as return value
- `Optional.and_then()`
- the insight that string interning is not that easy
  - had a look at [string-cache](https://github.com/servo/string-cache)

### Day 9

- simple pattern matching state machine
