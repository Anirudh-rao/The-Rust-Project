# 2. Primvites

Rust Provides access to wide variety of `pirmitives`. A Sample includes:

## 1. Scalary Types:

- Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
- Floating point: f32, f64
- char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
- bool either true or false
- The unit type (), whose only possible value is an empty tuple: ()

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.


## 2. Compound Types

- Array like `[1,2,3,]`.
- Tuples like `(1,true)`

Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. Note that Rust can also infer types from context.

## 3. Literals and Operators

Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true` and the unit type `()` can be expressed using literals.

Integers can, alternatively, be expressed using hexadecimal, octal or binary notation using these prefixes respectively: `0x`, `0o` or `0b`.

Underscores can be inserted in numeric literals to improve readability, e.g. `1_000 `is the same as `1000`, and `0.000_001` is the same as `0.000001`.


## 4. Tuples

A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature `(T1, T2, ...)`, where T1, T2 are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.



Lets Get started