# Primitives 


## Primitives 
Rust provides access to wide variety of ``primitives``. A Sample Includes

#### Scalar Types

1. Sigined Integers: `i8`, `i16`,`i32`,`i128` and `isize`(Pointer Size)
2. Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
3. Floating Point: `f32`,`f64`
4. char: Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
4. bool : Either `true` or `false`.
4. The Unit Type `()` , whose only possinlt value is an empty tuple:`()`

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

### Compound Types
1. Arrays like ``[1, 2, 3]``
2. Tuples like ``(1, true)``

Variables can always be type annotated. Numbers may additionally be annotated via a suffix or by default. Integers default to i32 and floats to f64. Note that Rust can also infer types from context.


## Literals and Operators

Integers `1`, floats `1.2`, characters `'a'`, strings `"abc"`, booleans `true` and the unit type `()` can be expressed using literals.

Intgers can be alternatively be expressed using hexadecimal , octal or binary notation using these prefixed respectively: `0x`, `0o` or `0b`.

Underscores can be inserted in numeric literals to improve readability, e.g. `1_000` is the same as `1000`, and `0.000_001` is the same as `0.000001`.

Rust also supports scientific `E-notation`, e.g. `1e6`, `7.6e-4`. The associated type is `f64`.

We need to tell the compiler the type of the literals we use. For now, we'll use the `u32` suffix to indicate that the literal is an unsigned `32-bit integer`, and the `i32` suffix to indicate that it's a signed `32-bit integer`.

## Tuples

A tuple is a collection of values of different types. Tuples are constructed using parentheses `()`, and each tuple itself is a value with type signature `(T1, T2, ...)`, where `T1`, `T2` are the types of its members. Functions can use tuples to return multiple values, as tuples can hold any number of values.

## Arrays and Slices:

An array is a collection of objects of the same type T, stored in contiguous memory. Arrays are created using brackets `[]`, and their length, which is known at compile time, is part of their type signature `[T; length]`.

Slices are similar to arrays, but their length is not known at compile time. Instead, a slice is a two-word object; the first word is a pointer to the data, the second word is the length of the slice. The word size is the same as usize, determined by the processor architecture, e.g. 64 bits on an x86-64. Slices can be used to borrow a section of an array and have the type signature `&[T]`.



